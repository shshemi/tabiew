use std::{
    ops::Div,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
};

use chrono::NaiveDateTime;
use itertools::Itertools;
use polars::{
    frame::DataFrame,
    prelude::{AnyValue, DataType, NamedFrom, TimeUnit},
    series::{ChunkCompareEq, Series},
};
use unicode_width::UnicodeWidthStr;

use crate::tui::sheet::SheetSection;

use super::type_ext::HasSubsequence;

pub trait SafeInferSchema {
    fn safe_infer_schema(&mut self);
}

pub trait IntoString {
    fn into_single_line(self) -> String;
    fn into_multi_line(self) -> String;
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

impl SafeInferSchema for DataFrame {
    fn safe_infer_schema(&mut self) {
        self.iter()
            .filter_map(type_infered_series)
            .map(|series| (series.name().to_owned(), series))
            .collect_vec()
            .into_iter()
            .for_each(|(name, series)| {
                self.replace(name.as_str(), series).unwrap();
            });
    }
}

fn type_infered_series(series: &Series) -> Option<Series> {
    [cast_int64, cast_float64, cast_boolean, cast_datetime]
        .iter()
        .find_map(|func_map| func_map(series))
}

fn cast_int64(series: &Series) -> Option<Series> {
    cast_custom(series, |value| match value {
        AnyValue::Null => Some(AnyValue::Null),
        AnyValue::String(slice) => slice.parse::<i64>().ok().map(AnyValue::Int64),
        AnyValue::StringOwned(pl_small_str) => {
            pl_small_str.parse::<i64>().ok().map(AnyValue::Int64)
        }
        AnyValue::Int8(i) => Some(AnyValue::Int64(i.into())),
        AnyValue::Int16(i) => Some(AnyValue::Int64(i.into())),
        AnyValue::Int32(i) => Some(AnyValue::Int64(i.into())),
        AnyValue::Int64(i) => Some(AnyValue::Int64(i)),
        _ => None,
    })
}

fn cast_float64(series: &Series) -> Option<Series> {
    cast_custom(series, |value| match value {
        AnyValue::Null => Some(AnyValue::Null),
        AnyValue::String(slice) => slice.parse::<f64>().ok().map(AnyValue::Float64),
        AnyValue::StringOwned(pl_small_str) => {
            pl_small_str.parse::<f64>().ok().map(AnyValue::Float64)
        }
        AnyValue::Float32(f) => Some(AnyValue::Float64(f.into())),
        AnyValue::Float64(f) => Some(AnyValue::Float64(f)),
        _ => None,
    })
}

fn cast_boolean(series: &Series) -> Option<Series> {
    cast_custom(series, |value| match value {
        AnyValue::Null => Some(AnyValue::Null),
        AnyValue::String(slice) => match slice {
            "true" => Some(AnyValue::Boolean(true)),
            "false" => Some(AnyValue::Boolean(false)),
            _ => None,
        },
        AnyValue::StringOwned(pl_small_str) => match pl_small_str.as_str() {
            "true" => Some(AnyValue::Boolean(true)),
            "false" => Some(AnyValue::Boolean(false)),
            _ => None,
        },
        AnyValue::Boolean(b) => Some(AnyValue::Boolean(b)),
        _ => None,
    })
}

fn cast_datetime(series: &Series) -> Option<Series> {
    cast_datetime_polars(series).or_else(|| {
        [
            "%Y-%m-%d %H:%M:%S",
            "%Y-%m-%dT%H:%M:%S",
            "%Y-%m-%dT%H:%M:%S%.f",
            "%Y%m%dT%H%M%S",
            "%Y-%m-%d",
            "%Y %m %d",
            "%Y.%m.%d",
            "%Y%m%d",
            "%Y-%j",
        ]
        .into_iter()
        .find_map(|fmt| cast_datetime_custom(series, fmt))
    })
}

fn cast_datetime_polars(series: &Series) -> Option<Series> {
    let new_series = series
        .cast(&DataType::Datetime(TimeUnit::Milliseconds, None))
        .ok()?;
    series
        .is_null()
        .equal(&new_series.is_null())
        .all()
        .then_some(new_series)
}

fn cast_datetime_custom(series: &Series, fmt: &'static str) -> Option<Series> {
    cast_custom(series, |val| match val {
        AnyValue::String(s) => parse_datetime(s, fmt),
        AnyValue::StringOwned(s) => parse_datetime(s.as_str(), fmt),
        AnyValue::Datetime(ts, unit, zone) => Some(AnyValue::DatetimeOwned(
            ts,
            unit,
            zone.map(|sm| sm.to_owned().into()),
        )),
        AnyValue::DatetimeOwned(ts, unit, zone) => Some(AnyValue::DatetimeOwned(ts, unit, zone)),
        AnyValue::Null => Some(AnyValue::Null),
        _ => None,
    })
}

fn parse_datetime(slice: &str, fmt: &str) -> Option<AnyValue<'static>> {
    NaiveDateTime::parse_from_str(slice, fmt)
        .map(|date| {
            AnyValue::DatetimeOwned(
                date.and_utc().timestamp_millis(),
                TimeUnit::Milliseconds,
                None,
            )
        })
        .ok()
}

fn cast_custom(
    series: &Series,
    cast: impl Fn(AnyValue) -> Option<AnyValue<'static>> + Sync + Send + 'static,
) -> Option<Series> {
    let break_out = Arc::new(AtomicBool::new(false));
    let mut new = vec![AnyValue::Null; series.len()];
    std::thread::scope(|scope| {
        let piece_len = if series.len() > num_cpus::get() {
            series.len() / num_cpus::get()
        } else {
            series.len()
        };
        for (idx, new_chunk) in new.chunks_mut(piece_len).enumerate() {
            let offset = (idx * piece_len) as i64;
            let break_out = break_out.clone();
            let cast = &cast;
            scope.spawn(move || {
                let series = series.slice(offset, piece_len);
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
    (!break_out.load(Ordering::Relaxed)).then_some(Series::new(series.name().to_owned(), new))
}

impl IntoString for AnyValue<'_> {
    fn into_single_line(self) -> String {
        match self {
            AnyValue::Null => "".to_owned(),
            AnyValue::StringOwned(v) => v.to_string(),
            AnyValue::String(v) => v.to_string(),
            AnyValue::Categorical(idx, rev_map, _) => rev_map.get(idx).to_owned(),
            AnyValue::CategoricalOwned(idx, rev_map, _) => rev_map.get(idx).to_owned(),
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
            AnyValue::Categorical(idx, rev_map, _) => rev_map.get(idx).to_owned(),
            AnyValue::CategoricalOwned(idx, rev_map, _) => rev_map.get(idx).to_owned(),
            AnyValue::Binary(buf) => bytes_to_string(buf),
            AnyValue::BinaryOwned(buf) => bytes_to_string(buf),
            _ => self.to_string(),
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
            .map(|b| format!("{:02X}", b))
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
            AnyValue::StringOwned(pl_small_str) => pl_small_str.has_subsequence(other, other.len()),
            AnyValue::String(val) => val.has_subsequence(other, other.len()),
            _ => self.into_multi_line().has_subsequence(other, other.len()),
        }
    }
}

impl GetSheetSections for DataFrame {
    fn get_sheet_sections(&self, pos: usize) -> Vec<SheetSection> {
        self.get_column_names()
            .into_iter()
            .map(|pl_str| pl_str.to_string().to_owned())
            .zip(
                self.get(pos)
                    .unwrap_or_default()
                    .into_iter()
                    .map(IntoString::into_multi_line)
                    .collect_vec(),
            )
            .map(|(header, content)| SheetSection::new(header, content))
            .collect_vec()
    }
}

#[cfg(test)]
mod tests {
    use polars::df;

    use super::*;

    #[test]
    fn test_infer_schema_safe_basic() {
        let mut df = df! {
            "integers"=> ["1", "2", "3", "4"],
            "floats"=> ["1.1", "2.2", "3.3", "4.4"],
            "datetimes"=> [ "2022-1-1 00:00:00", "2022-1-2 00:00:00", "2022-1-3 00:00:00", "2022-1-4 00:00:00" ],
            "strings"=> ["a", "b", "c", "d"],
        }
        .unwrap();
        df.safe_infer_schema();

        assert_eq!(df.column("integers").unwrap().dtype(), &DataType::Int64);
        assert_eq!(df.column("floats").unwrap().dtype(), &DataType::Float64);
        assert!(matches!(
            df.column("datetimes").unwrap().dtype(),
            &DataType::Datetime(_, _)
        ));
        assert_eq!(df.column("strings").unwrap().dtype(), &DataType::String);
    }

    #[test]
    fn test_cast_float64_valid_strings() {
        let series = Series::new("floats".into(), &["1.1", "2.2", "3.3", "4.4"]);
        let result = cast_float64(&series).unwrap();
        assert_eq!(result.dtype(), &DataType::Float64);
        assert_eq!(result.get(0).unwrap(), AnyValue::Float64(1.1));
        assert_eq!(result.get(1).unwrap(), AnyValue::Float64(2.2));
        assert_eq!(result.get(2).unwrap(), AnyValue::Float64(3.3));
        assert_eq!(result.get(3).unwrap(), AnyValue::Float64(4.4));
    }

    #[test]
    fn test_cast_float64_invalid_strings() {
        let series = Series::new("floats".into(), &["1.1", "invalid", "3.3", "4.4"]);
        let result = cast_float64(&series);
        assert!(result.is_none());
    }

    #[test]
    fn test_cast_int64_valid_strings() {
        let series = Series::new("integers".into(), &["1", "2", "3", "4"]);
        let result = cast_int64(&series).unwrap();
        assert_eq!(result.dtype(), &DataType::Int64);
        assert_eq!(result.get(0).unwrap(), AnyValue::Int64(1));
        assert_eq!(result.get(1).unwrap(), AnyValue::Int64(2));
        assert_eq!(result.get(2).unwrap(), AnyValue::Int64(3));
        assert_eq!(result.get(3).unwrap(), AnyValue::Int64(4));
    }

    #[test]
    fn test_cast_int64_invalid_strings() {
        let series = Series::new("integers".into(), &["1", "invalid", "3", "4"]);
        let result = cast_int64(&series);
        assert!(result.is_none());
    }

    #[test]
    fn test_cast_boolean_valid_strings() {
        let series = Series::new("booleans".into(), &["true", "false", "true", "false"]);
        let result = cast_boolean(&series).unwrap();
        assert_eq!(result.dtype(), &DataType::Boolean);
        assert_eq!(result.get(0).unwrap(), AnyValue::Boolean(true));
        assert_eq!(result.get(1).unwrap(), AnyValue::Boolean(false));
        assert_eq!(result.get(2).unwrap(), AnyValue::Boolean(true));
        assert_eq!(result.get(3).unwrap(), AnyValue::Boolean(false));
    }

    #[test]
    fn test_cast_boolean_invalid_strings() {
        let series = Series::new("booleans".into(), &["true", "invalid", "false", "true"]);
        let result = cast_boolean(&series);
        assert!(result.is_none());
    }

    #[test]
    fn test_cast_datetime_valid_strings() {
        let series = Series::new(
            "datetimes".into(),
            &[
                "2022-01-01 00:00:00",
                "2022-01-02 00:00:00",
                "2022-01-03 00:00:00",
                "2022-01-04 00:00:00",
            ],
        );
        let result = cast_datetime(&series).unwrap();
        assert_eq!(
            result.dtype(),
            &DataType::Datetime(TimeUnit::Milliseconds, None)
        );
        assert!(matches!(
            result.get(0).unwrap(),
            AnyValue::Datetime(_, _, _)
        ));
        assert!(matches!(
            result.get(1).unwrap(),
            AnyValue::Datetime(_, _, _)
        ));
        assert!(matches!(
            result.get(2).unwrap(),
            AnyValue::Datetime(_, _, _)
        ));
        assert!(matches!(
            result.get(3).unwrap(),
            AnyValue::Datetime(_, _, _)
        ));
    }

    #[test]
    fn test_cast_datetime_invalid_strings() {
        let series = Series::new(
            "dates".into(),
            &["2022-01-01", "invalid", "2022-01-03", "2022-01-04"],
        );
        let result = cast_datetime(&series);
        assert!(result.is_none());
    }
}
