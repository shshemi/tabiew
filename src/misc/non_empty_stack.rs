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
}
