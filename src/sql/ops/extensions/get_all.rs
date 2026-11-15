use crate::{
    AppResult,
    sql::{
        db::{Database, ValueRef},
        ops::{
            extensions::Info,
            reply::{Receiver, channel},
        },
        worker::submit,
    },
};

#[derive(Debug, Clone, Default)]
pub struct GetAll;

impl GetAll {
    pub fn new() -> Self {
        Self
    }

    pub fn run(&self) -> Receiver<Vec<Info>> {
        let this = self.clone();
        let (send, recv) = channel();
        submit(move |db| send.send(this.get(db)));
        recv
    }

    fn get(&self, db: &Database) -> AppResult<Vec<Info>> {
        let mut stmt = db.prepare(
            "SELECT extension_name, installed, loaded, coalesce(description, '') \
             FROM duckdb_extensions() ORDER BY extension_name",
        )?;
        let mut rows = stmt.query([])?;
        let mut extensions = Vec::new();
        while let Some(row) = rows.next()? {
            extensions.push(Info {
                name: text(row.get_ref(0)?),
                installed: matches!(row.get_ref(1)?, ValueRef::Boolean(true)),
                loaded: matches!(row.get_ref(2)?, ValueRef::Boolean(true)),
                description: text(row.get_ref(3)?),
            });
        }
        Ok(extensions)
    }
}

fn text(value: ValueRef) -> String {
    match value {
        ValueRef::Text(bytes) => String::from_utf8_lossy(bytes).into_owned(),
        _ => String::new(),
    }
}

#[cfg(test)]
mod tests {
    use std::{
        path::{Path, PathBuf},
        sync::atomic::{AtomicUsize, Ordering},
        time::{Duration, Instant},
    };

    use super::*;
    use crate::{
        AppResult,
        sql::{
            db::DatabaseConfig,
            ops::{Next, util::quote_literal},
        },
    };

    /// A fresh directory under the system temp dir, removed on drop.
    struct TempDir(PathBuf);

    impl TempDir {
        fn new() -> Self {
            static COUNTER: AtomicUsize = AtomicUsize::new(0);
            let path = std::env::temp_dir().join(format!(
                "tabiew-get-all-test-{}-{}",
                std::process::id(),
                COUNTER.fetch_add(1, Ordering::Relaxed)
            ));
            std::fs::create_dir_all(&path).unwrap();
            Self(path)
        }

        fn path(&self) -> &Path {
            &self.0
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

    fn find<'a>(infos: &'a [Info], name: &str) -> &'a Info {
        infos
            .iter()
            .find(|info| info.name == name)
            .unwrap_or_else(|| panic!("{name} not listed"))
    }

    #[test]
    fn lists_builtin_extensions_as_installed_and_loaded() {
        let infos = GetAll::new().get(&db()).unwrap();
        for name in ["parquet", "json"] {
            let info = find(&infos, name);
            assert!(info.installed, "{name} should be installed");
            assert!(info.loaded, "{name} should be loaded");
        }
    }

    #[test]
    fn lists_missing_extension_as_not_installed() {
        let (db, _dir) = db_with_empty_extension_dir();
        let infos = GetAll::new().get(&db).unwrap();
        let excel = find(&infos, "excel");
        assert!(!excel.installed);
        assert!(!excel.loaded);
    }

    #[test]
    fn is_sorted_by_name_and_has_descriptions() {
        let infos = GetAll::new().get(&db()).unwrap();
        assert!(infos.windows(2).all(|pair| pair[0].name <= pair[1].name));
        assert!(!find(&infos, "parquet").description.is_empty());
    }

    #[test]
    fn run_returns_the_list_through_the_worker() {
        let infos = wait(&GetAll::new().run()).unwrap();
        assert!(infos.iter().any(|info| info.name == "parquet"));
    }
}
