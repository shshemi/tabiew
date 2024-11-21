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

#[cfg(test)]
mod tests {
    use super::*;

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
}
