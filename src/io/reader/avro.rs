use std::fs::File;

use polars::io::{SerReader, avro::AvroReader};

use crate::{
    AppResult,
    args::Args,
    io::reader::{DataFrameReader, NamedFrames, ReaderSource},
    misc::stdin::stdin,
};

#[derive(Debug, Default)]
pub struct AvroToDataFrame {
    max_rows: Option<usize>,
}

impl AvroToDataFrame {
    pub fn from_args(args: &Args) -> Self {
        Self {
            max_rows: args.max_rows,
        }
    }
}

impl DataFrameReader for AvroToDataFrame {
    fn read_to_data_frames(&self, input: ReaderSource) -> AppResult<NamedFrames> {
        let df = match &input {
            ReaderSource::File(path) => AvroReader::new(File::open(path)?)
                .with_n_rows(self.max_rows)
                .set_rechunk(true)
                .finish()?,
            ReaderSource::Stdin => AvroReader::new(stdin())
                .with_n_rows(self.max_rows)
                .set_rechunk(true)
                .finish()?,
        };
        Ok([(input.table_name(), df)].into())
    }
}
