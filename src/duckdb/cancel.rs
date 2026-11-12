use std::{
    cell::RefCell,
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, Ordering},
    },
};

use anyhow::anyhow;
use duckdb::{Connection, InterruptHandle};

use crate::AppResult;

thread_local! {
    static CURRENT: RefCell<Option<Cancel>> = const { RefCell::new(None) };
}

#[derive(Clone, Default)]
pub struct Cancel(Arc<Inner>);

#[derive(Default)]
struct Inner {
    cancelled: AtomicBool,
    handles: Mutex<Vec<Arc<InterruptHandle>>>,
}

impl Cancel {
    pub fn cancel(&self) {
        self.0.cancelled.store(true, Ordering::Release);
        for handle in self.0.handles.lock().unwrap().iter() {
            handle.interrupt();
        }
    }
    pub fn cancelled(&self) -> bool {
        self.0.cancelled.load(Ordering::Acquire)
    }
    pub fn watch(&self, conn: &Connection) {
        let handle = conn.interrupt_handle();
        if self.cancelled() {
            handle.interrupt();
        }
        self.0.handles.lock().unwrap().push(handle);
    }
    pub fn scoped<T>(&self, job: impl FnOnce() -> T) -> T {
        struct Reset;
        impl Drop for Reset {
            fn drop(&mut self) {
                CURRENT.with(|current| current.borrow_mut().take());
            }
        }
        CURRENT.with(|current| *current.borrow_mut() = Some(self.clone()));
        let _reset = Reset;
        job()
    }
}

pub fn check() -> AppResult<()> {
    CURRENT.with(|current| match current.borrow().as_ref() {
        Some(cancel) if cancel.cancelled() => Err(anyhow!("Cancelled")),
        _ => Ok(()),
    })
}
