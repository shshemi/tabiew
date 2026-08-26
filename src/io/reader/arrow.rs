use std::fs::File;

use polars::{io::SerReader, prelude::IpcReader};

use crate::{
    AppResult,
    args::Args,
    io::reader::{DataFrameReader, NamedFrames, ReaderSource},
    misc::stdin::stdin,
};

#[derive(Debug, Default)]
pub struct ArrowIpcToDataFrame {
    max_rows: Option<usize>,
}

impl ArrowIpcToDataFrame {
    pub fn from_args(args: &Args) -> Self {
        Self {
            max_rows: args.max_rows,
        }
    }
}

impl DataFrameReader for ArrowIpcToDataFrame {
    fn read_to_data_frames(&self, input: ReaderSource) -> AppResult<NamedFrames> {
        let df = match &input {
            ReaderSource::File(path) => IpcReader::new(File::open(path)?)
                .with_n_rows(self.max_rows)
                .set_rechunk(true)
                .finish()?,
            ReaderSource::Stdin => IpcReader::new(stdin())
                .with_n_rows(self.max_rows)
                .set_rechunk(true)
                .finish()?,
        };
        Ok([(input.table_name(), df)].into())
    }
}
