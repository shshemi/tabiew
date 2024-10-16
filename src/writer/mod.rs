use std::{fs::File, path::PathBuf};

use polars::{frame::DataFrame, io::SerWriter, prelude::CsvWriter};

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
