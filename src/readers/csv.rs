use std::fs::File;

use anyhow::anyhow;
use polars::{
    frame::DataFrame,
    io::{SerReader, mmap::MmapBytesReader},
    prelude::{CsvParseOptions, CsvReadOptions},
};

use crate::{
    AppResult,
    args::Args,
    misc::stdin::stdin,
    readers::{DataFrameReader, NamedFrames, ReaderSource},
};

#[derive(Debug)]
pub struct CsvToDataFrame {
    quote_char: char,
    separator_char: char,
    no_header: bool,
    ignore_errors: bool,
    truncate_ragged_lines: bool,
    max_rows: Option<usize>,
}

impl CsvToDataFrame {
    pub fn from_args(args: &Args) -> Self {
        Self {
            quote_char: args.quote_char,
            separator_char: args.separator,
            no_header: args.no_header,
            ignore_errors: args.ignore_errors,
            truncate_ragged_lines: args.truncate_ragged_lines,
            max_rows: args.max_rows,
        }
    }

    pub fn with_ignore_errors(mut self, ignore_errors: bool) -> Self {
        self.ignore_errors = ignore_errors;
        self
    }

    pub fn with_truncate_ragged_lines(mut self, truncate_ragged_lines: bool) -> Self {
        self.truncate_ragged_lines = truncate_ragged_lines;
        self
    }

    pub fn with_max_rows(mut self, max_rows: Option<usize>) -> Self {
        self.max_rows = max_rows;
        self
    }

    pub fn with_separator(mut self, c: char) -> Self {
        self.separator_char = c;
        self
    }

    pub fn with_no_header(mut self, no_header: bool) -> Self {
        self.no_header = no_header;
        self
    }

    pub fn with_quote_char(mut self, c: char) -> Self {
        self.quote_char = c;
        self
    }

    fn try_into_frame(&self, reader: impl MmapBytesReader) -> AppResult<DataFrame> {
        let df = CsvReadOptions::default()
            .with_ignore_errors(self.ignore_errors)
            .with_infer_schema_length(0.into())
            .with_has_header(!self.no_header)
            .with_n_rows(self.max_rows)
            .with_parse_options(
                CsvParseOptions::default()
                    .with_truncate_ragged_lines(self.truncate_ragged_lines)
                    .with_quote_char(to_ascii(self.quote_char))
                    .with_separator(
                        to_ascii(self.separator_char)
                            .ok_or(anyhow!("non-ASCII separator character"))?,
                    ),
            )
            .with_rechunk(true)
            .into_reader_with_file_handle(reader)
            .finish()?;
        Ok(df)
    }
}

impl Default for CsvToDataFrame {
    fn default() -> Self {
        Self {
            quote_char: '"',
            separator_char: ',',
            no_header: false,
            ignore_errors: true,
            truncate_ragged_lines: false,
            max_rows: None,
        }
    }
}

impl DataFrameReader for CsvToDataFrame {
    fn read_to_data_frames(&self, input: ReaderSource) -> AppResult<NamedFrames> {
        let df = match &input {
            ReaderSource::File(path) => self.try_into_frame(File::open(path)?),
            ReaderSource::Stdin => self.try_into_frame(stdin()),
        }?;
        Ok([(input.table_name(), df)].into())
    }
}

#[inline]
fn to_ascii(c: char) -> Option<u8> {
    c.is_ascii().then_some(c as u8)
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    const CSV: &[u8] = b"a,b\n1,2\n3,4\n5,6\n7,8\n";

    #[test]
    fn reads_every_row_without_a_cap() {
        let reader = CsvToDataFrame::default();
        let df = reader.try_into_frame(Cursor::new(CSV.to_vec())).unwrap();
        assert_eq!(df.height(), 4);
    }

    #[test]
    fn caps_rows_when_max_rows_is_set() {
        let reader = CsvToDataFrame {
            max_rows: Some(2),
            ..Default::default()
        };
        let df = reader.try_into_frame(Cursor::new(CSV.to_vec())).unwrap();
        assert_eq!(df.height(), 2);
    }

    #[test]
    fn max_rows_larger_than_the_file_keeps_every_row() {
        let reader = CsvToDataFrame {
            max_rows: Some(100),
            ..Default::default()
        };
        let df = reader.try_into_frame(Cursor::new(CSV.to_vec())).unwrap();
        assert_eq!(df.height(), 4);
    }
}
