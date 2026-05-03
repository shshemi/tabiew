use std::fs::File;

use polars::io::{SerReader, avro::AvroReader};

use crate::{
    AppResult,
    io::reader::{DataFrameReader, NamedFrames, ReaderSource},
    misc::stdin::stdin,
};

#[derive(Debug)]
pub struct AvroToDataFrame;

impl DataFrameReader for AvroToDataFrame {
    fn read_to_data_frames(&self, input: ReaderSource) -> AppResult<NamedFrames> {
        let df = match &input {
            ReaderSource::File(path) => AvroReader::new(File::open(path)?)
                .set_rechunk(true)
                .finish()?,
            ReaderSource::Stdin => AvroReader::new(stdin()).set_rechunk(true).finish()?,
        };
        Ok([(input.table_name(), df)].into())
    }
}
