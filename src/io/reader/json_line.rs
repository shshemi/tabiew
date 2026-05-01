use std::fs::File;

use polars::{
    io::SerReader,
    prelude::{JsonFormat, JsonReader},
};

use crate::{
    AppResult,
    args::Args,
    io::{
        reader::ReaderSource,
        reader::{DataFrameReader, NamedFrames},
    },
    misc::stdin::stdin,
};

pub struct JsonLineToDataFrame {
    ignore_errors: bool,
}

impl JsonLineToDataFrame {
    pub fn from_args(args: &Args) -> Self {
        Self {
            ignore_errors: args.ignore_errors,
        }
    }
}

impl Default for JsonLineToDataFrame {
    fn default() -> Self {
        Self {
            ignore_errors: true,
        }
    }
}

impl DataFrameReader for JsonLineToDataFrame {
    fn read_to_data_frames(&self, input: ReaderSource) -> AppResult<NamedFrames> {
        let df = match &input {
            ReaderSource::File(path) => JsonReader::new(File::open(path)?)
                .with_json_format(JsonFormat::JsonLines)
                .infer_schema_len(None)
                .with_ignore_errors(self.ignore_errors)
                .set_rechunk(true)
                .finish()?,
            ReaderSource::Stdin => JsonReader::new(stdin())
                .with_json_format(JsonFormat::JsonLines)
                .infer_schema_len(None)
                .with_ignore_errors(self.ignore_errors)
                .set_rechunk(true)
                .finish()?,
        };
        Ok([(input.table_name(), df)].into())
    }
}
