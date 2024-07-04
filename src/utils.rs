use std::collections::HashMap;

use polars::{
    datatypes::{AnyValue, DataType},
    frame::DataFrame,
    series::Series,
};

pub struct ZipIters<Iter> {
    iterators: Vec<Iter>,
}

impl<Iter, T> Iterator for ZipIters<Iter>
where
    Iter: Iterator<Item = T>,
    T: Clone + Default,
{
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut items = Vec::new();
        let mut any_valid = false;

        for iter in self.iterators.iter_mut() {
            if let Some(item) = iter.next() {
                items.push(item);
                any_valid = true;
            } else {
                items.push(T::default()); // Using default to fill gaps
            }
        }

        if any_valid {
            Some(items)
        } else {
            None // If no valid items, all iterators are exhausted
        }
    }
}

pub fn zip_iters<I1: IntoIterator<Item = I2>, I2: Iterator<Item = T>, T: Clone + Default>(
    iter: I1,
) -> impl Iterator<Item = Vec<T>> {
    ZipIters {
        iterators: iter.into_iter().collect(),
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Scroll(usize);

impl From<Scroll> for usize {
    fn from(val: Scroll) -> Self {
        val.0
    }
}

impl From<Scroll> for u16 {
    fn from(val: Scroll) -> Self {
        val.0 as u16
    }
}

impl Scroll {
    pub fn up(&mut self) {
        self.0 = self.0.saturating_sub(1);
    }

    pub fn down(&mut self) {
        self.0 = self.0.saturating_add(1);
    }

    pub fn adjust(&mut self, lines: usize, space: usize) {
        self.0 = self.0.min(lines.saturating_sub(space))
    }
}

pub fn line_count(text: &str, width: usize) -> usize {
    let mut line_count = 1;
    let mut used_space = 0;
    for word_len in text.split(' ').map(str::len) {
        if word_len <= width {
            if used_space + word_len <= width {
                used_space += word_len + 1;
            } else {
                used_space = word_len + 1;
                line_count += 1;
            }
        } else {
            line_count += (word_len - width + used_space).div_ceil(width)
        }
    }
    line_count
}

pub fn data_frame_widths(df: &polars::frame::DataFrame) -> Vec<usize> {
    df.get_column_names()
        .into_iter()
        .zip(df.get_columns())
        .map(|(col, series)| col.len().max(series_width(series)))
        .collect::<Vec<_>>()
}

pub fn series_width(series: &Series) -> usize {
    series
        .iter()
        .map(|any_value| any_value_into_string(any_value).len())
        .max()
        .unwrap_or_default()
}

pub fn any_value_into_string(value: polars::datatypes::AnyValue) -> String {
    match value {
        AnyValue::Null => "".to_owned(),
        AnyValue::Boolean(v) => format!("{}", v),
        AnyValue::String(v) => v.to_string(),
        AnyValue::UInt8(v) => format!("{}", v),
        AnyValue::UInt16(v) => format!("{}", v),
        AnyValue::UInt32(v) => format!("{}", v),
        AnyValue::UInt64(v) => format!("{}", v),
        AnyValue::Int8(v) => format!("{}", v),
        AnyValue::Int16(v) => format!("{}", v),
        AnyValue::Int32(v) => format!("{}", v),
        AnyValue::Int64(v) => format!("{}", v),
        AnyValue::Float32(v) => format!("{}", v),
        AnyValue::Float64(v) => format!("{}", v),
        AnyValue::Decimal(v1, v2) => format!("{}.{}", v1, v2),
        AnyValue::Date(v) => format!("{}", v),
        AnyValue::Datetime(v1, v2, v3) => format!("{} {} {:?}", v1, v2, v3),
        AnyValue::Duration(v1, v2) => format!("{} {}", v1, v2),
        AnyValue::Time(v) => format!("{}", v),
        AnyValue::List(_) => value.to_string(),
        AnyValue::StringOwned(v) => v.to_string(),
        AnyValue::Binary(_) => value.to_string(),
        AnyValue::BinaryOwned(_) => value.to_string(),
        AnyValue::Struct(_, _, _) => value.to_string(),
        AnyValue::StructOwned(_) => value.to_string(),
    }
}

pub fn infer_schema_safe(data_frame: &mut DataFrame) {
    let dtypes = [
        DataType::Int64,
        DataType::Float64,
        DataType::Boolean,
        DataType::Date,
        DataType::Time,
    ];
    data_frame
        .get_column_names()
        .into_iter()
        .map(|col_name| (col_name, data_frame.column(col_name).unwrap()))
        .filter_map(|(col_name, series)| {
            dtypes
                .iter()
                .filter_map(|dtype| series.cast(dtype).ok())
                .find(|series| series.null_count() != series.len())
                .map(|series| (col_name.to_owned(), series))
        })
        .collect::<HashMap<String, Series>>()
        .into_iter()
        .for_each(|(col_name, series)| {
            data_frame.replace(col_name.as_str(), series).unwrap();
        });
}

#[cfg(test)]
mod tests {
    use super::*;
    use polars::prelude::*;

    #[test]
    fn test_zip_iters_all_same_length() {
        let iter1 = vec![1, 2, 3].into_iter();
        let iter2 = vec![4, 5, 6].into_iter();
        let iter3 = vec![7, 8, 9].into_iter();

        let mut zipped = zip_iters(vec![iter1, iter2, iter3]);

        assert_eq!(zipped.next(), Some(vec![1, 4, 7]));
        assert_eq!(zipped.next(), Some(vec![2, 5, 8]));
        assert_eq!(zipped.next(), Some(vec![3, 6, 9]));
        assert_eq!(zipped.next(), None);
    }

    #[test]
    fn test_zip_iters_different_lengths() {
        let iter1 = vec![1, 2].into_iter();
        let iter2 = vec![4, 5, 6].into_iter();
        let iter3 = vec![7].into_iter();

        let mut zipped = zip_iters(vec![iter1, iter2, iter3]);

        assert_eq!(zipped.next(), Some(vec![1, 4, 7]));
        assert_eq!(zipped.next(), Some(vec![2, 5, Default::default()]));
        assert_eq!(
            zipped.next(),
            Some(vec![Default::default(), 6, Default::default()])
        );
        assert_eq!(zipped.next(), None);
    }

    #[test]
    fn test_zip_iters_empty_iterator() {
        let iter1 = vec![].into_iter();
        let iter2 = vec![4, 5, 6].into_iter();
        let iter3 = vec![].into_iter();

        let mut zipped = zip_iters(vec![iter1, iter2, iter3]);

        assert_eq!(
            zipped.next(),
            Some(vec![Default::default(), 4, Default::default()])
        );
        assert_eq!(
            zipped.next(),
            Some(vec![Default::default(), 5, Default::default()])
        );
        assert_eq!(
            zipped.next(),
            Some(vec![Default::default(), 6, Default::default()])
        );
        assert_eq!(zipped.next(), None);
    }

    #[test]
    fn test_zip_iters_single_iterator() {
        let iter1 = vec![1, 2, 3].into_iter();

        let mut zipped = zip_iters(vec![iter1]);

        assert_eq!(zipped.next(), Some(vec![1]));
        assert_eq!(zipped.next(), Some(vec![2]));
        assert_eq!(zipped.next(), Some(vec![3]));
        assert_eq!(zipped.next(), None);
    }

    #[test]
    fn test_zip_iters_default_value() {
        #[derive(Clone, Default, PartialEq, Debug)]
        struct CustomType(i32);

        let iter1 = vec![CustomType(1), CustomType(2)].into_iter();
        let iter2 = vec![CustomType(4), CustomType(5), CustomType(6)].into_iter();
        let iter3 = vec![CustomType(7)].into_iter();

        let mut zipped = zip_iters(vec![iter1, iter2, iter3]);

        assert_eq!(
            zipped.next(),
            Some(vec![CustomType(1), CustomType(4), CustomType(7)])
        );
        assert_eq!(
            zipped.next(),
            Some(vec![CustomType(2), CustomType(5), CustomType::default()])
        );
        assert_eq!(
            zipped.next(),
            Some(vec![
                CustomType::default(),
                CustomType(6),
                CustomType::default()
            ])
        );
        assert_eq!(zipped.next(), None);
    }

    #[test]
    fn test_line_count_single_line() {
        let text = "Hello world";
        assert_eq!(line_count(text, 15), 1);
        assert_eq!(line_count(text, 11), 1);
        assert_eq!(line_count(text, 10), 2);
    }

    #[test]
    fn test_line_count_multiple_lines() {
        let text = "Hello world this is a test";
        assert_eq!(line_count(text, 15), 2);
        assert_eq!(line_count(text, 10), 3);
        assert_eq!(line_count(text, 5), 5);
    }

    #[test]
    fn test_line_count_exact_width() {
        let text = "Hello world";
        assert_eq!(line_count(text, 5), 2);
        assert_eq!(line_count(text, 6), 2);
        assert_eq!(line_count(text, 11), 1);
    }

    #[test]
    fn test_line_count_with_long_word() {
        let text = "supercalifragilisticexpialidocious";
        assert_eq!(line_count(text, 10), 4);
        assert_eq!(line_count(text, 20), 2);
        assert_eq!(line_count(text, 30), 2);
    }

    #[test]
    fn test_line_count_with_mixed_length_words() {
        let text = "a bb ccc dddd eeeee ffffff ggggggg";
        assert_eq!(line_count(text, 10), 4);
        assert_eq!(line_count(text, 5), 8);
        assert_eq!(line_count(text, 20), 2);
    }

    #[test]
    fn test_line_count_empty_string() {
        let text = "";
        assert_eq!(line_count(text, 10), 1);
    }

    #[test]
    fn test_infer_schema_safe_basic() {
        let mut df = df! {
            "integers"=> ["1", "2", "3", "4"],
            "floats"=> ["1.1", "2.2", "3.3", "4.4"],
            "dates"=> [ "2022-1-1", "2022-1-2", "2022-1-3", "2022-1-4" ],
            "strings"=> ["a", "b", "c", "d"],
        }
        .unwrap();
        infer_schema_safe(&mut df);

        assert_eq!(df.column("integers").unwrap().dtype(), &DataType::Int64);
        assert_eq!(df.column("floats").unwrap().dtype(), &DataType::Float64);
        assert_eq!(df.column("dates").unwrap().dtype(), &DataType::Date);
        assert_eq!(df.column("strings").unwrap().dtype(), &DataType::String);
    }
}
