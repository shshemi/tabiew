use std::fs::File;

use anyhow::anyhow;
use polars::{
    frame::DataFrame,
    io::{SerReader, mmap::MmapBytesReader},
    prelude::{CsvParseOptions, CsvReadOptions},
};

use crate::{
    AppResult,
    args::{Args, InferSchema},
    io::reader::{DataFrameReader, NamedFrames, ReaderSource},
    misc::{stdin::stdin, type_ext::ToAscii},
};

pub struct CsvToDataFrame {
    infer_schema: InferSchema,
    quote_char: char,
    separator_char: char,
    no_header: bool,
    ignore_errors: bool,
    truncate_ragged_lines: bool,
}

impl CsvToDataFrame {
    pub fn from_args(args: &Args) -> Self {
        Self {
            infer_schema: args.infer_schema,
            quote_char: args.quote_char,
            separator_char: args.separator,
            no_header: args.no_header,
            ignore_errors: args.ignore_errors,
            truncate_ragged_lines: args.truncate_ragged_lines,
        }
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
            .with_infer_schema_length(self.infer_schema.to_csv_infer_schema_length())
            .with_has_header(!self.no_header)
            .with_parse_options(
                CsvParseOptions::default()
                    .with_truncate_ragged_lines(self.truncate_ragged_lines)
                    .with_quote_char(self.quote_char.to_ascii())
                    .with_separator(
                        self.separator_char
                            .to_ascii()
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
            infer_schema: InferSchema::Safe,
            quote_char: '"',
            separator_char: ',',
            no_header: false,
            ignore_errors: true,
            truncate_ragged_lines: false,
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
