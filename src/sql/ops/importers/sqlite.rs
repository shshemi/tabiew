use std::path::Path;

use crate::{
    AppResult,
    sql::{
        db::Database,
        ops::{
            extensions::{self, Token},
            reply::{Receiver, channel},
            util::{quote_ident, quote_literal, table_name, utf8_path},
        },
        worker::submit,
    },
};

#[derive(Debug, Clone)]
pub struct Sqlite {}

impl Sqlite {
    pub fn new(_token: Token<extensions::SqliteScanner>) -> Self {
        Self {}
    }

    pub fn run(&self, path: impl AsRef<Path>) -> Receiver<()> {
        let this = self.clone();
        let path = path.as_ref().to_path_buf();
        let (send, recv) = channel();
        submit(move |db| send.send(this.import(db, &path)));
        recv
    }

    fn import(&self, db: &mut Database, path: &Path) -> AppResult<()> {
        let name = table_name(path)?;
        let path = utf8_path(path)?;

        db.prepare("LOAD sqlite_scanner")?.execute([])?;
        db.prepare(&format!(
            "ATTACH {} AS {} (TYPE sqlite, READ_ONLY)",
            quote_literal(path),
            quote_ident(&name)
        ))?
        .execute([])?;
        let checked = db
            .prepare("SELECT COUNT(*) FROM duckdb_tables() WHERE database_name = ?")
            .and_then(|mut stmt| stmt.query([&name])?.next().map(|_| ()));
        if let Err(err) = checked {
            db.prepare(&format!("DETACH {}", quote_ident(&name)))?
                .execute([])?;
            return Err(err);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::{
        path::{Path, PathBuf},
        time::{Duration, Instant},
    };

    use super::*;
    use crate::sql::{
        db::{DatabaseConfig, ValueRef},
        ops::{
            Next,
            extensions::{Install, IsInstalled},
        },
    };

    fn db() -> Database {
        DatabaseConfig::default().open_in_memory().unwrap()
    }

    fn write_sqlite(dir: &Path, name: &str) -> PathBuf {
        let path = dir.join(name);
        let conn = rusqlite::Connection::open(&path).unwrap();
        conn.execute_batch(
            "CREATE TABLE users (id INTEGER, name TEXT); \
             INSERT INTO users VALUES (1, 'ann'), (2, 'bob'); \
             CREATE TABLE \"odd \"\"name\" (v REAL); \
             INSERT INTO \"odd \"\"name\" VALUES (1.5);",
        )
        .unwrap();
        path
    }

    fn text(db: &Database, sql: &str) -> String {
        let mut stmt = db.prepare(sql).unwrap();
        let mut rows = stmt.query([]).unwrap();
        let row = rows.next().unwrap().expect("query returned no rows");
        match row.get_ref(0).unwrap() {
            ValueRef::Null => "NULL".to_owned(),
            ValueRef::Text(bytes) => String::from_utf8_lossy(bytes).into_owned(),
            other => panic!("expected text, got {other:?}"),
        }
    }

    fn wait<T>(recv: &Receiver<T>) -> AppResult<T> {
        let deadline = Instant::now() + Duration::from_secs(60);
        loop {
            match recv.recv() {
                Next::Value(value) => return value,
                Next::Done => panic!("sender dropped without sending a value"),
                Next::Empty if Instant::now() > deadline => panic!("timed out waiting for a value"),
                Next::Empty => std::thread::sleep(Duration::from_millis(5)),
            }
        }
    }

    fn token() -> Token<extensions::SqliteScanner> {
        if let Some(token) = wait(&IsInstalled::new().run::<extensions::SqliteScanner>()).unwrap() {
            return token;
        }
        wait(&Install::new().run::<extensions::SqliteScanner>()).unwrap();
        wait(&IsInstalled::new().run::<extensions::SqliteScanner>())
            .unwrap()
            .expect("sqlite_scanner should be installed now")
    }

    #[test]
    #[ignore = "needs the sqlite_scanner extension, which may be downloaded"]
    fn import_attaches_every_table() {
        let token = token();
        let dir = tempfile::tempdir().unwrap();
        let path = write_sqlite(dir.path(), "shop.db");
        let mut db = db();
        Sqlite::new(token).import(&mut db, &path).unwrap();
        assert_eq!(
            text(
                &db,
                "SELECT string_agg(name, ',' ORDER BY id) FROM shop.users"
            ),
            "ann,bob"
        );
        assert_eq!(
            text(&db, "SELECT v::VARCHAR FROM shop.\"odd \"\"name\""),
            "1.5"
        );
        assert_eq!(
            text(
                &db,
                "SELECT string_agg(table_name, ',' ORDER BY table_name) \
                 FROM duckdb_tables() WHERE database_name = 'shop'"
            ),
            "odd \"name,users"
        );
    }

    #[test]
    #[ignore = "needs the sqlite_scanner extension, which may be downloaded"]
    fn import_is_read_only() {
        let token = token();
        let dir = tempfile::tempdir().unwrap();
        let path = write_sqlite(dir.path(), "locked.db");
        let mut db = db();
        Sqlite::new(token).import(&mut db, &path).unwrap();
        assert!(
            db.execute("INSERT INTO locked.users VALUES (3, 'cat')", [])
                .is_err()
        );
    }

    #[test]
    #[ignore = "needs the sqlite_scanner extension, which may be downloaded"]
    fn import_twice_with_the_same_name_fails() {
        let token = token();
        let dir = tempfile::tempdir().unwrap();
        let path = write_sqlite(dir.path(), "twice.db");
        let mut db = db();
        let sqlite = Sqlite::new(token);
        sqlite.import(&mut db, &path).unwrap();
        assert!(sqlite.import(&mut db, &path).is_err());
    }

    #[test]
    #[ignore = "needs the sqlite_scanner extension, which may be downloaded"]
    fn import_non_sqlite_file_fails() {
        let token = token();
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("fake.db");
        std::fs::write(&path, "not a database\n".repeat(64)).unwrap();
        let mut db = db();
        assert!(Sqlite::new(token).import(&mut db, &path).is_err());
        assert_eq!(
            text(
                &db,
                "SELECT COUNT(*)::VARCHAR FROM duckdb_databases() WHERE database_name = 'fake'"
            ),
            "0"
        );
    }

    #[test]
    #[ignore = "needs the sqlite_scanner extension, which may be downloaded"]
    fn import_quotes_the_path() {
        let token = token();
        let dir = tempfile::tempdir().unwrap();
        let path = write_sqlite(dir.path(), "it's.db");
        let mut db = db();
        Sqlite::new(token).import(&mut db, &path).unwrap();
        assert_eq!(
            text(&db, "SELECT name FROM \"it's\".users WHERE id = 2"),
            "bob"
        );
    }

    #[test]
    #[ignore = "needs the sqlite_scanner extension, which may be downloaded"]
    fn run_imports_through_the_worker() {
        let token = token();
        let dir = tempfile::tempdir().unwrap();
        let path = write_sqlite(dir.path(), "sqlite_run_through_worker.db");
        wait(&Sqlite::new(token).run(&path)).unwrap();
    }
}
