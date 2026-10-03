use std::ops::Add;

use anyhow::anyhow;
use itertools::Itertools;
use polars::{
    frame::DataFrame,
    prelude::{ChunkAgg, DataType, SeriesMethods},
};

use crate::{AppResult, misc::config::config, tui::misc::any_value_formatter::AnyValueFormatter};

pub trait DataFrameExt {
    fn histogram_plot_data(&self, col: &str, buckets: usize) -> AppResult<Vec<(String, u64)>>;
}

impl DataFrameExt for DataFrame {
    fn histogram_plot_data(&self, col_name: &str, buckets: usize) -> AppResult<Vec<(String, u64)>> {
        let col = self.column(col_name)?;
        match col.dtype() {
            DataType::UInt8
            | DataType::UInt16
            | DataType::UInt32
            | DataType::UInt64
            | DataType::Int8
            | DataType::Int16
            | DataType::Int32
            | DataType::Int64
            | DataType::Int128
            | DataType::Float32
            | DataType::Float64
            | DataType::Decimal(_, _) => continues_histogram(
                col.as_materialized_series()
                    .value_counts(true, true, "value".into(), false)?,
                buckets,
            ),
            DataType::Boolean | DataType::String => discrete_histogram(
                col.as_materialized_series()
                    .value_counts(true, true, "value".into(), false)?,
            ),
            _ => Err(anyhow!("Unsupported column type"))?,
        }
    }
}

fn discrete_histogram(mut counts: DataFrame) -> AppResult<Vec<(String, u64)>> {
    let fp_precision = config().fp_precision();
    counts.rechunk_mut();
    Ok(counts[0]
        .as_materialized_series()
        .iter()
        .map(|val| {
            AnyValueFormatter::new(fp_precision)
                .into_single_line(val)
                .into_owned()
        })
        .zip(counts[1].as_materialized_series().u32()?.iter())
        .map(|(v, c)| (v, c.unwrap_or_default() as u64))
        .collect_vec())
}

fn continues_histogram(counts: DataFrame, buckets: usize) -> AppResult<Vec<(String, u64)>> {
    let casted = counts[0].cast(&DataType::Float64)?;
    let arr = casted.f64()?;
    let (min, max) = arr.min_max().ok_or(anyhow!("No value found"))?;
    let width = (max - min) / (buckets as f64);
    let counts = arr
        .iter()
        .flatten()
        .zip(counts[1].as_materialized_series().u32()?.iter().flatten())
        .fold(vec![0; buckets], |mut buckets, (v, c)| {
            let idx = (((v - min) / width) as usize).min(buckets.len().saturating_sub(1));
            buckets[idx] += c;
            buckets
        });
    let label_len = format!("{max:.2}").len();
    Ok(counts
        .into_iter()
        .enumerate()
        .map(|(idx, r)| {
            let start = (idx as f64) * width + min;
            let end = (idx.add(1) as f64) * width + min;
            (
                format!(" {start:>w$.2} - {end:>w$.2}", w = label_len),
                r as u64,
            )
        })
        .collect())
}
