use std::marker::PhantomData;

use crate::{
    AppResult,
    sql::{
        db::{Database, ValueRef},
        ops::extensions::Extension,
        ops::reply::{Receiver, channel},
        worker::submit,
    },
};

#[derive(Debug, Clone, Default)]
pub struct IsInstalled;

impl IsInstalled {
    pub fn new() -> Self {
        Self
    }

    pub fn run<E: Extension>(&self) -> Receiver<Option<Token<E>>> {
        let this = self.clone();
        let (send, recv) = channel();
        submit(move |db| send.send(this.check::<E>(db)));
        recv
    }

    fn check<E: Extension>(&self, db: &Database) -> AppResult<Option<Token<E>>> {
        let mut stmt =
            db.prepare("SELECT installed FROM duckdb_extensions() WHERE extension_name = ?")?;
        let mut rows = stmt.query([E::as_sql()])?;
        match rows.next()? {
            Some(row) if matches!(row.get_ref(0)?, ValueRef::Boolean(true)) => Ok(Some(Token {
                _phantom: PhantomData,
            })),
            _ => Ok(None),
        }
    }
}

pub struct Token<E> {
    _phantom: PhantomData<E>,
}

#[cfg(test)]
mod tests {
    use std::{
        path::{Path, PathBuf},
        sync::atomic::{AtomicUsize, Ordering},
        time::{Duration, Instant},
    };

    use super::*;
    use crate::sql::{
        db::DatabaseConfig,
        ops::{
            Next,
            extensions::{Excel, Json, Parquet},
            util::quote_literal,
        },
    };

    /// A fresh directory under the system temp dir, removed on drop.
    struct TempDir(PathBuf);

    impl TempDir {
        fn new() -> Self {
            static COUNTER: AtomicUsize = AtomicUsize::new(0);
            let path = std::env::temp_dir().join(format!(
                "tabiew-is-installed-test-{}-{}",
                std::process::id(),
                COUNTER.fetch_add(1, Ordering::Relaxed)
            ));
            std::fs::create_dir_all(&path).unwrap();
            Self(path)
        }

        fn path(&self) -> &Path {
            &self.0
        }

        fn file_count(&self) -> usize {
            fn count(dir: &Path) -> usize {
                std::fs::read_dir(dir)
                    .map(|entries| {
                        entries
                            .flatten()
                            .map(|e| {
                                if e.path().is_dir() {
                                    count(&e.path())
                                } else {
                                    1
                                }
                            })
                            .sum()
                    })
                    .unwrap_or(0)
            }
            count(&self.0)
        }
    }

    impl Drop for TempDir {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }

    /// A fresh in-memory database with the default config.
    fn db() -> Database {
        DatabaseConfig::default().open_in_memory().unwrap()
    }

    /// A fresh database whose extension directory is an empty temp dir,
    /// so only statically linked extensions count as installed.
    fn db_with_empty_extension_dir() -> (Database, TempDir) {
        let dir = TempDir::new();
        let db = db();
        db.execute(
            &format!(
                "SET extension_directory = {}",
                quote_literal(dir.path().to_str().unwrap())
            ),
            [],
        )
        .unwrap();
        (db, dir)
    }

    /// Waits for the first value on a receiver, failing the test after 30 seconds.
    fn wait<T>(recv: &Receiver<T>) -> AppResult<T> {
        let deadline = Instant::now() + Duration::from_secs(30);
        loop {
            match recv.recv() {
                Next::Value(value) => return value,
                Next::Done => panic!("sender dropped without sending a value"),
                Next::Empty if Instant::now() > deadline => panic!("timed out waiting for a value"),
                Next::Empty => std::thread::sleep(Duration::from_millis(5)),
            }
        }
    }

    #[test]
    fn builtin_extensions_are_installed() {
        let db = db();
        assert!(IsInstalled::new().check::<Parquet>(&db).unwrap().is_some());
        assert!(IsInstalled::new().check::<Json>(&db).unwrap().is_some());
    }

    #[test]
    fn builtin_extensions_do_not_depend_on_the_extension_dir() {
        let (db, _dir) = db_with_empty_extension_dir();
        assert!(IsInstalled::new().check::<Parquet>(&db).unwrap().is_some());
    }

    #[test]
    fn missing_extension_is_none() {
        let (db, dir) = db_with_empty_extension_dir();
        assert!(IsInstalled::new().check::<Excel>(&db).unwrap().is_none());
        assert_eq!(dir.file_count(), 0, "checking must not download anything");
    }

    #[test]
    fn run_returns_a_token_through_the_worker() {
        assert!(
            wait(&IsInstalled::new().run::<Parquet>())
                .unwrap()
                .is_some()
        );
    }
}
