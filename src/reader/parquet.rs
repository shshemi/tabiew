use std::fs::File;

use polars::{io::SerReader, prelude::ParquetReader};

use crate::{
    AppResult,
    misc::stdin::stdin,
    reader::{NamedFrames, ReadToDataFrames, Source},
};

pub struct ParquetToDataFrame;

impl ReadToDataFrames for ParquetToDataFrame {
    fn named_frames(&self, input: Source) -> AppResult<NamedFrames> {
        let df = match &input {
            Source::File(path) => ParquetReader::new(File::open(path)?)
                .set_rechunk(true)
                .finish()?,

            Source::Stdin => ParquetReader::new(stdin()).set_rechunk(true).finish()?,
        };
        Ok([(input.table_name(), df)].into())
    }
}
