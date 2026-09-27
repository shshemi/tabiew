use std::sync::{Arc, LazyLock, Mutex};

#[derive(Debug)]
pub struct History<T> {
    len: usize,
    vec: LazyLock<Arc<Mutex<Vec<T>>>>,
}

impl<T> History<T>
where
    T: Clone,
{
    pub const fn new(len: usize) -> Self {
        Self {
            len,
            vec: LazyLock::new(|| Arc::new(Mutex::new(Vec::default()))),
        }
    }

    pub fn push(&self, val: T) {
        if let Ok(mut v) = self.vec.lock() {
            v.insert(0, val);
            if v.len() > self.len {
                v.pop();
            }
        }
    }

    pub fn to_vec(&self) -> Vec<T> {
        self.vec.lock().unwrap().clone()
    }
}
