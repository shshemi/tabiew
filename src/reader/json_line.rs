use std::fs::File;

use polars::{io::SerReader, prelude::JsonLineReader};

use crate::{
    AppResult,
    args::Args,
    misc::stdin::stdin,
    reader::{NamedFrames, ReadToDataFrames, Source},
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

impl ReadToDataFrames for JsonLineToDataFrame {
    fn read_to_data_frames(&self, input: Source) -> AppResult<NamedFrames> {
        let df = match &input {
            Source::File(path) => JsonLineReader::new(File::open(path)?)
                .with_rechunk(true)
                .infer_schema_len(None)
                .with_ignore_errors(self.ignore_errors)
                .finish()?,
            Source::Stdin => JsonLineReader::new(stdin())
                .with_rechunk(true)
                .infer_schema_len(None)
                .with_ignore_errors(self.ignore_errors)
                .finish()?,
        };
        Ok([(input.table_name(), df)].into())
    }
}
