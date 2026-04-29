use std::fs::File;

use polars::{
    io::SerReader,
    prelude::{JsonFormat, JsonReader},
};

use crate::{
    AppResult,
    args::Args,
    io::{
        DataSource,
        reader::{NamedFrames, ReadToDataFrames},
    },
    misc::{download::download_to_temp, stdin::stdin},
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
    fn read_to_data_frames(&self, input: DataSource) -> AppResult<NamedFrames> {
        let df = match &input {
            DataSource::File(path) => JsonReader::new(File::open(path)?)
                .with_json_format(JsonFormat::JsonLines)
                .infer_schema_len(None)
                .with_ignore_errors(self.ignore_errors)
                .set_rechunk(true)
                .finish()?,
            DataSource::Stdin => JsonReader::new(stdin())
                .with_json_format(JsonFormat::JsonLines)
                .infer_schema_len(None)
                .with_ignore_errors(self.ignore_errors)
                .set_rechunk(true)
                .finish()?,
            DataSource::Url(url) => {
                let temp = download_to_temp(url)?;
                JsonReader::new(File::open(temp.path())?)
                    .with_json_format(JsonFormat::JsonLines)
                    .infer_schema_len(None)
                    .with_ignore_errors(self.ignore_errors)
                    .set_rechunk(true)
                    .finish()?
            }
        };
        Ok([(input.table_name(), df)].into())
    }
}
