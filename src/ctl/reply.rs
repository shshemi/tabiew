use std::path::PathBuf;

use crate::{
    AppResult,
    app::App,
    ctl::{message::Message, types::ImportSpec},
    handler,
    misc::sql::{TableSource, sql},
    readers::{
        ArrowIpcToDataFrame, AvroToDataFrame, CsvToDataFrame, DataFrameReader, ExcelToDataFrames,
        FwfToDataFrame, HtmlToDataFrame, JsonLineToDataFrame, JsonToDataFrame, LogfmtToDataFrame,
        MarkdownToDataFrame, ParquetToDataFrame, ReaderSource, SqliteToDataFrames,
    },
};

pub struct Reply<'a> {
    msg: Message,
    app: Option<&'a App>,
}

impl<'a> Reply<'a> {
    pub fn new(msg: Message) -> Self {
        Self { msg, app: None }
    }
    pub fn with_app(self, app: &'a App) -> Self {
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

fn handle(msg: Message, _app: Option<&App>) -> Option<Message> {
    match msg {
        Message::Ps => Some(Message::ps_reply()),
        Message::Schema { pid } if is_for_me(pid) => Some(Message::schema_replay(sql().schema())),
        Message::Sql { pid, query } if is_for_me(pid) => match sql().execute(&query, None) {
            Ok(df) => {
                let msg = format!("Query {} run successfully", query);
                handler::message::Message::TabsAddQueryPane(df, query).enqueue();
                Some(Message::sql_replay(msg))
            }
            Err(err) => {
                //
                Some(Message::sql_replay(format!(
                    "Query {} failed: {}",
                    query, err
                )))
            }
        },
        Message::Import { pid, spec } if is_for_me(pid) => {
            //
            match import_to_backend(spec) {
                Ok(names) => Some(Message::import_reply(format!(
                    "Import success with name(s) {} ",
                    names.join(", "),
                ))),
                Err(err) => Some(Message::import_reply(format!("Failed to import: {}", err))),
            }
        }

        _ => None,
    }
}

fn is_for_me(pid: u32) -> bool {
    std::process::id() == pid
}

fn import_to_backend(spec: ImportSpec) -> AppResult<Vec<String>> {
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
        ),
        ImportSpec::Parquet { source, max_rows } => register_file(
            ParquetToDataFrame::default().with_max_rows(max_rows),
            source,
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
        ),
        ImportSpec::Arrow { source, max_rows } => register_file(
            ArrowIpcToDataFrame::default().with_max_rows(max_rows),
            source,
        ),
        ImportSpec::Avro { source, max_rows } => {
            register_file(AvroToDataFrame::default().with_max_rows(max_rows), source)
        }
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
        ),
        ImportSpec::Sqlite { source, sqlite_key } => {
            let reader = SqliteToDataFrames::default();
            match sqlite_key {
                Some(key) => register_file(reader.key(key), source),
                None => register_file(reader, source),
            }
        }
        ImportSpec::Excel { source } => register_file(ExcelToDataFrames, source),
        ImportSpec::Logfmt { source } => register_file(LogfmtToDataFrame::default(), source),
        ImportSpec::Html { source } => register_file(HtmlToDataFrame, source),
        ImportSpec::Markdown { source } => register_file(MarkdownToDataFrame, source),
    }
}

fn register_file(reader: impl DataFrameReader, source: String) -> AppResult<Vec<String>> {
    let path = PathBuf::from(source);
    Ok(reader
        .read_to_data_frames(ReaderSource::File(path.clone()))?
        .into_iter()
        .map(|(name, df)| sql().register(&name, df, TableSource::File(path.clone())))
        .collect())
}

fn parse_widths(widths: &str) -> AppResult<Vec<usize>> {
    Ok(widths
        .split(',')
        .map(str::trim)
        .filter(|w| !w.is_empty())
        .map(str::parse)
        .collect::<Result<_, _>>()?)
}
