use serde::{Deserialize, Serialize};

use crate::args::CtlImportArgs;

#[derive(Debug, Serialize, Deserialize)]
pub enum ImportSpec {
    Csv {
        source: String,
        separator: char,
        quote_char: char,
        no_header: bool,
        ignore_errors: bool,
        truncate_ragged_lines: bool,
        max_rows: Option<usize>,
    },
    Parquet {
        source: String,
        max_rows: Option<usize>,
    },
    Json {
        source: String,
        ignore_errors: bool,
        max_rows: Option<usize>,
    },
    Jsonl {
        source: String,
        ignore_errors: bool,
        max_rows: Option<usize>,
    },
    Arrow {
        source: String,
        max_rows: Option<usize>,
    },
    Avro {
        source: String,
        max_rows: Option<usize>,
    },
    Fwf {
        source: String,
        widths: String,
        separator_length: usize,
        no_flexible_width: bool,
        no_header: bool,
    },
    Sqlite {
        source: String,
        sqlite_key: Option<String>,
    },
    Excel {
        source: String,
    },
    Logfmt {
        source: String,
    },
    Html {
        source: String,
    },
    Markdown {
        source: String,
    },
}

impl From<&CtlImportArgs> for ImportSpec {
    fn from(value: &CtlImportArgs) -> Self {
        match value {
            CtlImportArgs::Csv {
                common,
                separator,
                quote_char,
                no_header,
                ignore_errors,
                truncate_ragged_lines,
                max_rows,
            } => ImportSpec::Csv {
                source: common.source.clone(),
                separator: *separator,
                quote_char: *quote_char,
                no_header: *no_header,
                ignore_errors: *ignore_errors,
                truncate_ragged_lines: *truncate_ragged_lines,
                max_rows: *max_rows,
            },
            CtlImportArgs::Parquet { common, max_rows } => ImportSpec::Parquet {
                source: common.source.clone(),
                max_rows: *max_rows,
            },
            CtlImportArgs::Json {
                common,
                ignore_errors,
                max_rows,
            } => ImportSpec::Json {
                source: common.source.clone(),
                ignore_errors: *ignore_errors,
                max_rows: *max_rows,
            },
            CtlImportArgs::Jsonl {
                common,
                ignore_errors,
                max_rows,
            } => ImportSpec::Jsonl {
                source: common.source.clone(),
                ignore_errors: *ignore_errors,
                max_rows: *max_rows,
            },
            CtlImportArgs::Arrow { common, max_rows } => ImportSpec::Arrow {
                source: common.source.clone(),
                max_rows: *max_rows,
            },
            CtlImportArgs::Avro { common, max_rows } => ImportSpec::Avro {
                source: common.source.clone(),
                max_rows: *max_rows,
            },
            CtlImportArgs::Fwf {
                common,
                widths,
                separator_length,
                no_flexible_width,
                no_header,
            } => ImportSpec::Fwf {
                source: common.source.clone(),
                widths: widths.clone(),
                separator_length: *separator_length,
                no_flexible_width: *no_flexible_width,
                no_header: *no_header,
            },
            CtlImportArgs::Sqlite { common, sqlite_key } => ImportSpec::Sqlite {
                source: common.source.clone(),
                sqlite_key: sqlite_key.clone(),
            },
            CtlImportArgs::Excel { common } => ImportSpec::Excel {
                source: common.source.clone(),
            },
            CtlImportArgs::Logfmt { common } => ImportSpec::Logfmt {
                source: common.source.clone(),
            },
            CtlImportArgs::Html { common } => ImportSpec::Html {
                source: common.source.clone(),
            },
            CtlImportArgs::Markdown { common } => ImportSpec::Markdown {
                source: common.source.clone(),
            },
        }
    }
}
