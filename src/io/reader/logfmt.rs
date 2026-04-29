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
    io::{
        DataSource,
        reader::{NamedFrames, ReadToDataFrames},
    },
    misc::{download::download_to_temp, stdin::stdin},
};

#[derive(Debug, Default)]
pub struct LogfmtToDataFrame {}

impl LogfmtToDataFrame {
    pub fn from_args(_args: &Args) -> Self {
        LogfmtToDataFrame::default()
    }
}

impl ReadToDataFrames for LogfmtToDataFrame {
    fn read_to_data_frames(&self, input: DataSource) -> AppResult<NamedFrames> {
        let contents = match &input {
            DataSource::File(path_buf) => fs::read_to_string(path_buf)?,
            DataSource::Stdin => {
                let mut s = String::new();
                stdin().read_to_string(&mut s)?;
                s
            }
            DataSource::Url(url) => {
                let temp = download_to_temp(url)?;
                fs::read_to_string(temp.path())?
            }
        };
        let row_count = contents.lines().count();
        let mut data = IndexMap::new();

        for (row, line) in contents.lines().enumerate() {
            for (col, value) in line.logfmt() {
                insert(&mut data, col, row, row_count, value);
            }
        }

        let df = DataFrame::new_infer_height(
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
