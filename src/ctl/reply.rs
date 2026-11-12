use anyhow::anyhow;
use std::path::PathBuf;

use crate::{
    AppResult,
    app::App,
    args::Format,
    ctl::{
        message::Message,
        types::{ExportSpec, ImportSpec, Rows},
    },
    duckdb::{DataSource, FwfOptions, Import},
    handler,
    misc::sql::sql,
    tui::popups::pickers::inline_query_picker::QueryType,
    writers::ExportFormat,
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
        Message::Sql { pid, from, query } if is_for_me(pid) => app?.ctl_query(from, query),
        Message::Import { pid, from, spec } if is_for_me(pid) => match import_spec(spec) {
            Ok((source, import, max_rows)) => app?.ctl_import(from, source, import, max_rows),
            Err(err) => Some(Message::import_reply(
                from,
                Err(format!("Import failed: {err:#}")),
            )),
        },
        Message::Tabs { pid } if is_for_me(pid) => app.map(|app| Message::tabs_reply(app.tabs())),
        Message::TabSwitch { pid, tab_idx } if is_for_me(pid) => app.map(|app| {
            if tab_idx < app.tabs().len() {
                handler::message::Message::TabsSelect(tab_idx).enqueue();
                Message::tab_switch_reply(Some(tab_idx))
            } else {
                Message::tab_switch_reply(None)
            }
        }),

        Message::Filter { pid, from, filter } if is_for_me(pid) => {
            app?.ctl_inline(from, QueryType::Filter, filter)
        }
        Message::Select { pid, from, columns } if is_for_me(pid) => {
            app?.ctl_inline(from, QueryType::Select, columns)
        }
        Message::Order {
            pid,
            from,
            order_by,
        } if is_for_me(pid) => app?.ctl_inline(from, QueryType::Order, order_by),
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
        Message::Export { pid, from, spec } if is_for_me(pid) => {
            let path = spec.path().to_path_buf();
            app?.ctl_export(from, export_format(spec), path)
        }

        _ => None,
    }
}

fn export_format(spec: ExportSpec) -> ExportFormat {
    match spec {
        ExportSpec::Csv {
            separator,
            quote_char,
            no_header,
            ..
        } => ExportFormat::Csv {
            separator,
            quote: quote_char,
            header: !no_header,
        },
        ExportSpec::Tsv {
            quote_char,
            no_header,
            ..
        } => ExportFormat::Csv {
            separator: '\t',
            quote: quote_char,
            header: !no_header,
        },
        ExportSpec::Json { .. } => ExportFormat::Json,
        ExportSpec::Jsonl { .. } => ExportFormat::Jsonl,
        ExportSpec::Parquet { .. } => ExportFormat::Parquet,
        ExportSpec::Arrow { .. } => ExportFormat::Arrow,
        ExportSpec::Avro { .. } => ExportFormat::Avro,
        ExportSpec::Markdown { .. } => ExportFormat::Markdown,
    }
}

fn read_rows(app: &App, offset: Option<usize>, count: Option<usize>) -> AppResult<Rows> {
    let table = app
        .tabs()
        .selected()
        .ok_or_else(|| anyhow!("no tab is open"))?
        .table();
    let offset = offset.unwrap_or(table.offset());
    let count = count.unwrap_or(table.rendered_rows());
    let (Some(duckdb), Some(view)) = (app.duckdb(), table.view()) else {
        return Err(anyhow!("not available until the table is ingested"));
    };
    Ok(Rows {
        offset,
        total: table.row_count(),
        records: duckdb.rows_json(&view, offset, count)?,
    })
}

fn is_for_me(pid: u32) -> bool {
    std::process::id() == pid
}

fn import_spec(spec: ImportSpec) -> AppResult<(DataSource, Import, Option<usize>)> {
    let format = |format| Import::Format(format);
    let (source, import, max_rows) = match spec {
        ImportSpec::Csv {
            source,
            separator,
            quote_char,
            no_header,
            ignore_errors,
            truncate_ragged_lines,
            max_rows,
        } => (
            source,
            Import::Csv {
                delimiter: separator,
                quote: quote_char,
                header: !no_header,
                ignore_errors,
                null_padding: truncate_ragged_lines,
            },
            max_rows,
        ),
        ImportSpec::Parquet { source, max_rows } => (source, format(Format::Parquet), max_rows),
        ImportSpec::Json {
            source,
            ignore_errors,
            max_rows,
        } => (
            source,
            Import::Json {
                lines: false,
                ignore_errors,
            },
            max_rows,
        ),
        ImportSpec::Jsonl {
            source,
            ignore_errors,
            max_rows,
        } => (
            source,
            Import::Json {
                lines: true,
                ignore_errors,
            },
            max_rows,
        ),
        ImportSpec::Arrow { source, max_rows } => (source, format(Format::Arrow), max_rows),
        ImportSpec::Avro { source, max_rows } => (source, format(Format::Avro), max_rows),
        ImportSpec::Fwf {
            source,
            widths,
            separator_length,
            no_flexible_width,
            no_header,
        } => {
            let widths = parse_widths(&widths)?;
            (
                source,
                Import::Fwf(FwfOptions {
                    widths: (!widths.is_empty()).then_some(widths),
                    separator_length,
                    flexible_width: !no_flexible_width,
                    header: !no_header,
                }),
                None,
            )
        }
        ImportSpec::Sqlite { source, sqlite_key } => (source, Import::Sqlite(sqlite_key), None),
        ImportSpec::Excel { source } => (source, format(Format::Excel), None),
        ImportSpec::Logfmt { source } => (source, format(Format::Logfmt), None),
        ImportSpec::Html { source } => (source, format(Format::Html), None),
        ImportSpec::Markdown { source } => (source, format(Format::Markdown), None),
    };
    Ok((DataSource::File(PathBuf::from(source)), import, max_rows))
}

fn parse_widths(widths: &str) -> AppResult<Vec<usize>> {
    Ok(widths
        .split(',')
        .map(str::trim)
        .filter(|w| !w.is_empty())
        .map(str::parse)
        .collect::<Result<_, _>>()?)
}
