use std::{
    ops::Deref,
    sync::{OnceLock, RwLock, RwLockReadGuard},
};

use crate::tui::theme::{Monokai, Styler};

static THEME: Global<Box<dyn Styler + Send + Sync>> = Global::new(|| Box::new(Monokai));

pub fn set_theme(theme: Box<dyn Styler + Send + Sync>) {
    THEME.set(theme);
}

pub fn theme() -> GlobalRef<'static, Box<dyn Styler + Send + Sync>> {
    THEME.as_ref()
}

pub struct Global<T> {
    inner: RwLock<Lazy<T>>,
}

impl<T> Global<T> {
    pub const fn new(init: fn() -> T) -> Self {
        Self {
            inner: RwLock::new(Lazy::new(init)),
        }
    }

    pub fn as_ref(&self) -> GlobalRef<'_, T> {
        GlobalRef {
            inner: self.inner.read().unwrap(),
        }
    }

    pub fn set(&self, val: T) {
        self.inner.write().unwrap().set(val);
    }
}

pub struct GlobalRef<'a, T> {
    inner: RwLockReadGuard<'a, Lazy<T>>,
}

impl<T> Deref for GlobalRef<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.inner.as_ref()
    }
}

struct Lazy<T> {
    val: OnceLock<T>,
    init: fn() -> T,
}

impl<T> Lazy<T> {
    pub const fn new(init: fn() -> T) -> Self {
        Lazy {
            val: OnceLock::new(),
            init,
        }
    }

    pub fn as_ref(&self) -> &T {
        self.val.get_or_init(self.init)
    }
    pub fn set(&mut self, val: T) {
        let _ = self.val.take();
        let _ = self.val.set(val);
    }
}
