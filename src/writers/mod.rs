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
pub(crate) use markdown::escape_cell;
pub use parquet::WriteToParquet;
use polars::frame::DataFrame;

use crate::{AppResult, handler::message::Message};

#[derive(Debug, Clone)]
pub enum Destination {
    File(PathBuf),
    Clipboard,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ExportFormat {
    Csv {
        separator: char,
        quote: char,
        header: bool,
    },
    Json,
    Jsonl,
    Parquet,
    Arrow,
    Avro,
    Markdown,
}

pub fn export(view: &str, format: ExportFormat, dest: Destination) {
    Message::AppExport(view.to_owned(), format, dest).enqueue();
    Message::PaneDismissModal.enqueue();
}

pub trait WriteToFile {
    fn write_to_file(&self, dest: Destination, data_frame: &mut DataFrame) -> AppResult<()>;
    fn format(&self) -> ExportFormat;
}
