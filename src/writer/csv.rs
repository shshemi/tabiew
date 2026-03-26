use std::fs::File;

use anyhow::Ok;
use polars::{frame::DataFrame, io::SerWriter, prelude::CsvWriter};

use crate::{AppResult, misc::osc52::CopyToClipboardOsc52};

use super::traits::{Destination, WriteToFile};

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
