use std::fs::File;

use polars::{io::SerReader, prelude::ParquetReader};

use crate::{
    AppResult,
    io::reader::{DataFrameReader, NamedFrames, ReaderSource},
    misc::stdin::stdin,
};

#[derive(Debug)]
pub struct ParquetToDataFrame;

impl DataFrameReader for ParquetToDataFrame {
    fn read_to_data_frames(&self, input: ReaderSource) -> AppResult<NamedFrames> {
        let df = match &input {
            ReaderSource::File(path) => ParquetReader::new(File::open(path)?)
                .set_rechunk(true)
                .finish()?,

            ReaderSource::Stdin => ParquetReader::new(stdin()).set_rechunk(true).finish()?,
        };
        Ok([(input.table_name(), df)].into())
    }
}
