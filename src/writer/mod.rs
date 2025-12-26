use std::{fs::File, path::PathBuf};

use anyhow::Ok;
use polars::{
    frame::DataFrame,
    io::SerWriter,
    prelude::{CsvWriter, IpcWriter, JsonWriter, ParquetWriter},
};

use crate::{AppResult, misc::osc52::CopyToClipboardOsc52};

#[derive(Debug, Clone)]
pub enum Destination {
    File(PathBuf),
    Clipboard,
}

impl<T> From<T> for Destination
where
    T: AsRef<str>,
{
    fn from(value: T) -> Self {
        match value.as_ref() {
            "$clipboard" => Destination::Clipboard,
            _ => Destination::File(value.as_ref().into()),
        }
    }
}

pub trait WriteToFile {
    fn write_to_file(&self, dest: Destination, data_frame: &mut DataFrame) -> AppResult<()>;
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
    fn write_to_file(&self, dest: Destination, data_frame: &mut DataFrame) -> AppResult<()> {
        match dest {
            Destination::File(path) => Ok(CsvWriter::new(File::create(path)?)
                .with_separator(self.separator.try_into()?)
                .with_quote_char(self.quote.try_into()?)
                .include_header(self.header)
                .finish(data_frame)?),
            Destination::Clipboard => {
                let mut buf = Vec::new();
                CsvWriter::new(&mut buf)
                    .with_separator(self.separator.try_into()?)
                    .with_quote_char(self.quote.try_into()?)
                    .include_header(self.header)
                    .finish(data_frame)?;
                buf.copy_to_clipboard_via_osc52();
                Ok(())
            }
        }
    }
}

#[derive(Debug, Default)]
pub struct WriteToParquet;

impl WriteToFile for WriteToParquet {
    fn write_to_file(&self, dest: Destination, data_frame: &mut DataFrame) -> AppResult<()> {
        match dest {
            Destination::File(path) => {
                ParquetWriter::new(File::create(path)?).finish(data_frame)?;
                Ok(())
            }
            Destination::Clipboard => {
                let mut buf = Vec::new();
                ParquetWriter::new(&mut buf).finish(data_frame)?;
                buf.copy_to_clipboard_via_osc52();
                Ok(())
            }
        }
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
    fn write_to_file(&self, dest: Destination, data_frame: &mut DataFrame) -> AppResult<()> {
        match dest {
            Destination::File(path) => Ok(JsonWriter::new(File::create(path)?)
                .with_json_format(self.fmt.into())
                .finish(data_frame)?),
            Destination::Clipboard => {
                let mut buf = Vec::new();
                JsonWriter::new(&mut buf)
                    .with_json_format(self.fmt.into())
                    .finish(data_frame)?;
                buf.copy_to_clipboard_via_osc52();
                Ok(())
            }
        }
    }
}

#[derive(Debug, Default)]
pub struct WriteToArrow;

impl WriteToFile for WriteToArrow {
    fn write_to_file(&self, dest: Destination, data_frame: &mut DataFrame) -> AppResult<()> {
        match dest {
            Destination::File(path) => Ok(IpcWriter::new(File::create(path)?).finish(data_frame)?),
            Destination::Clipboard => {
                let mut buf = Vec::new();
                IpcWriter::new(&mut buf).finish(data_frame)?;
                buf.copy_to_clipboard_via_osc52();
                Ok(())
            }
        }
    }
}
