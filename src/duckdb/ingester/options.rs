use crate::{
    args::{Args, Format},
    duckdb::ingester::formats::{CsvOptions, FwfOptions, type_candidates},
};
use clap::ValueEnum;
use serde::{Serialize, Serializer};

pub(super) const TYPES: [&str; 5] = ["BIGINT", "DOUBLE", "BOOLEAN", "DATE", "TIMESTAMP"];

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct IngestOptions {
    #[serde(serialize_with = "format_name")]
    pub format: Option<Format>,
    pub csv: CsvOptions,
    pub fwf: FwfOptions,
    pub types: Vec<String>,
    #[serde(skip)]
    pub sqlite_key: Option<String>,
}

impl IngestOptions {
    pub fn from_args(args: &Args) -> Self {
        Self {
            format: args.format.clone(),
            csv: CsvOptions::from_args(args),
            fwf: FwfOptions::from_args(args),
            types: types_from_args(args),
            sqlite_key: args.sqlite_key.clone(),
        }
    }
}

impl Default for IngestOptions {
    fn default() -> Self {
        Self {
            format: None,
            csv: CsvOptions::default(),
            fwf: FwfOptions::default(),
            types: vec!["BIGINT".to_string(), "DOUBLE".to_string()],
            sqlite_key: None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Import {
    Format(Format),
    Csv {
        delimiter: char,
        quote: char,
        header: bool,
        ignore_errors: bool,
        null_padding: bool,
    },
    Fwf(FwfOptions),
    Sqlite(Option<String>),
    Json {
        lines: bool,
        ignore_errors: bool,
    },
}

impl Import {
    pub fn csv(delimiter: char, quote: char, header: bool) -> Self {
        Import::Csv {
            delimiter,
            quote,
            header,
            ignore_errors: false,
            null_padding: false,
        }
    }

    pub fn options(&self, base: IngestOptions) -> IngestOptions {
        match self {
            Import::Format(format) => IngestOptions {
                format: Some(format.clone()),
                ..base
            },
            Import::Csv {
                delimiter,
                quote,
                header,
                ignore_errors,
                null_padding,
            } => IngestOptions {
                format: Some(Format::Csv),
                csv: CsvOptions {
                    delimiter: *delimiter,
                    quote: *quote,
                    header: *header,
                    ignore_errors: *ignore_errors,
                    null_padding: *null_padding,
                    ..base.csv
                },
                ..base
            },
            Import::Fwf(fwf) => IngestOptions {
                format: Some(Format::Fwf),
                fwf: fwf.clone(),
                ..base
            },
            Import::Sqlite(key) => IngestOptions {
                format: Some(Format::Sqlite),
                sqlite_key: key.clone(),
                ..base
            },
            Import::Json {
                lines,
                ignore_errors,
            } => IngestOptions {
                format: Some(match lines {
                    true => Format::Jsonl,
                    false => Format::Json,
                }),
                csv: CsvOptions {
                    ignore_errors: *ignore_errors,
                    ..base.csv
                },
                ..base
            },
        }
    }
}

fn types_from_args(args: &Args) -> Vec<String> {
    if args.no_type_inference {
        return Vec::new();
    }
    let types = type_candidates(args.infer_types.inner())
        .unwrap_or_else(|| TYPES.iter().map(|kind| kind.to_string()).collect());
    TYPES
        .iter()
        .filter(|kind| types.iter().any(|t| t == *kind))
        .map(|kind| kind.to_string())
        .collect()
}

fn format_name<S: Serializer>(format: &Option<Format>, serializer: S) -> Result<S::Ok, S::Error> {
    let name = format
        .as_ref()
        .and_then(ValueEnum::to_possible_value)
        .map(|value| value.get_name().to_string());
    name.serialize(serializer)
}
