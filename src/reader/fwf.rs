use std::{
    collections::HashSet,
    fs::read_to_string,
    io::{self, Cursor, Read},
    iter::once,
};

use fwf_rs::Reader;
use itertools::Itertools;
use polars::{frame::DataFrame, prelude::NamedFrom, series::Series};

use crate::{
    AppResult,
    args::Args,
    misc::{iter_ext::ZipItersExt, snake_case_name_gen::SnakeCaseNameGenExt},
};

use super::{NamedFrames, ReadToDataFrames, Source};

pub struct FwfToDataFrame {
    widths: Vec<usize>,
    has_header: bool,
    separator_length: usize,
    flexible_width: bool,
}

impl FwfToDataFrame {
    pub fn from_args(args: &Args) -> Self {
        Self {
            widths: parse_width(&args.widths).unwrap_or_default(),
            has_header: !args.no_header,
            separator_length: args.separator_length,
            flexible_width: !args.no_flexible_width,
        }
    }

    pub fn with_widths(mut self, widths: Vec<usize>) -> Self {
        self.widths = widths;
        self
    }

    pub fn with_has_header(mut self, has_header: bool) -> Self {
        self.has_header = has_header;
        self
    }

    pub fn with_separator_length(mut self, separator_length: usize) -> Self {
        self.separator_length = separator_length;
        self
    }

    pub fn with_flexible_width(mut self, flexible_width: bool) -> Self {
        self.flexible_width = flexible_width;
        self
    }
}

impl Default for FwfToDataFrame {
    fn default() -> Self {
        Self {
            widths: Vec::default(),
            has_header: true,
            separator_length: 0,
            flexible_width: true,
        }
    }
}

impl ReadToDataFrames for FwfToDataFrame {
    fn named_frames(&self, input: Source) -> AppResult<NamedFrames> {
        let file_content = match &input {
            Source::File(path) => read_to_string(path)?,
            Source::Stdin => {
                let mut buf = String::new();
                io::stdin().read_to_string(&mut buf)?;
                buf
            }
        };

        let widths = if self.widths.is_empty() {
            let common_space_indices = file_content
                .lines()
                .map(|line| line.trim())
                .filter(|line| !line.is_empty())
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
            self.widths.clone()
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
            .map(|rec| {
                rec.iter().fold(Vec::new(), |mut vec, slice| {
                    if let Some(name) = slice.snake_case_names().find(|name| !vec.contains(name)) {
                        vec.push(name);
                    } else {
                        panic!("Not implemented")
                    }
                    vec
                })
            })
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

        let df: DataFrame = header
            .into_iter()
            .zip(columns)
            .map(|(name, vals)| Series::new(name.into(), vals))
            .collect();

        Ok([(input.table_name(), df)].into())
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
