use std::fs::File;

use polars::{io::SerReader, prelude::JsonReader};

use crate::{
    AppResult,
    args::Args,
    io::{
        reader::ReaderSource,
        reader::{DataFrameReader, NamedFrames},
    },
    misc::stdin::stdin,
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
        let df = match &input {
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
        let df = df.head(self.max_rows);
        Ok([(input.table_name(), df)].into())
    }
}
