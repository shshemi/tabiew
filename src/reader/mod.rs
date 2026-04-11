mod arrow;
pub(crate) mod csv;
mod excel;
pub(crate) mod fwf;
mod json;
pub(crate) mod json_line;
pub(crate) mod logfmt;
mod parquet;
mod source;
mod sqlite;
pub(crate) mod traits;

pub use arrow::ArrowIpcToDataFrame;
pub use csv::{CsvStreamReader, CsvToDataFrame};
pub use excel::ExcelToDataFarmes;
pub use fwf::FwfToDataFrame;
pub use json::JsonToDataFrame;
pub use json_line::JsonLineToDataFrame;
pub use logfmt::LogfmtToDataFrame;
pub use parquet::ParquetToDataFrame;
pub use source::Source;
pub use sqlite::SqliteToDataFrames;
pub use traits::{
    BuildReader, BuildStreamReader, NamedFrames, ReadToDataFrames, StreamEvent, StreamToDataFrames,
};
