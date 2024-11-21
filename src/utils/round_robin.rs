use std::collections::VecDeque;

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

#[cfg(test)]
mod tests {
    use super::*;

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
}
