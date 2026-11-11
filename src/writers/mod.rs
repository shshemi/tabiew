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

#[derive(Debug, Clone)]
pub enum ExportData {
    Frame(DataFrame),
    View(String),
}

impl From<DataFrame> for ExportData {
    fn from(value: DataFrame) -> Self {
        Self::Frame(value)
    }
}

pub trait WriteToFile {
    fn write_to_file(&self, dest: Destination, data_frame: &mut DataFrame) -> AppResult<()>;
    fn format(&self) -> ExportFormat;
    fn export(&self, dest: Destination, data: &mut ExportData) -> AppResult<bool> {
        match data {
            ExportData::Frame(df) => self.write_to_file(dest, df).map(|()| true),
            ExportData::View(view) => {
                Message::AppExport(view.clone(), self.format(), dest).enqueue();
                Message::PaneDismissModal.enqueue();
                Ok(false)
            }
        }
    }
}
