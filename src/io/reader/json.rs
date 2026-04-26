use std::fs::File;

use polars::{io::SerReader, prelude::JsonReader};

use crate::{
    AppResult,
    args::Args,
    io::{
        Resource,
        reader::{NamedFrames, ReadToDataFrames},
    },
    misc::stdin::stdin,
};

pub struct JsonToDataFrame {
    ignore_errors: bool,
}

impl JsonToDataFrame {
    pub fn from_args(args: &Args) -> Self {
        Self {
            ignore_errors: args.ignore_errors,
        }
    }
}

impl Default for JsonToDataFrame {
    fn default() -> Self {
        Self {
            ignore_errors: true,
        }
    }
}

impl ReadToDataFrames for JsonToDataFrame {
    fn read_to_data_frames(&self, input: Resource) -> AppResult<NamedFrames> {
        let df = match &input {
            Resource::LocalFile(path) => JsonReader::new(File::open(path)?)
                .set_rechunk(true)
                .infer_schema_len(None)
                .with_ignore_errors(self.ignore_errors)
                .finish()?,
            Resource::Stdin => JsonReader::new(stdin())
                .set_rechunk(true)
                .infer_schema_len(None)
                .with_ignore_errors(self.ignore_errors)
                .finish()?,
        };
        Ok([(input.table_name(), df)].into())
    }
}
