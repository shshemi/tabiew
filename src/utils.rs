use std::collections::{HashMap, VecDeque};

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
                .find(|dtype_series| series.is_null().equal(&dtype_series.is_null()).all())
                .map(|series| (col_name.to_owned(), series))
        })
        .collect::<HashMap<String, Series>>()
        .into_iter()
        .for_each(|(col_name, series)| {
            data_frame.replace(col_name.as_str(), series).unwrap();
        });
}

#[inline]
pub fn as_ascii(c: char) -> Option<u8> {
    c.is_ascii().then_some(c as u8)
}

#[cfg(test)]
mod tests {
    use super::*;
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
