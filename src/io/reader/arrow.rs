use std::fs::File;

use polars::{io::SerReader, prelude::IpcReader};

use crate::{
    AppResult,
    io::{
        DataSource,
        reader::{NamedFrames, ReadToDataFrames},
    },
    misc::{download::download_to_temp, stdin::stdin},
};

pub struct ArrowIpcToDataFrame;

impl ReadToDataFrames for ArrowIpcToDataFrame {
    fn read_to_data_frames(&self, input: DataSource) -> AppResult<NamedFrames> {
        let df = match &input {
            DataSource::File(path) => IpcReader::new(File::open(path)?)
                .set_rechunk(true)
                .finish()?,
            DataSource::Stdin => IpcReader::new(stdin()).set_rechunk(true).finish()?,
            DataSource::Url(url) => {
                let temp = download_to_temp(url)?;
                IpcReader::new(File::open(temp.path())?)
                    .set_rechunk(true)
                    .finish()?
            }
        };
        Ok([(input.table_name(), df)].into())
    }
}
