use itertools::Itertools;
use polars::{
    frame::DataFrame,
    prelude::{AnyValue, DataType},
    series::{ChunkCompareEq, Series},
};

use super::type_ext::HasSubsequence;

pub trait SafeInferSchema {
    fn safe_infer_schema(&mut self);
}

pub trait IntoString {
    fn into_string(self) -> String;
}

pub trait TuiWidths {
    fn tui_widths(&self) -> Vec<usize>;
}

pub trait FuzzyCmp {
    fn fuzzy_cmp(self, other: &str) -> bool;
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
    [
        DataType::Int64,
        DataType::Float64,
        DataType::Boolean,
        DataType::Date,
        DataType::Time,
    ]
    .iter()
    .filter_map(|dtype| series.cast(dtype).ok())
    .find(|dtype_series| series.is_null().equal(&dtype_series.is_null()).all())
}

impl IntoString for AnyValue<'_> {
    fn into_string(self) -> String {
        match self {
            AnyValue::Null => "".to_owned(),
            AnyValue::StringOwned(v) => v.to_string(),
            AnyValue::String(v) => v.to_string(),
            _ => self.to_string(),
        }
    }
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
                .into_string()
                .lines()
                .next()
                .map(str::len)
                .unwrap_or(0)
        })
        .max()
        .unwrap_or_default()
        .max(series.name().len())
}

impl<'a> FuzzyCmp for AnyValue<'a> {
    fn fuzzy_cmp(self, other: &str) -> bool {
        match self {
            AnyValue::Null => false,
            AnyValue::Boolean(val) => {
                if val {
                    ["T", "Tr", "Tru", "True", "t", "tr", "tru", "true"].contains(&other)
                } else {
                    [
                        "F", "Fa", "Fal", "Fals", "False", "f", "fa", "fal", "fals", "false",
                    ]
                    .contains(&other)
                }
            }
            AnyValue::StringOwned(pl_small_str) => pl_small_str.has_subsequence(other, other.len()),
            AnyValue::String(val) => val.has_subsequence(other, other.len()),
            AnyValue::UInt8(_)|
            AnyValue::UInt16(_)|
            AnyValue::UInt32(_)|
            AnyValue::UInt64(_)|
            AnyValue::Int8(_)|
            AnyValue::Int16(_)|
            AnyValue::Int32(_)|
            AnyValue::Int64(_)|
            AnyValue::Float32(_)|
            AnyValue::Float64(_)|
            AnyValue::Date(_)|
            AnyValue::Decimal(_, _)|
            AnyValue::Datetime(_, _, _)|
            AnyValue::DatetimeOwned(_, _, _)|
            AnyValue::Duration(_, _)|
            AnyValue::Time(_) => self.into_string().has_subsequence(other, other.len()),

            AnyValue::List(_) => false,
            AnyValue::Struct(_, _, _) => false,
            AnyValue::Binary(_) => false,
            AnyValue::BinaryOwned(_) => false,
            AnyValue::StructOwned(_) => false,
        }
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
            "dates"=> [ "2022-1-1", "2022-1-2", "2022-1-3", "2022-1-4" ],
            "strings"=> ["a", "b", "c", "d"],
        }
        .unwrap();
        df.safe_infer_schema();

        assert_eq!(df.column("integers").unwrap().dtype(), &DataType::Int64);
        assert_eq!(df.column("floats").unwrap().dtype(), &DataType::Float64);
        assert_eq!(df.column("dates").unwrap().dtype(), &DataType::Date);
        assert_eq!(df.column("strings").unwrap().dtype(), &DataType::String);
    }
}
