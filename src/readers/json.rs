use std::fs::File;

use polars::{io::SerReader, prelude::JsonReader};

use crate::{
    AppResult,
    args::Args,
    misc::stdin::stdin,
    readers::{DataFrameReader, NamedFrames, ReaderSource},
};

#[derive(Debug)]
pub struct JsonToDataFrame {
    ignore_errors: bool,
    max_rows: Option<usize>,
}

impl JsonToDataFrame {
    pub fn from_args(args: &Args) -> Self {
        Self {
            ignore_errors: args.ignore_errors,
            max_rows: args.max_rows,
        }
    }

    pub fn with_ignore_errors(mut self, ignore_errors: bool) -> Self {
        self.ignore_errors = ignore_errors;
        self
    }

    pub fn with_max_rows(mut self, max_rows: Option<usize>) -> Self {
        self.max_rows = max_rows;
        self
    }
}

impl Default for JsonToDataFrame {
    fn default() -> Self {
        Self {
            ignore_errors: true,
            max_rows: None,
        }
    }
}

impl DataFrameReader for JsonToDataFrame {
    fn read_to_data_frames(&self, input: ReaderSource) -> AppResult<NamedFrames> {
        let mut df = match &input {
            ReaderSource::File(path) => JsonReader::new(File::open(path)?)
                .set_rechunk(true)
                .infer_schema_len(None)
                .with_ignore_errors(self.ignore_errors)
                .finish()?,
            ReaderSource::Stdin => JsonReader::new(stdin())
                .set_rechunk(true)
                .infer_schema_len(None)
                .with_ignore_errors(self.ignore_errors)
                .finish()?,
        };
        // JsonReader has no native row limit, so cap the frame after reading.
        if self.max_rows.is_some() {
            df = df.head(self.max_rows);
        }
        Ok([(input.table_name(), df)].into())
    }
}
