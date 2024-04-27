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