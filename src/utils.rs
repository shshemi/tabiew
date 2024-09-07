use std::collections::VecDeque;

use polars::{
    datatypes::DataType,
    frame::DataFrame,
    series::{ChunkCompare, Series},
};

#[derive(Debug)]
pub struct RoundRobin<T> {
    queue: VecDeque<T>,
}

impl<T> RoundRobin<T> {
    fn new(iter: impl IntoIterator<Item = T>) -> Self {
        Self {
            queue: iter.into_iter().collect(),
        }
    }
}

impl<I, T> Iterator for RoundRobin<T>
where
    T: Iterator<Item = I>,
{
    type Item = I;

    fn next(&mut self) -> Option<Self::Item> {
        self.queue.pop_front().and_then(|mut iter| {
            let next = iter.next();
            self.queue.push_back(iter);
            next
        })
    }
}

pub trait RoundRobinExt {
    type Item;

    fn round_robin(self) -> RoundRobin<Self::Item>;
}

impl<I, T> RoundRobinExt for T
where
    T: IntoIterator<Item = I>,
{
    type Item = I;

    fn round_robin(self) -> RoundRobin<Self::Item> {
        RoundRobin::new(self)
    }
}

pub struct ZipIters<Iter> {
    iterators: Vec<Iter>,
}

impl<Iter, T> Iterator for ZipIters<Iter>
where
    Iter: Iterator<Item = T>,
    T: Default,
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

pub trait ZipItersExt<T> {
    fn zip_iters(self) -> ZipIters<T>;
}

impl<I1, I2, T> ZipItersExt<I2> for I1
where
    I1: IntoIterator<Item = I2>,
    I2: Iterator<Item = T>,
    T: Default,
{
    fn zip_iters(self) -> ZipIters<I2> {
        ZipIters {
            iterators: self.into_iter().collect(),
        }
    }
}

pub struct SplitByLength<'a, WidthIter> {
    slice: &'a str,
    width_iter: WidthIter,
    start: usize,
}

impl<'a, WI> Iterator for SplitByLength<'a, WI>
where
    WI: Iterator<Item = usize>,
{
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        self.width_iter.next().map(|width| {
            if let Some(end) = self.slice[self.start..]
                .char_indices()
                .nth(width)
                .map(|(offset, _)| self.start + offset)
            {
                let slice = &self.slice[self.start..end];
                self.start = end;
                slice
            } else {
                let slice = &self.slice[self.start..];
                self.start = self.slice.len();
                slice
            }
        })
    }
}

pub trait SplitByLengthExt<'a, WidthIter, IntoWidthIter> {
    fn split_by_length(&'a self, width: IntoWidthIter) -> SplitByLength<'a, WidthIter>;
}

impl<'a, WidthIter, IntoWidthIter> SplitByLengthExt<'a, WidthIter, IntoWidthIter> for str
where
    WidthIter: Iterator<Item = usize>,
    IntoWidthIter: IntoIterator<IntoIter = WidthIter>,
{
    fn split_by_length(&'a self, width: IntoWidthIter) -> SplitByLength<'a, WidthIter> {
        SplitByLength {
            slice: self,
            width_iter: width.into_iter(),
            start: 0,
        }
    }
}

pub fn safe_infer_schema(data_frame: &mut DataFrame) {
    for col_name in data_frame.get_column_names_owned() {
        let col_name = col_name.as_str();
        if let Some(series) = type_infered_series(data_frame.column(col_name).unwrap()) {
            data_frame.replace(col_name, series).unwrap();
        }
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

#[inline]
pub fn as_ascii(c: char) -> Option<u8> {
    c.is_ascii().then_some(c as u8)
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;
    use polars::prelude::*;

    #[test]
    fn test_round_robin() {
        let v1 = vec![1, 2, 3];
        let v2 = vec![4, 5];
        let v3 = vec![6, 7, 8, 9];

        let iter1 = v1.into_iter();
        let iter2 = v2.into_iter();
        let iter3 = v3.into_iter();

        let iterators = vec![iter1, iter2, iter3];
        let round_robin = iterators.round_robin();
        let result: Vec<_> = round_robin.collect();

        assert_eq!(result, vec![1, 4, 6, 2, 5, 7, 3]);
    }

    #[test]
    fn test_round_robin_complete() {
        let v1 = vec![1, 4, 7];
        let v2 = vec![2, 5, 8];
        let v3 = vec![3, 6, 9];

        let iter1 = v1.into_iter();
        let iter2 = v2.into_iter();
        let iter3 = v3.into_iter();

        let iterators = vec![iter1, iter2, iter3];
        let round_robin = iterators.round_robin();
        let result: Vec<_> = round_robin.collect();

        assert_eq!(result, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn test_zip_iters_all_same_length() {
        let iter1 = vec![1, 2, 3].into_iter();
        let iter2 = vec![4, 5, 6].into_iter();
        let iter3 = vec![7, 8, 9].into_iter();

        let mut zipped = vec![iter1, iter2, iter3].zip_iters();

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

        let mut zipped = vec![iter1, iter2, iter3].zip_iters();

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

        let mut zipped = vec![iter1, iter2, iter3].zip_iters();

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

        let mut zipped = vec![iter1].zip_iters();

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

        let mut zipped = vec![iter1, iter2, iter3].zip_iters();

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
    fn test_split_by_length() {
        let slice = "123ab9999";
        let c = slice.split_by_length([3, 2, 4, 1, 2]).collect_vec();
        assert_eq!(c[0], "123");
        assert_eq!(c[1], "ab");
        assert_eq!(c[2], "9999");
        assert_eq!(c[3], "");
        assert_eq!(c[4], "");
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
        safe_infer_schema(&mut df);

        assert_eq!(df.column("integers").unwrap().dtype(), &DataType::Int64);
        assert_eq!(df.column("floats").unwrap().dtype(), &DataType::Float64);
        assert_eq!(df.column("dates").unwrap().dtype(), &DataType::Date);
        assert_eq!(df.column("strings").unwrap().dtype(), &DataType::String);
    }
}
