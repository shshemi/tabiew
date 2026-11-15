use crate::{
    AppResult,
    sql::{
        db::Database,
        ops::extensions::Extension,
        ops::{
            reply::{Receiver, channel},
            util::quote_literal,
        },
        worker::submit,
    },
};

#[derive(Debug, Clone, Default)]
pub struct Install {
    force: bool,
    from: Option<Repository>,
    version: Option<String>,
}

impl Install {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_force(mut self, force: bool) -> Self {
        self.force = force;
        self
    }

    pub fn with_from(mut self, from: Repository) -> Self {
        self.from = Some(from);
        self
    }

    pub fn with_version(mut self, version: impl Into<String>) -> Self {
        self.version = Some(version.into());
        self
    }

    pub fn run<E: Extension>(&self) -> Receiver<()> {
        let this = self.clone();
        let (send, recv) = channel();
        submit(move |db| send.send(this.install::<E>(db)));
        recv
    }

    fn install<E: Extension>(&self, db: &mut Database) -> AppResult<()> {
        db.execute(&self.sql::<E>(), [])?;
        Ok(())
    }

    fn sql<E: Extension>(&self) -> String {
        let mut sql = String::new();
        if self.force {
            sql.push_str("FORCE ");
        }
        sql.push_str("INSTALL ");
        sql.push_str(E::as_sql());
        if let Some(from) = &self.from {
            sql.push_str(" FROM ");
            sql.push_str(&from.as_sql());
        }
        if let Some(version) = &self.version {
            sql.push_str(" VERSION ");
            sql.push_str(&quote_literal(version));
        }
        sql
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Repository {
    Core,
    CoreNightly,
    Community,
    Custom(String),
}

impl Repository {
    pub fn as_sql(&self) -> String {
        match self {
            Repository::Core => "core".to_owned(),
            Repository::CoreNightly => "core_nightly".to_owned(),
            Repository::Community => "community".to_owned(),
            Repository::Custom(url) => quote_literal(url),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{
        path::{Path, PathBuf},
        sync::atomic::{AtomicUsize, Ordering},
    };

    use super::*;
    use crate::sql::{
        db::{DatabaseConfig, ValueRef},
        ops::extensions::{Excel, SqliteScanner},
    };

    /// A fresh directory under the system temp dir, removed on drop.
    struct TempDir(PathBuf);

    impl TempDir {
        fn new() -> Self {
            static COUNTER: AtomicUsize = AtomicUsize::new(0);
            let path = std::env::temp_dir().join(format!(
                "tabiew-install-test-{}-{}",
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

    /// Whether DuckDB reports the extension as installed in this database.
    fn installed(db: &Database, name: &str) -> bool {
        let mut stmt = db
            .prepare("SELECT installed FROM duckdb_extensions() WHERE extension_name = ?")
            .unwrap();
        let mut rows = stmt.query([name]).unwrap();
        match rows.next().unwrap() {
            Some(row) => matches!(row.get_ref(0).unwrap(), ValueRef::Boolean(true)),
            None => false,
        }
    }

    #[test]
    fn sql_without_options() {
        assert_eq!(Install::new().sql::<Excel>(), "INSTALL excel");
        assert_eq!(
            Install::new().sql::<SqliteScanner>(),
            "INSTALL sqlite_scanner"
        );
    }

    #[test]
    fn sql_with_all_options() {
        let install = Install::new()
            .with_force(true)
            .with_from(Repository::Core)
            .with_version("v1.2.0");
        assert_eq!(
            install.sql::<Excel>(),
            "FORCE INSTALL excel FROM core VERSION 'v1.2.0'"
        );
    }

    #[test]
    fn sql_escapes_quotes_in_version() {
        let install = Install::new().with_version("it's");
        assert_eq!(install.sql::<Excel>(), "INSTALL excel VERSION 'it''s'");
    }

    #[test]
    fn repository_as_sql() {
        assert_eq!(Repository::Core.as_sql(), "core");
        assert_eq!(Repository::CoreNightly.as_sql(), "core_nightly");
        assert_eq!(Repository::Community.as_sql(), "community");
        assert_eq!(
            Repository::Custom("/repo's dir".to_owned()).as_sql(),
            "'/repo''s dir'"
        );
    }

    #[test]
    fn install_from_missing_local_repository_fails() {
        let (mut db, dir) = db_with_empty_extension_dir();
        let missing = TempDir::new();
        let repo = missing.path().join("does-not-exist");
        let result = Install::new()
            .with_from(Repository::Custom(repo.to_str().unwrap().to_owned()))
            .install::<Excel>(&mut db);
        assert!(result.is_err());
        assert!(!installed(&db, Excel::as_sql()));
        assert_eq!(dir.file_count(), 0);
    }

    #[test]
    #[ignore = "downloads the excel extension from extensions.duckdb.org"]
    fn install_from_core_downloads_the_extension() {
        let (mut db, dir) = db_with_empty_extension_dir();
        Install::new().install::<Excel>(&mut db).unwrap();
        assert!(installed(&db, Excel::as_sql()));
        assert!(dir.file_count() > 0);
    }
}
