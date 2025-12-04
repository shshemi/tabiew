#[derive(Debug)]
pub struct JaggedVec<T> {
    data: Vec<T>,
    splt: Vec<usize>,
}

impl<T> JaggedVec<T> {
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            splt: Vec::new(),
        }
    }

    pub fn push(&mut self, v: impl IntoIterator<Item = T>) {
        self.splt.push(self.data.len());
        self.data.extend(v);
    }

    pub fn get(&self, idx: usize) -> Option<&[T]> {
        let start = self.splt.get(idx).copied()?;
        let end = self
            .splt
            .get(idx.saturating_add(1))
            .copied()
            .unwrap_or(self.data.len());
        Some(&self.data[start..end])
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            nidx: 0,
            jvec: self,
        }
    }
}

impl<A> FromIterator<A> for JaggedVec<A> {
    fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self {
        let mut jv = JaggedVec::default();
        jv.push(iter.into_iter());
        jv
    }
}

impl<A> IntoIterator for JaggedVec<A> {
    type Item = A;

    type IntoIter = std::vec::IntoIter<A>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

#[derive(Debug)]
pub struct Iter<'a, T> {
    nidx: usize,
    jvec: &'a JaggedVec<T>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a [T];

    fn next(&mut self) -> Option<Self::Item> {
        self.jvec.get(self.nidx).inspect(|_| self.nidx += 1)
    }
}

impl<T> Default for JaggedVec<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_creates_empty_jagged_vec() {
        let jv: JaggedVec<i32> = JaggedVec::new();
        assert_eq!(jv.data.len(), 0);
        assert_eq!(jv.splt.len(), 0);
    }

    #[test]
    fn test_push_single_element() {
        let mut jv = JaggedVec::new();
        jv.push(vec![1]);
        assert_eq!(jv.get(0), Some(&[1][..]));
    }

    #[test]
    fn test_push_multiple_elements() {
        let mut jv = JaggedVec::new();
        jv.push(vec![1, 2, 3]);
        assert_eq!(jv.get(0), Some(&[1, 2, 3][..]));
    }

    #[test]
    fn test_push_empty_vec() {
        let mut jv = JaggedVec::new();
        jv.push(Vec::<i32>::new());
        assert_eq!(jv.get(0), Some(&[][..]));
    }

    #[test]
    fn test_push_multiple_vecs() {
        let mut jv = JaggedVec::new();
        jv.push(vec![1, 2]);
        jv.push(vec![3, 4, 5]);
        assert_eq!(jv.get(0), Some(&[1, 2][..]));
        assert_eq!(jv.get(1), Some(&[3, 4, 5][..]));
    }

    #[test]
    fn test_push_with_empty_vecs_interspersed() {
        let mut jv = JaggedVec::new();
        jv.push(vec![1]);
        jv.push(Vec::<i32>::new());
        jv.push(vec![2]);
        assert_eq!(jv.get(0), Some(&[1][..]));
        assert_eq!(jv.get(1), Some(&[][..]));
        assert_eq!(jv.get(2), Some(&[2][..]));
    }

    #[test]
    fn test_push_all_empty_vecs() {
        let mut jv = JaggedVec::new();
        jv.push(Vec::<i32>::new());
        jv.push(Vec::<i32>::new());
        jv.push(Vec::<i32>::new());
        assert_eq!(jv.get(0), Some(&[][..]));
        assert_eq!(jv.get(1), Some(&[][..]));
        assert_eq!(jv.get(2), Some(&[][..]));
    }

    #[test]
    fn test_get_first_element() {
        let mut jv = JaggedVec::new();
        jv.push(vec![10, 20]);
        jv.push(vec![30]);
        assert_eq!(jv.get(0), Some(&[10, 20][..]));
    }

    #[test]
    fn test_get_last_element() {
        let mut jv = JaggedVec::new();
        jv.push(vec![1]);
        jv.push(vec![2]);
        jv.push(vec![3, 4]);
        assert_eq!(jv.get(2), Some(&[3, 4][..]));
    }

    #[test]
    fn test_get_middle_element() {
        let mut jv = JaggedVec::new();
        jv.push(vec![1]);
        jv.push(vec![2, 3]);
        jv.push(vec![4]);
        assert_eq!(jv.get(1), Some(&[2, 3][..]));
    }

    #[test]
    fn test_get_out_of_bounds_returns_none() {
        let mut jv = JaggedVec::new();
        jv.push(vec![1, 2]);
        assert_eq!(jv.get(1), None);
    }

    #[test]
    fn test_get_on_empty_jagged_vec() {
        let jv: JaggedVec<i32> = JaggedVec::new();
        assert_eq!(jv.get(0), None);
    }

    #[test]
    fn test_get_far_out_of_bounds() {
        let mut jv = JaggedVec::new();
        jv.push(vec![1]);
        assert_eq!(jv.get(100), None);
    }

    #[test]
    fn test_get_with_usize_max() {
        let mut jv = JaggedVec::new();
        jv.push(vec![1]);
        assert_eq!(jv.get(usize::MAX), None);
    }

    #[test]
    fn test_push_and_get_string_type() {
        let mut jv = JaggedVec::new();
        jv.push(vec!["hello".to_string(), "world".to_string()]);
        assert_eq!(
            jv.get(0),
            Some(&["hello".to_string(), "world".to_string()][..])
        );
    }

    #[test]
    fn test_push_from_array() {
        let mut jv = JaggedVec::new();
        jv.push([1, 2, 3]);
        assert_eq!(jv.get(0), Some(&[1, 2, 3][..]));
    }

    #[test]
    fn test_push_from_iterator() {
        let mut jv = JaggedVec::new();
        jv.push((0..5).map(|x| x * 2));
        assert_eq!(jv.get(0), Some(&[0, 2, 4, 6, 8][..]));
    }

    #[test]
    fn test_sequential_pushes_maintain_order() {
        let mut jv = JaggedVec::new();
        for i in 0..10 {
            jv.push(vec![i]);
        }
        for i in 0..10 {
            assert_eq!(jv.get(i), Some(&[i][..]));
        }
    }

    #[test]
    fn test_splits_are_correct_after_multiple_pushes() {
        let mut jv = JaggedVec::new();
        jv.push(vec![1, 2]); // split at 0
        jv.push(vec![3, 4, 5]); // split at 2
        jv.push(vec![6]); // split at 5

        assert_eq!(jv.splt, vec![0, 2, 5]);
    }

    #[test]
    fn test_data_vector_grows_correctly() {
        let mut jv = JaggedVec::new();
        jv.push(vec![1, 2]);
        jv.push(vec![3, 4, 5]);

        assert_eq!(jv.data.len(), 5);
        assert_eq!(jv.data, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_large_number_of_elements() {
        let mut jv = JaggedVec::new();
        jv.push((0..1000).collect::<Vec<_>>());

        let result = jv.get(0);
        assert!(result.is_some());
        assert_eq!(result.unwrap().len(), 1000);
    }

    #[test]
    fn test_many_small_vecs() {
        let mut jv = JaggedVec::new();
        for i in 0..1000 {
            jv.push(vec![i]);
        }

        assert_eq!(jv.splt.len(), 1000);
        assert_eq!(jv.get(999), Some(&[999][..]));
    }

    #[test]
    fn test_mixed_sizes() {
        let mut jv = JaggedVec::new();
        jv.push(vec![1]);
        jv.push(vec![2, 3, 4, 5, 6]);
        jv.push(vec![7, 8]);
        jv.push(Vec::<i32>::new());
        jv.push(vec![9]);

        assert_eq!(jv.get(0), Some(&[1][..]));
        assert_eq!(jv.get(1), Some(&[2, 3, 4, 5, 6][..]));
        assert_eq!(jv.get(2), Some(&[7, 8][..]));
        assert_eq!(jv.get(3), Some(&[][..]));
        assert_eq!(jv.get(4), Some(&[9][..]));
    }

    #[test]
    fn test_custom_type() {
        #[derive(Debug, PartialEq)]
        struct Point {
            x: i32,
            y: i32,
        }

        let mut jv = JaggedVec::new();
        jv.push(vec![Point { x: 1, y: 2 }, Point { x: 3, y: 4 }]);

        assert_eq!(
            jv.get(0),
            Some(&[Point { x: 1, y: 2 }, Point { x: 3, y: 4 }][..])
        );
    }
    #[test]
    fn test_iter_on_empty_jagged_vec() {
        let jv: JaggedVec<i32> = JaggedVec::new();
        let mut iter = jv.iter();
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_iter_single_vec() {
        let mut jv = JaggedVec::new();
        jv.push(vec![1, 2, 3]);
        let mut iter = jv.iter();
        assert_eq!(iter.next(), Some(&[1, 2, 3][..]));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_iter_multiple_vecs() {
        let mut jv = JaggedVec::new();
        jv.push(vec![1, 2]);
        jv.push(vec![3, 4, 5]);
        jv.push(vec![6]);

        let mut iter = jv.iter();
        assert_eq!(iter.next(), Some(&[1, 2][..]));
        assert_eq!(iter.next(), Some(&[3, 4, 5][..]));
        assert_eq!(iter.next(), Some(&[6][..]));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_iter_returns_none_after_exhaustion() {
        let mut jv = JaggedVec::new();
        jv.push(vec![1]);

        let mut iter = jv.iter();
        iter.next();
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_iter_with_empty_slices() {
        let mut jv = JaggedVec::new();
        jv.push(Vec::<i32>::new());
        jv.push(vec![1]);
        jv.push(Vec::<i32>::new());

        let mut iter = jv.iter();
        assert_eq!(iter.next(), Some(&[][..]));
        assert_eq!(iter.next(), Some(&[1][..]));
        assert_eq!(iter.next(), Some(&[][..]));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_iter_for_loop() {
        let mut jv = JaggedVec::new();
        jv.push(vec![1, 2]);
        jv.push(vec![3, 4]);

        let mut result = Vec::new();
        for slice in jv.iter() {
            result.extend_from_slice(slice);
        }

        assert_eq!(result, vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_multiple_iterators_independent() {
        let mut jv = JaggedVec::new();
        jv.push(vec![1]);
        jv.push(vec![2]);

        let mut iter1 = jv.iter();
        let mut iter2 = jv.iter();

        assert_eq!(iter1.next(), Some(&[1][..]));
        assert_eq!(iter2.next(), Some(&[1][..]));
        assert_eq!(iter1.next(), Some(&[2][..]));
        assert_eq!(iter2.next(), Some(&[2][..]));
    }

    #[test]
    fn test_iter_nidx_increments_only_on_some() {
        let mut jv = JaggedVec::new();
        jv.push(vec![1]);

        let mut iter = jv.iter();
        iter.next(); // Returns Some, nidx should be 1
        iter.next(); // Returns None, nidx should stay 1
        iter.next(); // Returns None, nidx should stay 1

        // If nidx incorrectly incremented, this would fail
        assert_eq!(iter.nidx, 1);
    }
}
