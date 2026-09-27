use std::{
    collections::VecDeque,
    sync::{Arc, LazyLock, Mutex},
};

#[derive(Debug)]
pub struct CappedVec<T> {
    len: usize,
    vec: LazyLock<Arc<Mutex<VecDeque<T>>>>,
}

impl<T> CappedVec<T>
where
    T: Clone,
{
    pub const fn new(len: usize) -> Self {
        Self {
            len,
            vec: LazyLock::new(|| Arc::new(Mutex::new(VecDeque::new()))),
        }
    }

    pub fn push(&self, val: T) {
        if let Ok(mut v) = self.vec.lock() {
            v.push_front(val);
            v.truncate(self.len);
        }
    }

    pub fn to_vec(&self) -> Vec<T> {
        self.vec.lock().unwrap().iter().cloned().collect()
    }
}
