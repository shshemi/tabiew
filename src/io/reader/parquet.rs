use std::fs::File;

use polars::{io::SerReader, prelude::ParquetReader};

use crate::{
    AppResult,
    io::{
        DataSource,
        reader::{NamedFrames, ReadToDataFrames},
    },
    misc::{download::download_to_temp, stdin::stdin},
};

pub struct ParquetToDataFrame;

impl ReadToDataFrames for ParquetToDataFrame {
    fn read_to_data_frames(&self, input: DataSource) -> AppResult<NamedFrames> {
        let df = match &input {
            DataSource::File(path) => ParquetReader::new(File::open(path)?)
                .set_rechunk(true)
                .finish()?,

            DataSource::Stdin => ParquetReader::new(stdin()).set_rechunk(true).finish()?,
            DataSource::Url(url) => {
                let temp = download_to_temp(url)?;
                ParquetReader::new(File::open(temp.path())?)
                    .set_rechunk(true)
                    .finish()?
            }
        };
        Ok([(input.table_name(), df)].into())
    }
}
