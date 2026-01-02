use std::{fs, io::Read};

use indexmap::IndexMap;
use logfmt_zerocopy::Logfmt;
use polars::{
    frame::DataFrame,
    prelude::{AnyValue, Column},
};

use crate::{
    AppResult,
    args::Args,
    misc::globals::stdin,
    reader::{NamedFrames, ReadToDataFrames, Source},
};

#[derive(Debug, Default)]
pub struct LogfmtToDataFrame {}

impl LogfmtToDataFrame {
    pub fn from_args(_args: &Args) -> Self {
        LogfmtToDataFrame::default()
    }
}

impl ReadToDataFrames for LogfmtToDataFrame {
    fn named_frames(&self, input: Source) -> AppResult<NamedFrames> {
        let contents = match &input {
            Source::File(path_buf) => fs::read_to_string(path_buf)?,
            Source::Stdin => {
                let mut s = String::new();
                stdin().read_to_string(&mut s)?;
                s
            }
        };
        let row_count = contents.lines().count();
        let mut data = IndexMap::new();

        for (row, line) in contents.lines().enumerate() {
            for (col, value) in line.logfmt() {
                if !data.contains_key(col) {
                    data.insert(col, vec![AnyValue::Null; row_count]);
                }
                data.get_mut(col).unwrap()[row] = AnyValue::String(value);
            }
        }

        let df = DataFrame::new(
            data.into_iter()
                .map(|(name, values)| Column::new(name.into(), values))
                .collect(),
        )?;
        Ok([(input.table_name(), df)].into())
    }
}
