use std::{
    collections::HashSet,
    io::{Cursor, Read},
    iter::once,
};

use fwf_rs::Reader;
use itertools::Itertools;
use polars::{frame::DataFrame, prelude::NamedFrom, series::Series};

use crate::{
    args::{Args, InferSchema},
    utils::{safe_infer_schema, ZipItersExt},
    AppResult,
};

use super::ReadToDataFrame;

pub struct ReadFwfToDataFrame {
    width_str: String,
    has_header: bool,
    separator_length: usize,
    flexible_width: bool,
    infer_schema: InferSchema,
}

impl ReadFwfToDataFrame {
    pub fn from_args(args: &Args) -> Self {
        Self {
            width_str: args.widths.to_owned(),
            has_header: !args.no_header,
            separator_length: args.separator_length,
            flexible_width: !args.no_flexible_width,
            infer_schema: args.infer_schema,
        }
    }
}

impl<R: Read> ReadToDataFrame<R> for ReadFwfToDataFrame {
    fn read_to_data_frame(&self, mut reader: R) -> AppResult<DataFrame> {
        let file_content = {
            let mut buf = String::new();
            reader.read_to_string(&mut buf)?;
            buf
        };
        let widths = if self.width_str.is_empty() {
            let common_space_indices = file_content
                .lines()
                .map(|line| {
                    let length = line.chars().count();
                    let spaces = line
                        .chars()
                        .enumerate()
                        .filter_map(|(i, c)| c.is_whitespace().then_some(i))
                        .collect::<HashSet<usize>>();
                    (length, spaces)
                })
                .reduce(|(la, sa), (lb, sb)| (la.max(lb), sa.intersection(&sb).copied().collect()))
                .map(|(len, idx_set)| idx_set.into_iter().chain(once(len)).sorted().collect_vec())
                .unwrap_or_default();
            infer_widths(common_space_indices)
        } else {
            parse_width(&self.width_str)?
        };

        let reader = Reader::new(
            Cursor::new(file_content),
            widths.clone(),
            self.separator_length,
            self.flexible_width,
            self.has_header,
        )?;
        let header = reader
            .header()
            .map(|rec| rec.iter().map(|slice| slice.trim().to_owned()).collect())
            .unwrap_or_else(|| {
                (0..widths.len())
                    .map(|idx| format!("column_{}", idx + 1))
                    .collect_vec()
            });

        let columns = reader
            .records()
            .filter_map(Result::ok)
            .map(|record| {
                record
                    .iter()
                    .map(str::trim)
                    .map(ToOwned::to_owned)
                    .collect_vec()
                    .into_iter()
            })
            .zip_iters()
            .collect_vec();

        let mut df = DataFrame::new(
            header
                .into_iter()
                .zip(columns)
                .map(|(name, values)| Series::new(name.into(), values))
                .collect_vec(),
        )?;

        if matches!(
            self.infer_schema,
            InferSchema::Fast | InferSchema::Full | InferSchema::Safe
        ) {
            safe_infer_schema(&mut df);
        }

        Ok(df)
    }
}

fn parse_width(widths: impl AsRef<str>) -> AppResult<Vec<usize>> {
    Ok(widths
        .as_ref()
        .split(',')
        .map(|w| w.parse::<usize>())
        .collect::<Result<Vec<_>, _>>()?)
}

fn infer_widths(space_indices: Vec<usize>) -> Vec<usize> {
    let mut indices = Vec::default();
    let mut start = 0;
    // let chars = line.chars().collect_vec();
    for (i, idx) in space_indices.iter().enumerate() {
        if let Some(nidx) = space_indices.get(i + 1) {
            if nidx - idx > 1 {
                indices.push(idx - start);
                start = idx + 1
            }
        } else {
            indices.push(idx - start);
        }
    }
    indices
}
