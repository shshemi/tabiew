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
    misc::stdin::stdin,
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
                insert(&mut data, col, row, row_count, value);
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

fn insert<'k, 'v>(
    data: &mut IndexMap<&'k str, Vec<AnyValue<'v>>>,
    col: &'k str,
    row: usize,
    row_count: usize,
    value: &'v str,
) {
    if let Some(col) = data.get_mut(col) {
        col[row] = AnyValue::String(value);
    } else {
        let mut new_col = vec![AnyValue::Null; row_count];
        new_col[row] = AnyValue::String(value);
        data.insert(col, new_col);
    }
}
