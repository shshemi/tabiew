use std::fs::File;

use polars::{io::SerReader, prelude::IpcReader};

use crate::{
    AppResult,
    io::reader::{DataFrameReader, NamedFrames, ReaderSource},
    misc::stdin::stdin,
};

#[derive(Debug)]
pub struct ArrowIpcToDataFrame;

impl DataFrameReader for ArrowIpcToDataFrame {
    fn read_to_data_frames(&self, input: ReaderSource) -> AppResult<NamedFrames> {
        let df = match &input {
            ReaderSource::File(path) => IpcReader::new(File::open(path)?)
                .set_rechunk(true)
                .finish()?,
            ReaderSource::Stdin => IpcReader::new(stdin()).set_rechunk(true).finish()?,
        };
        Ok([(input.table_name(), df)].into())
    }
}
