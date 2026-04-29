use std::fs::File;

use polars::{io::SerReader, prelude::ParquetReader};

use crate::{
    AppResult,
    io::{
        Resource,
        reader::{NamedFrames, ReadToDataFrames},
    },
    misc::{download::download_to_temp, stdin::stdin},
};

pub struct ParquetToDataFrame;

impl ReadToDataFrames for ParquetToDataFrame {
    fn read_to_data_frames(&self, input: Resource) -> AppResult<NamedFrames> {
        let df = match &input {
            Resource::File(path) => ParquetReader::new(File::open(path)?)
                .set_rechunk(true)
                .finish()?,

            Resource::Stdin => ParquetReader::new(stdin()).set_rechunk(true).finish()?,
            Resource::Url(url) => {
                let temp = download_to_temp(url)?;
                ParquetReader::new(File::open(temp.path())?)
                    .set_rechunk(true)
                    .finish()?
            }
        };
        Ok([(input.table_name(), df)].into())
    }
}
