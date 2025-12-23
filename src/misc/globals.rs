use std::{
    cell::UnsafeCell,
    io::{Cursor, Read},
    ops::{Deref, DerefMut},
    sync::{Mutex, MutexGuard, Once, OnceLock, RwLockReadGuard},
};

use crate::{misc::config::Config, tui::themes::theme::LoadedTheme};

use super::sql::SqlBackend;

// static CONFIG: Config = Config::new();
static SQL_BACKEND: Mutex<Lazy<SqlBackend>> = Mutex::new(Lazy::new(SqlBackend::default));
static STDIN_CONTENT: OnceLock<Vec<u8>> = OnceLock::new();

pub fn config() -> &'static Config {
    static CONFIG: OnceLock<Config> = OnceLock::new();
    CONFIG.get_or_init(Config::new)
}

pub fn set_theme(theme: impl Into<LoadedTheme>) {
    *config().theme_mut() = theme.into();
}

pub fn theme() -> impl Deref<Target = LoadedTheme> {
    config().theme()
}

pub fn sql() -> impl DerefMut<Target = SqlBackend> {
    Global {
        inner: SQL_BACKEND.lock().unwrap(),
    }
}

pub fn stdin() -> Cursor<&'static Vec<u8>> {
    Cursor::new(STDIN_CONTENT.get_or_init(|| {
        let mut buf = Vec::new();
        std::io::stdin().read_to_end(&mut buf).unwrap();
        buf
    }))
}

pub struct Global<T> {
    inner: T,
}

impl<T> Deref for Global<RwLockReadGuard<'_, Lazy<T>>> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.inner.as_ref()
    }
}

impl<T> Deref for Global<MutexGuard<'_, Lazy<T>>> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.inner.as_ref()
    }
}

impl<T> DerefMut for Global<MutexGuard<'_, Lazy<T>>> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.inner.deref_mut()
    }
}

#[derive(Debug)]
struct Lazy<T> {
    once: Once,
    val: UnsafeCell<Option<T>>,
    init: fn() -> T,
}

unsafe impl<T> Sync for Lazy<T> where T: Sync {}

impl<T> Lazy<T> {
    pub const fn new(init: fn() -> T) -> Self {
        Lazy {
            once: Once::new(),
            val: UnsafeCell::new(None),
            init,
        }
    }

    pub fn as_ref(&self) -> &T {
        self.once.call_once(|| {
            let deref = unsafe { &mut *self.val.get() };
            if deref.is_none() {
                *deref = Some((self.init)())
            }
        });
        unsafe { (*self.val.get()).as_ref().unwrap() }
    }

    pub fn as_mut(&mut self) -> &mut T {
        self.once.call_once(|| {
            if self.val.get_mut().is_none() {
                *self.val.get_mut() = Some((self.init)());
            }
        });
        self.val.get_mut().as_mut().unwrap()
    }
}

impl<T> Deref for Lazy<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl<T> DerefMut for Lazy<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Barrier};
    use std::thread;

    #[test]
    fn test_lazy_init() {
        // Test that the initialization function is called exactly once
        static mut INIT_COUNT: usize = 0;

        let lazy = Lazy::new(|| unsafe {
            INIT_COUNT += 1;
            42
        });

        assert_eq!(*lazy, 42);
        assert_eq!(*lazy, 42);
        assert_eq!(unsafe { INIT_COUNT }, 1);
    }

    #[test]
    fn test_lazy_set() {
        // Test that set replaces the value
        static mut INIT_COUNT: usize = 0;

        let mut lazy = Lazy::new(|| unsafe {
            INIT_COUNT += 1;
            30
        });

        *lazy = 20;
        assert_eq!(*lazy, 20);
        assert_eq!(unsafe { INIT_COUNT }, 1)
    }

    #[test]
    fn test_lazy_deref() {
        // Test Deref implementation
        let lazy = Lazy::new(|| String::from("test"));

        assert_eq!(lazy.len(), 4);
        assert_eq!(lazy.as_str(), "test");
    }

    #[test]
    fn test_lazy_deref_mut() {
        // Test DerefMut implementation
        let mut lazy = Lazy::new(|| vec![1, 2, 3]);

        lazy.push(4);
        assert_eq!(*lazy, vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_lazy_multithreaded_init() {
        // Test that initialization happens exactly once even with multiple threads
        static mut INIT_COUNT: usize = 0;

        let lazy = Arc::new(Lazy::new(|| {
            unsafe {
                INIT_COUNT += 1;
            }
            42
        }));

        let threads = 32;
        let barrier = Arc::new(Barrier::new(threads));
        let mut handles = vec![];

        for _ in 0..threads {
            let lazy = lazy.clone();
            let barrier_clone = Arc::clone(&barrier);

            let handle = thread::spawn(move || {
                barrier_clone.wait();
                assert_eq!(lazy.as_ref().as_ref(), &42);
            });

            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        assert_eq!(unsafe { INIT_COUNT }, 1);
    }

    #[test]
    fn test_lazy_as_ref_as_mut() {
        // Test that as_ref and as_mut work correctly
        let mut lazy = Lazy::new(|| String::from("hello"));

        {
            let reference = lazy.as_ref();
            assert_eq!(reference, "hello");
        }

        {
            let mut_reference = lazy.as_mut();
            mut_reference.push_str(" world");
        }

        assert_eq!(*lazy, "hello world");
    }

    #[test]
    fn test_large_value() {
        // Test with a larger value to ensure memory handling is correct
        struct LargeStruct {
            data: [u8; 1024],
        }

        impl LargeStruct {
            fn new() -> Self {
                let mut data = [0u8; 1024];
                for (i, d) in data.iter_mut().enumerate() {
                    *d = (i % 255) as u8;
                }
                LargeStruct { data }
            }

            fn sum(&self) -> u32 {
                self.data.iter().map(|&x| x as u32).sum()
            }
        }

        let lazy = Lazy::new(LargeStruct::new);
        assert_eq!(lazy.sum(), (0..1024).map(|i| (i % 255) as u32).sum::<u32>());
    }
}
