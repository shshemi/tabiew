use std::fs::File;

use polars::{io::SerReader, prelude::IpcReader};

use crate::{
    AppResult,
    io::reader::{NamedFrames, ReadToDataFrames, Source},
    misc::stdin::stdin,
};

pub struct ArrowIpcToDataFrame;

impl ReadToDataFrames for ArrowIpcToDataFrame {
    fn read_to_data_frames(&self, input: Source) -> AppResult<NamedFrames> {
        let df = match &input {
            Source::File(path) => IpcReader::new(File::open(path)?)
                .set_rechunk(true)
                .finish()?,
            Source::Stdin => IpcReader::new(stdin()).set_rechunk(true).finish()?,
        };
        Ok([(input.table_name(), df)].into())
    }
}
