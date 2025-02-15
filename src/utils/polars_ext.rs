use itertools::Itertools;
use polars::{
    frame::DataFrame,
    prelude::{AnyValue, DataType},
    series::{ChunkCompareEq, Series},
};
use unicode_width::UnicodeWidthStr;

use crate::tui::sheet::SheetSection;

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
            AnyValue::Categorical(idx, rev_map, _) => rev_map.get(idx).to_owned(),
            AnyValue::CategoricalOwned(idx, rev_map, _) => rev_map.get(idx).to_owned(),
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
            _ => self.into_string().has_subsequence(other, other.len()),
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
                    .map(IntoString::into_string)
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
