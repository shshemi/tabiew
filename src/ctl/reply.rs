use anyhow::anyhow;
use polars::{frame::DataFrame, io::SerWriter, prelude::JsonWriter};
use std::{path::PathBuf, sync::Arc};

use crate::{
    AppResult,
    app::App,
    ctl::{
        message::Message,
        types::{ExportSpec, ImportSpec, Rows, Shape},
    },
    duckdb::DataSource,
    handler,
    misc::sql::{TableSource, sql},
    net::remote_load::Importer,
    readers::{
        ArrowIpcToDataFrame, AvroToDataFrame, CsvToDataFrame, ExcelToDataFrames, FwfToDataFrame,
        HtmlToDataFrame, JsonLineToDataFrame, JsonToDataFrame, LogfmtToDataFrame,
        MarkdownToDataFrame, ParquetToDataFrame, ReaderSource, SqliteToDataFrames,
    },
    tui::{pane::TableDescription, popups::pickers::inline_query_picker::QueryType},
    writers::{
        Destination, JsonFormat, WriteToArrow, WriteToAvro, WriteToCsv, WriteToFile, WriteToJson,
        WriteToMarkdown, WriteToParquet,
    },
};

pub struct Reply<'a> {
    msg: Message,
    app: Option<&'a mut App>,
}

impl<'a> Reply<'a> {
    pub fn new(msg: Message) -> Self {
        Self { msg, app: None }
    }
    pub fn with_app(self, app: &'a mut App) -> Self {
        Self {
            app: Some(app),
            ..self
        }
    }

    pub fn send(self) {
        if let Some(rp) = handle(self.msg, self.app) {
            rp.send();
        }
    }
}

fn handle(msg: Message, app: Option<&mut App>) -> Option<Message> {
    let duckdb = app.as_deref().and_then(App::duckdb).is_some();
    let view = app
        .as_deref()
        .and_then(|app| app.tabs().selected())
        .and_then(|pane| pane.table().view())
        .is_some();
    match &msg {
        Message::Sql { pid, from, .. }
        | Message::Import { pid, from, .. }
        | Message::Filter { pid, from, .. }
        | Message::Select { pid, from, .. }
        | Message::Order { pid, from, .. }
        | Message::Export { pid, from, .. }
            if is_for_me(*pid) =>
        {
            Message::ack(*from).send()
        }
        _ => (),
    }
    match msg {
        Message::Ps => Some(Message::ps_reply()),
        Message::Schema { pid } if is_for_me(pid) => Some(Message::schema_replay(sql().schema())),
        Message::Cancel { pid, from } if is_for_me(pid) => app?.ctl_cancel(from),
        Message::Sql { pid, from, query } if is_for_me(pid) && duckdb => {
            app?.ctl_query(from, query)
        }
        Message::Sql { pid, from, query } if is_for_me(pid) => {
            match sql().execute_with_default(&query, None) {
                Ok(df) => {
                    let shape = Shape::from(&df);
                    handler::message::Message::TabsAddQueryPane(df, query).enqueue();
                    Some(Message::sql_replay(from, Ok(shape)))
                }
                Err(err) => Some(Message::sql_replay(
                    from,
                    Err(format!("Query {query} failed: {err}")),
                )),
            }
        }
        Message::Import { pid, from, spec } if is_for_me(pid) => {
            match import_to_backend(spec, duckdb) {
                Ok(Imported::Names(names)) => Some(Message::import_reply(from, Ok(names))),
                Ok(Imported::Queued(source, reader)) => app?.ctl_import(from, source, reader),
                Err(err) => Some(Message::import_reply(
                    from,
                    Err(format!("Import failed: {err:#}")),
                )),
            }
        }
        Message::Tabs { pid } if is_for_me(pid) => app.map(|app| Message::tabs_reply(app.tabs())),
        Message::TabSwitch { pid, tab_idx } if is_for_me(pid) => app.map(|app| {
            if tab_idx < app.tabs().len() {
                handler::message::Message::TabsSelect(tab_idx).enqueue();
                Message::tab_switch_reply(Some(tab_idx))
            } else {
                Message::tab_switch_reply(None)
            }
        }),

        Message::Filter { pid, from, filter } if is_for_me(pid) && duckdb && view => {
            app?.ctl_inline(from, QueryType::Filter, filter)
        }
        Message::Select { pid, from, columns } if is_for_me(pid) && duckdb && view => {
            app?.ctl_inline(from, QueryType::Select, columns)
        }
        Message::Order {
            pid,
            from,
            order_by,
        } if is_for_me(pid) && duckdb && view => app?.ctl_inline(from, QueryType::Order, order_by),
        Message::Filter { pid, from, filter } if is_for_me(pid) => app
            .as_deref()
            .map(|app| Message::filter_reply(from, apply_filter(app, filter))),
        Message::Select { pid, from, columns } if is_for_me(pid) => app
            .as_deref()
            .map(|app| Message::select_reply(from, apply_select(app, columns))),
        Message::Order {
            pid,
            from,
            order_by,
        } if is_for_me(pid) => app
            .as_deref()
            .map(|app| Message::order_reply(from, apply_order(app, order_by))),
        Message::RmTab { pid, tab_idx } if is_for_me(pid) => app.map(|app| {
            if tab_idx < app.tabs().len() {
                handler::message::Message::TabsRemove(tab_idx).enqueue();
                Message::rm_tab_reply(Some(tab_idx))
            } else {
                Message::rm_tab_reply(None)
            }
        }),
        Message::RowSelect { pid, row_idx } if is_for_me(pid) => app.map(|app| {
            let height = app
                .tabs()
                .selected()
                .map(|pane| pane.table().row_count())
                .unwrap_or_default();
            if row_idx < height {
                handler::message::Message::PaneTableSelect(row_idx).enqueue();
                Message::row_select_reply(Some(row_idx))
            } else {
                Message::row_select_reply(None)
            }
        }),
        Message::Rows { pid, offset, count } if is_for_me(pid) => app.map(|app| {
            Message::rows_reply(
                read_rows(app, offset, count)
                    .map_err(|err| format!("Reading rows failed: {err:#}")),
            )
        }),
        Message::Export { pid, from, spec } if is_for_me(pid) && duckdb && view => {
            let path = spec.path().to_path_buf();
            app?.ctl_export(from, writer(spec).format(), path)
        }
        Message::Export { pid, from, spec } if is_for_me(pid) => app
            .as_deref()
            .map(|app| Message::export_reply(from, apply_export(app, spec))),

        _ => None,
    }
}

fn apply_filter(app: &App, filter: String) -> Result<Shape, String> {
    match query_current_table(app, &format!("SELECT * FROM _ where {filter}")) {
        Ok(df) => {
            let shape = Shape::from(&df);
            let msg = format!("Filter '{filter}' applied");
            push_view(df, TableDescription::Filter(filter), &msg);
            Ok(shape)
        }
        Err(err) => Err(format!("Filter '{filter}' failed: {err}")),
    }
}

fn apply_select(app: &App, columns: String) -> Result<Shape, String> {
    match query_current_table(app, &format!("SELECT {columns} FROM _")) {
        Ok(df) => {
            let shape = Shape::from(&df);
            let msg = format!("Column selection '{columns}' occurred");
            push_view(df, TableDescription::Select(columns), &msg);
            Ok(shape)
        }
        Err(err) => Err(format!("Column selection '{columns}' failed: {err}")),
    }
}

fn apply_order(app: &App, order_by: String) -> Result<Shape, String> {
    match query_current_table(app, &format!("SELECT * FROM _ ORDER BY {order_by}")) {
        Ok(df) => {
            let shape = Shape::from(&df);
            let msg = format!("Data frame ordered by '{order_by}'");
            push_view(df, TableDescription::Order(order_by), &msg);
            Ok(shape)
        }
        Err(err) => Err(format!("Ordering by '{order_by}' failed: {err}")),
    }
}

fn apply_export(app: &App, spec: ExportSpec) -> Result<PathBuf, String> {
    let path = spec.path().to_path_buf();
    match export_current_table(app, spec) {
        Ok(()) => {
            handler::message::Message::AppShowToast(format!(
                "Data frame exported to '{}'",
                path.display()
            ))
            .enqueue();
            Ok(path)
        }
        Err(err) => Err(format!("Export to '{}' failed: {err:#}", path.display())),
    }
}

fn writer(spec: ExportSpec) -> Box<dyn WriteToFile> {
    match spec {
        ExportSpec::Csv {
            separator,
            quote_char,
            no_header,
            ..
        } => Box::new(
            WriteToCsv::default()
                .with_separator_char(separator)
                .with_quote_char(quote_char)
                .with_header(!no_header),
        ),
        ExportSpec::Tsv {
            quote_char,
            no_header,
            ..
        } => Box::new(
            WriteToCsv::default()
                .with_separator_char('\t')
                .with_quote_char(quote_char)
                .with_header(!no_header),
        ),
        ExportSpec::Json { .. } => Box::new(WriteToJson::default().with_format(JsonFormat::Json)),
        ExportSpec::Jsonl { .. } => {
            Box::new(WriteToJson::default().with_format(JsonFormat::JsonLine))
        }
        ExportSpec::Parquet { .. } => Box::new(WriteToParquet),
        ExportSpec::Arrow { .. } => Box::new(WriteToArrow),
        ExportSpec::Avro { .. } => Box::new(WriteToAvro),
        ExportSpec::Markdown { .. } => Box::new(WriteToMarkdown),
    }
}

fn export_current_table(app: &App, spec: ExportSpec) -> AppResult<()> {
    let path = spec.path().to_path_buf();
    let writer = writer(spec);
    let df = app
        .tabs()
        .selected()
        .ok_or_else(|| anyhow!("no tab is open"))?
        .table()
        .data_frame()
        .ok_or_else(|| anyhow!("not available until the table is ingested"))?;
    writer.write_to_file(Destination::File(path), &mut df.clone())
}

fn read_rows(app: &App, offset: Option<usize>, count: Option<usize>) -> AppResult<Rows> {
    let table = app
        .tabs()
        .selected()
        .ok_or_else(|| anyhow!("no tab is open"))?
        .table();
    let offset = offset.unwrap_or(table.offset());
    let count = count.unwrap_or(table.rendered_rows());
    if let (Some(duckdb), Some(view)) = (app.duckdb(), table.view()) {
        return Ok(Rows {
            offset,
            total: table.row_count(),
            records: duckdb.rows_json(&view, offset, count)?,
        });
    }
    let frame = table
        .data_frame()
        .ok_or_else(|| anyhow!("not available on DuckDB tables yet"))?;
    let mut df = frame.slice(offset as i64, count);
    let mut buf = Vec::new();
    JsonWriter::new(&mut buf)
        .with_json_format(polars::io::json::JsonFormat::Json)
        .finish(&mut df)?;
    Ok(Rows {
        offset,
        total: frame.height(),
        records: String::from_utf8(buf)?,
    })
}

fn current_table(app: &App) -> AppResult<DataFrame> {
    let pane = app
        .tabs()
        .selected()
        .ok_or_else(|| anyhow!("no tab is open"))?;
    pane.table()
        .data_frame()
        .cloned()
        .ok_or_else(|| anyhow!("not available on DuckDB tables yet"))
}

fn query_current_table(app: &App, query: &str) -> AppResult<DataFrame> {
    Ok(sql().execute_with_default(query, current_table(app)?)?)
}

fn push_view(df: DataFrame, description: TableDescription, msg: &str) {
    handler::message::Message::PanePushDataFrame(df, description).enqueue();
    handler::message::Message::AppShowToast(msg.to_owned()).enqueue();
}

fn is_for_me(pid: u32) -> bool {
    std::process::id() == pid
}

enum Imported {
    Names(Vec<String>),
    Queued(DataSource, Arc<dyn Importer>),
}

fn import_to_backend(spec: ImportSpec, duckdb: bool) -> AppResult<Imported> {
    match spec {
        ImportSpec::Csv {
            source,
            separator,
            quote_char,
            no_header,
            ignore_errors,
            truncate_ragged_lines,
            max_rows,
        } => register_file(
            CsvToDataFrame::default()
                .with_separator(separator)
                .with_quote_char(quote_char)
                .with_no_header(no_header)
                .with_ignore_errors(ignore_errors)
                .with_truncate_ragged_lines(truncate_ragged_lines)
                .with_max_rows(max_rows),
            source,
            duckdb,
        ),
        ImportSpec::Parquet { source, max_rows } => register_file(
            ParquetToDataFrame::default().with_max_rows(max_rows),
            source,
            duckdb,
        ),
        ImportSpec::Json {
            source,
            ignore_errors,
            max_rows,
        } => register_file(
            JsonToDataFrame::default()
                .with_ignore_errors(ignore_errors)
                .with_max_rows(max_rows),
            source,
            duckdb,
        ),
        ImportSpec::Jsonl {
            source,
            ignore_errors,
            max_rows,
        } => register_file(
            JsonLineToDataFrame::default()
                .with_ignore_errors(ignore_errors)
                .with_max_rows(max_rows),
            source,
            duckdb,
        ),
        ImportSpec::Arrow { source, max_rows } => register_file(
            ArrowIpcToDataFrame::default().with_max_rows(max_rows),
            source,
            duckdb,
        ),
        ImportSpec::Avro { source, max_rows } => register_file(
            AvroToDataFrame::default().with_max_rows(max_rows),
            source,
            duckdb,
        ),
        ImportSpec::Fwf {
            source,
            widths,
            separator_length,
            no_flexible_width,
            no_header,
        } => register_file(
            FwfToDataFrame::default()
                .with_widths(parse_widths(&widths)?)
                .with_separator_length(separator_length)
                .with_flexible_width(!no_flexible_width)
                .with_has_header(!no_header),
            source,
            duckdb,
        ),
        ImportSpec::Sqlite { source, sqlite_key } => {
            let reader = SqliteToDataFrames::default();
            match sqlite_key {
                Some(key) => register_file(reader.key(key), source, duckdb),
                None => register_file(reader, source, duckdb),
            }
        }
        ImportSpec::Excel { source } => register_file(ExcelToDataFrames, source, duckdb),
        ImportSpec::Logfmt { source } => {
            register_file(LogfmtToDataFrame::default(), source, duckdb)
        }
        ImportSpec::Html { source } => register_file(HtmlToDataFrame, source, duckdb),
        ImportSpec::Markdown { source } => register_file(MarkdownToDataFrame, source, duckdb),
    }
}

fn register_file(
    reader: impl Importer + 'static,
    source: String,
    duckdb: bool,
) -> AppResult<Imported> {
    let path = PathBuf::from(source);
    if duckdb {
        return Ok(Imported::Queued(DataSource::File(path), Arc::new(reader)));
    }
    Ok(Imported::Names(
        reader
            .read_to_data_frames(ReaderSource::File(path.clone()))?
            .into_iter()
            .map(|(name, df)| sql().register(&name, df, TableSource::File(path.clone())))
            .collect(),
    ))
}

fn parse_widths(widths: &str) -> AppResult<Vec<usize>> {
    Ok(widths
        .split(',')
        .map(str::trim)
        .filter(|w| !w.is_empty())
        .map(str::parse)
        .collect::<Result<_, _>>()?)
}
