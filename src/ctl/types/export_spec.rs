use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::args::CtlExportArgs;

#[derive(Debug, Serialize, Deserialize)]
pub enum ExportSpec {
    Csv {
        path: PathBuf,
        separator: char,
        quote_char: char,
        no_header: bool,
    },
    Tsv {
        path: PathBuf,
        quote_char: char,
        no_header: bool,
    },
    Json {
        path: PathBuf,
    },
    Jsonl {
        path: PathBuf,
    },
    Parquet {
        path: PathBuf,
    },
    Arrow {
        path: PathBuf,
    },
    Avro {
        path: PathBuf,
    },
    Markdown {
        path: PathBuf,
    },
}

impl ExportSpec {
    pub fn path(&self) -> &Path {
        match self {
            ExportSpec::Csv { path, .. }
            | ExportSpec::Tsv { path, .. }
            | ExportSpec::Json { path }
            | ExportSpec::Jsonl { path }
            | ExportSpec::Parquet { path }
            | ExportSpec::Arrow { path }
            | ExportSpec::Avro { path }
            | ExportSpec::Markdown { path } => path,
        }
    }
}

impl From<&CtlExportArgs> for ExportSpec {
    /// Paths are made absolute here, against the sender's current directory, since the
    /// receiving instance may run in a different one.
    fn from(value: &CtlExportArgs) -> Self {
        let absolute = |path: &PathBuf| std::path::absolute(path).unwrap_or_else(|_| path.clone());
        match value {
            CtlExportArgs::Csv {
                common,
                separator,
                quote_char,
                no_header,
            } => ExportSpec::Csv {
                path: absolute(&common.path),
                separator: *separator,
                quote_char: *quote_char,
                no_header: *no_header,
            },
            CtlExportArgs::Tsv {
                common,
                quote_char,
                no_header,
            } => ExportSpec::Tsv {
                path: absolute(&common.path),
                quote_char: *quote_char,
                no_header: *no_header,
            },
            CtlExportArgs::Json { common } => ExportSpec::Json {
                path: absolute(&common.path),
            },
            CtlExportArgs::Jsonl { common } => ExportSpec::Jsonl {
                path: absolute(&common.path),
            },
            CtlExportArgs::Parquet { common } => ExportSpec::Parquet {
                path: absolute(&common.path),
            },
            CtlExportArgs::Arrow { common } => ExportSpec::Arrow {
                path: absolute(&common.path),
            },
            CtlExportArgs::Avro { common } => ExportSpec::Avro {
                path: absolute(&common.path),
            },
            CtlExportArgs::Markdown { common } => ExportSpec::Markdown {
                path: absolute(&common.path),
            },
        }
    }
}
