use std::fs::File;

use polars::{io::SerReader, prelude::ParquetReader};

use crate::{
    AppResult,
    args::Args,
    io::reader::{DataFrameReader, NamedFrames, ReaderSource},
    misc::stdin::stdin,
};

#[derive(Debug, Default)]
pub struct ParquetToDataFrame {
    max_rows: Option<usize>,
}

impl ParquetToDataFrame {
    pub fn from_args(args: &Args) -> Self {
        Self {
            max_rows: args.max_rows,
        }
    }

    fn slice(&self) -> Option<(usize, usize)> {
        self.max_rows.map(|n| (0, n))
    }
}

impl DataFrameReader for ParquetToDataFrame {
    fn read_to_data_frames(&self, input: ReaderSource) -> AppResult<NamedFrames> {
        let df = match &input {
            ReaderSource::File(path) => ParquetReader::new(File::open(path)?)
                .with_slice(self.slice())
                .set_rechunk(true)
                .finish()?,

            ReaderSource::Stdin => ParquetReader::new(stdin())
                .with_slice(self.slice())
                .set_rechunk(true)
                .finish()?,
        };
        Ok([(input.table_name(), df)].into())
    }
}
