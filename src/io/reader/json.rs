use std::fs::File;

use polars::{io::SerReader, prelude::JsonReader};

use crate::{
    AppResult,
    args::Args,
    io::{
        DataSource,
        reader::{NamedFrames, ReadToDataFrames},
    },
    misc::{download::download_to_temp, stdin::stdin},
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
    fn read_to_data_frames(&self, input: DataSource) -> AppResult<NamedFrames> {
        let df = match &input {
            DataSource::File(path) => JsonReader::new(File::open(path)?)
                .set_rechunk(true)
                .infer_schema_len(None)
                .with_ignore_errors(self.ignore_errors)
                .finish()?,
            DataSource::Stdin => JsonReader::new(stdin())
                .set_rechunk(true)
                .infer_schema_len(None)
                .with_ignore_errors(self.ignore_errors)
                .finish()?,
            DataSource::Url(url) => {
                let temp = download_to_temp(url)?;
                JsonReader::new(File::open(temp.path())?)
                    .set_rechunk(true)
                    .infer_schema_len(None)
                    .with_ignore_errors(self.ignore_errors)
                    .finish()?
            }
        };
        Ok([(input.table_name(), df)].into())
    }
}
