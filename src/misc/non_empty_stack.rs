#[derive(Debug)]
pub struct NonEmptyStack<T> {
    base: T,
    stack: Vec<T>,
}

impl<T> NonEmptyStack<T> {
    pub fn new(base: T) -> Self {
        Self {
            base,
            stack: Default::default(),
        }
    }

    pub fn push(&mut self, value: T) {
        self.stack.push(value);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.stack.pop()
    }

    pub fn last(&self) -> &T {
        self.stack.last().unwrap_or(&self.base)
    }

    pub fn last_mut(&mut self) -> &mut T {
        self.stack.last_mut().unwrap_or(&mut self.base)
    }

    pub fn len_without_base(&self) -> usize {
        self.stack.len()
    }

    pub fn base(&self) -> &T {
        &self.base
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        std::iter::once(&self.base).chain(self.stack.iter())
    }
}

#[cfg(test)]
mod tests {
    use super::NonEmptyStack;

    #[test]
    fn new_and_base() {
        let s = NonEmptyStack::new(1);
        assert_eq!(s.base(), &1);
        assert_eq!(s.len_without_base(), 0);
        assert_eq!(s.last(), &1);
        let v: Vec<_> = s.iter().copied().collect();
        assert_eq!(v, vec![1]);
    }

    #[test]
    fn push_pop_and_last() {
        let mut s = NonEmptyStack::new(1);
        s.push(2);
        s.push(3);
        assert_eq!(s.len_without_base(), 2);
        assert_eq!(s.last(), &3);
        assert_eq!(s.pop(), Some(3));
        assert_eq!(s.pop(), Some(2));
        assert_eq!(s.pop(), None);
        assert_eq!(s.last(), &1);
    }

    #[test]
    fn last_mut_modifies_top_or_base() {
        // modify base when stack is empty
        let mut s = NonEmptyStack::new(0);
        *s.last_mut() = 5;
        assert_eq!(s.base(), &5);

        // modify top when stack is non-empty
        s.push(10);
        s.push(20);
        *s.last_mut() += 7;
        assert_eq!(s.pop(), Some(27));
        assert_eq!(s.last(), &10);
    }

    #[test]
    fn iter_order() {
        let mut s = NonEmptyStack::new(1);
        s.push(2);
        s.push(3);
        let v: Vec<_> = s.iter().copied().collect();
        assert_eq!(v, vec![1, 2, 3]);
    }
}
