mod arrow;
mod avro;
mod csv;
mod json;
mod markdown;
mod parquet;

use std::path::PathBuf;

pub use arrow::WriteToArrow;
pub use avro::WriteToAvro;
pub use csv::WriteToCsv;
pub use json::{JsonFormat, WriteToJson};
pub use markdown::WriteToMarkdown;
pub use parquet::WriteToParquet;
use polars::frame::DataFrame;

use crate::AppResult;

#[derive(Debug, Clone)]
pub enum Destination {
    File(PathBuf),
    Clipboard,
}

pub trait WriteToFile {
    fn write_to_file(&self, dest: Destination, data_frame: &mut DataFrame) -> AppResult<()>;
}
