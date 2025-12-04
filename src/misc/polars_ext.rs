use std::{
    ops::{Add, Div},
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
};

use anyhow::anyhow;
use itertools::{Itertools, izip};
use polars::{
    frame::DataFrame,
    prelude::{AnyValue, ChunkAgg, DataType, NamedFrom, SeriesMethods},
    series::Series,
};
use ratatui::widgets::Cell;
use unicode_width::UnicodeWidthStr;

use crate::{AppResult, misc::jagged_vec::JaggedVec, tui::sheet::SheetSection};

use super::type_ext::HasSubsequence;

pub trait AnyValueExt {
    fn into_single_line(self) -> String;
    fn into_multi_line(self) -> String;
    fn into_cell(self, width: usize) -> Cell<'static>;
}

pub trait TuiWidths {
    fn tui_widths(&self) -> Vec<usize>;
}

pub trait FuzzyCmp {
    fn fuzzy_cmp(self, other: &str) -> bool;
}

pub trait GetSheetSections {
    fn get_sheet_sections(&self, pos: usize) -> Vec<SheetSection>;
}

pub trait TryMapAll {
    fn try_map_all(
        &self,
        f: impl Fn(AnyValue) -> Option<AnyValue<'static>> + Sync + Send + 'static,
    ) -> Option<Series>;
}

pub trait PlotData {
    fn scatter_plot_data(&self, x_label: &str, y_label: &str) -> AppResult<JaggedVec<(f64, f64)>>;
    fn scatter_plot_data_grouped(
        &self,
        x_label: &str,
        y_label: &str,
        group_by: &str,
    ) -> AppResult<(JaggedVec<(f64, f64)>, Vec<String>)>;
    fn histogram_plot_data(&self, col: &str, buckets: usize) -> AppResult<Vec<(String, u64)>>;
}

impl AnyValueExt for AnyValue<'_> {
    fn into_single_line(self) -> String {
        match self {
            AnyValue::Null => "".to_owned(),
            AnyValue::StringOwned(v) => v.to_string(),
            AnyValue::String(v) => v.to_string(),
            AnyValue::Categorical(idx, rev_map) => {
                rev_map.cat_to_str(idx).unwrap_or_default().to_owned()
            }
            AnyValue::CategoricalOwned(idx, rev_map) => {
                rev_map.cat_to_str(idx).unwrap_or_default().to_owned()
            }
            AnyValue::Binary(buf) => format!("Blob (Length: {})", buf.len()),
            AnyValue::BinaryOwned(buf) => format!("Blob (Length: {})", buf.len()),
            _ => self.to_string(),
        }
    }

    fn into_multi_line(self) -> String {
        match self {
            AnyValue::Null => "".to_owned(),
            AnyValue::StringOwned(v) => v.to_string(),
            AnyValue::String(v) => v.to_string(),
            AnyValue::Categorical(idx, rev_map) => {
                rev_map.cat_to_str(idx).unwrap_or_default().to_owned()
            }
            AnyValue::CategoricalOwned(idx, rev_map) => {
                rev_map.cat_to_str(idx).unwrap_or_default().to_owned()
            }
            AnyValue::Binary(buf) => bytes_to_string(buf),
            AnyValue::BinaryOwned(buf) => bytes_to_string(buf),
            _ => self.to_string(),
        }
    }

    fn into_cell(self, width: usize) -> Cell<'static> {
        match self {
            AnyValue::Float32(f) => Cell::new(format!("{f:>w$.2}", w = width)),
            AnyValue::Float64(f) => Cell::new(format!("{f:>w$.2}", w = width)),
            _ => Cell::new(self.into_single_line()),
        }
    }
}

fn bytes_to_string(buf: impl AsRef<[u8]>) -> String {
    let buf = buf.as_ref();
    let index_width = buf.len().div(16).to_string().len();
    let index_width = if index_width % 2 == 0 {
        index_width
    } else {
        index_width + 1
    };
    format!(
        "Blob (Length: {})\n{}",
        buf.len(),
        buf.iter()
            .map(|b| format!("{b:02X}"))
            .chunks(8)
            .into_iter()
            .map(|mut chunk| chunk.join(" "))
            .chunks(2)
            .into_iter()
            .enumerate()
            .map(|(idx, mut chunk)| format!("{:0index_width$}:  {}", idx, chunk.join("   ")))
            .join("\n")
    )
}

impl TuiWidths for DataFrame {
    fn tui_widths(&self) -> Vec<usize> {
        self.iter().map(series_width).collect()
    }
}

fn series_width(series: &Series) -> usize {
    series
        .iter()
        .map(|any_value| {
            any_value
                .into_multi_line()
                .lines()
                .next()
                .map(|s| s.width())
                .unwrap_or(0)
        })
        .max()
        .unwrap_or_default()
        .max(series.name().as_str().width())
}

impl FuzzyCmp for AnyValue<'_> {
    fn fuzzy_cmp(self, other: &str) -> bool {
        match self {
            AnyValue::Null => false,
            AnyValue::StringOwned(pl_small_str) => pl_small_str.has_subsequence(other),
            AnyValue::String(val) => val.has_subsequence(other),
            _ => self.into_multi_line().has_subsequence(other),
        }
    }
}

impl GetSheetSections for DataFrame {
    fn get_sheet_sections(&self, pos: usize) -> Vec<SheetSection> {
        izip!(
            self.get_column_names().into_iter(),
            self.get(pos)
                .unwrap_or_default()
                .into_iter()
                .map(AnyValueExt::into_multi_line),
            self.dtypes()
        )
        .map(|(header, content, dtype)| SheetSection::new(format!("{header} ({dtype})"), content))
        .collect_vec()
    }
}

impl TryMapAll for Series {
    fn try_map_all(
        &self,
        cast: impl Fn(AnyValue) -> Option<AnyValue<'static>> + Sync + Send + 'static,
    ) -> Option<Series> {
        let break_out = Arc::new(AtomicBool::new(false));
        let mut new = vec![AnyValue::Null; self.len()];
        std::thread::scope(|scope| {
            let piece_len = if self.len() > num_cpus::get() {
                self.len() / num_cpus::get()
            } else {
                1
            };
            for (idx, new_chunk) in new.chunks_mut(piece_len).enumerate() {
                let offset = (idx * piece_len) as i64;
                let break_out = break_out.clone();
                let cast = &cast;
                scope.spawn(move || {
                    let series = self.slice(offset, piece_len);
                    for (new_val, val) in new_chunk.iter_mut().zip(series.iter()) {
                        if let Some(parsed) = cast(val) {
                            *new_val = parsed;
                        } else {
                            break_out.store(true, Ordering::Relaxed);
                            break;
                        }
                        if break_out.load(Ordering::Relaxed) {
                            break;
                        }
                    }
                });
            }
        });
        (!break_out.load(Ordering::Relaxed)).then_some(Series::new(self.name().to_owned(), new))
    }
}

impl PlotData for DataFrame {
    fn scatter_plot_data(&self, x_label: &str, y_label: &str) -> AppResult<JaggedVec<(f64, f64)>> {
        Ok(self
            .column(x_label)?
            .cast(&DataType::Float64)?
            .f64()?
            .iter()
            .zip(
                self.column(y_label)?
                    .cast(&DataType::Float64)?
                    .f64()?
                    .iter(),
            )
            .filter_map(|(x, y)| Some((x?, y?)))
            .collect())
    }

    fn scatter_plot_data_grouped(
        &self,
        x_label: &str,
        y_label: &str,
        group_by: &str,
    ) -> AppResult<(JaggedVec<(f64, f64)>, Vec<String>)> {
        let mut groups = Vec::new();
        let mut data = JaggedVec::new();
        for (name, df) in self
            .partition_by(vec![group_by], true)?
            .into_iter()
            .map(|df| {
                let name = df
                    .column(group_by)
                    .and_then(|column| column.get(0))
                    .map(AnyValueExt::into_single_line)
                    .unwrap_or("null".to_owned());
                (name, df)
            })
            .sorted_by(|(a, _), (b, _)| a.cmp(b))
        {
            groups.push(name);
            data.push(df.scatter_plot_data(x_label, y_label)?);
        }
        Ok((data, groups))
    }

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
            | DataType::Int128 => {
                let counts = col.as_materialized_series().value_counts(
                    true,
                    true,
                    format!("{col_name}_count").into(),
                    false,
                )?;
                if counts.height() <= buckets {
                    discrete_histogram(counts)
                } else {
                    continues_histogram(counts, buckets)
                }
            }
            DataType::Float32 | DataType::Float64 | DataType::Decimal(_, _) => continues_histogram(
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

fn discrete_histogram(counts: DataFrame) -> AppResult<Vec<(String, u64)>> {
    Ok(counts[0]
        .as_materialized_series()
        .iter()
        .map(AnyValue::into_single_line)
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
