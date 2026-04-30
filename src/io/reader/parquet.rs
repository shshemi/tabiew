use std::fs::File;

use polars::{io::SerReader, prelude::ParquetReader};

use crate::{
    AppResult,
    io::{
        reader::ReaderSource,
        reader::{NamedFrames, ReadToDataFrames},
    },
    misc::stdin::stdin,
};

pub struct ParquetToDataFrame;

impl ReadToDataFrames for ParquetToDataFrame {
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
