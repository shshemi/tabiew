use std::{fs::File, path::PathBuf};

use polars::{
    frame::DataFrame,
    io::SerWriter,
    prelude::{CsvWriter, IpcWriter, JsonWriter, ParquetWriter},
};

use crate::AppResult;

pub trait WriteToFile {
    fn write_to_file(&self, path: PathBuf, data_frame: &mut DataFrame) -> AppResult<()>;
}
pub struct WriteToCsv {
    separator: char,
    quote: char,
    header: bool,
}

impl Default for WriteToCsv {
    fn default() -> Self {
        Self {
            separator: ',',
            quote: '"',
            header: false,
        }
    }
}

impl WriteToCsv {
    pub fn with_separator_char(mut self, c: char) -> Self {
        self.separator = c;
        self
    }
    pub fn with_quote_char(mut self, c: char) -> Self {
        self.quote = c;
        self
    }
    pub fn with_header(mut self, no_header: bool) -> Self {
        self.header = no_header;
        self
    }
}

impl WriteToFile for WriteToCsv {
    fn write_to_file(&self, path: PathBuf, data_frame: &mut DataFrame) -> AppResult<()> {
        Ok(CsvWriter::new(File::create(path)?)
            .with_separator(self.separator.try_into()?)
            .with_quote_char(self.quote.try_into()?)
            .include_header(self.header)
            .finish(data_frame)?)
    }
}

#[derive(Debug, Default)]
pub struct WriteToParquet;

impl WriteToFile for WriteToParquet {
    fn write_to_file(&self, path: PathBuf, data_frame: &mut DataFrame) -> AppResult<()> {
        ParquetWriter::new(File::create(path)?).finish(data_frame)?;
        Ok(())
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum JsonFormat {
    #[default]
    Json,
    JsonLine,
}

impl From<JsonFormat> for polars::prelude::JsonFormat {
    fn from(value: JsonFormat) -> Self {
        match value {
            JsonFormat::Json => polars::prelude::JsonFormat::Json,
            JsonFormat::JsonLine => polars::prelude::JsonFormat::JsonLines,
        }
    }
}

#[derive(Debug, Default)]
pub struct WriteToJson {
    fmt: JsonFormat,
}

impl WriteToJson {
    pub fn with_format(mut self, fmt: JsonFormat) -> Self {
        self.fmt = fmt;
        self
    }
}

impl WriteToFile for WriteToJson {
    fn write_to_file(&self, path: PathBuf, data_frame: &mut DataFrame) -> AppResult<()> {
        Ok(JsonWriter::new(File::create(path)?)
            .with_json_format(self.fmt.into())
            .finish(data_frame)?)
    }
}

#[derive(Debug, Default)]
pub struct WriteToArrow;

impl WriteToFile for WriteToArrow {
    fn write_to_file(&self, path: PathBuf, data_frame: &mut DataFrame) -> AppResult<()> {
        Ok(IpcWriter::new(File::create(path)?).finish(data_frame)?)
    }
}
