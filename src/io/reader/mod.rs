mod arrow;
mod csv;
mod excel;
mod fwf;
mod json;
mod json_line;
mod logfmt;
mod parquet;
mod sqlite;
mod traits;

pub use arrow::ArrowIpcToDataFrame;
pub use csv::CsvToDataFrame;
pub use excel::ExcelToDataFrames;
pub use fwf::FwfToDataFrame;
pub use json::JsonToDataFrame;
pub use json_line::JsonLineToDataFrame;
pub use logfmt::LogfmtToDataFrame;
pub use parquet::ParquetToDataFrame;
pub use sqlite::SqliteToDataFrames;
pub use traits::{BuildReader, NamedFrames, ReadToDataFrames};

use std::{borrow::Cow, convert::Infallible, ffi::OsStr, path::PathBuf, str::FromStr};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum ReaderSource {
    File(PathBuf),
    Stdin,
}

impl ReaderSource {
    pub fn table_name(&self) -> String {
        match self {
            ReaderSource::File(path_buf) => path_buf
                .file_stem()
                .map(OsStr::to_string_lossy)
                .unwrap_or("unknown".into())
                .into_owned(),
            ReaderSource::Stdin => String::from("Stdin"),
        }
    }

    pub fn display_path(&self) -> Cow<'_, str> {
        match self {
            ReaderSource::File(path_buf) => {
                path_buf.file_name().unwrap_or_default().to_string_lossy()
            }
            ReaderSource::Stdin => Cow::Borrowed("Stdin"),
        }
    }
}

impl FromStr for ReaderSource {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(ReaderSource::File(PathBuf::from(s)))
    }
}
