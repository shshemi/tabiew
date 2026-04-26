use std::fs::File;

use polars::{io::SerReader, prelude::IpcReader};

use crate::{
    AppResult,
    io::{
        Resource,
        reader::{NamedFrames, ReadToDataFrames},
    },
    misc::stdin::stdin,
};

pub struct ArrowIpcToDataFrame;

impl ReadToDataFrames for ArrowIpcToDataFrame {
    fn read_to_data_frames(&self, input: Resource) -> AppResult<NamedFrames> {
        let df = match &input {
            Resource::LocalFile(path) => IpcReader::new(File::open(path)?)
                .set_rechunk(true)
                .finish()?,
            Resource::Stdin => IpcReader::new(stdin()).set_rechunk(true).finish()?,
        };
        Ok([(input.table_name(), df)].into())
    }
}
