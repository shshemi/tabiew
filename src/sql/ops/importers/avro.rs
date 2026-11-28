use std::path::Path;

use duckdb::types::Value;

use crate::{
    AppResult,
    sql::{
        db::Database,
        ops::{
            extensions::{self, Token},
            reply::{Receiver, channel},
            util::{pairs_to_duck_db_value, quote_ident, table_name, utf8_path},
        },
        worker::submit,
    },
};

#[derive(Debug, Clone)]
pub struct Avro {
    filename: Option<Value>,
    hive_partitioning: Option<Value>,
    hive_types: Option<Value>,
    hive_types_autocast: Option<Value>,
    allow_empty: Option<Value>,
}

impl Avro {
    pub fn new(_token: Token<extensions::Avro>) -> Self {
        Self {
            filename: None,
            hive_partitioning: None,
            hive_types: None,
            hive_types_autocast: None,
            allow_empty: None,
        }
    }

    pub fn with_filename(mut self, filename: bool) -> Self {
        self.filename = Some(Value::Boolean(filename));
        self
    }

    pub fn with_hive_partitioning(mut self, hive_partitioning: bool) -> Self {
        self.hive_partitioning = Some(Value::Boolean(hive_partitioning));
        self
    }

    pub fn with_hive_types(mut self, hive_types: Vec<(String, String)>) -> Self {
        self.hive_types = pairs_to_duck_db_value(hive_types);
        self
    }

    pub fn with_hive_types_autocast(mut self, hive_types_autocast: bool) -> Self {
        self.hive_types_autocast = Some(Value::Boolean(hive_types_autocast));
        self
    }

    pub fn with_allow_empty(mut self, allow_empty: bool) -> Self {
        self.allow_empty = Some(Value::Boolean(allow_empty));
        self
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

        db.prepare("LOAD avro")?.execute([])?;

        let mut sql = format!(
            "CREATE TABLE {} AS SELECT * FROM read_avro(?",
            quote_ident(&name)
        );
        let mut params = vec![Value::Text(path.to_owned())];
        for (option, value) in self.options() {
            sql.push_str(", ");
            sql.push_str(option);
            sql.push_str(" = ?");
            params.push(value);
        }
        sql.push(')');

        db.prepare(&sql)?
            .execute(duckdb::params_from_iter(params))?;
        Ok(())
    }

    fn options(&self) -> Vec<(&'static str, Value)> {
        [
            ("filename", &self.filename),
            ("hive_partitioning", &self.hive_partitioning),
            ("hive_types", &self.hive_types),
            ("hive_types_autocast", &self.hive_types_autocast),
            ("allow_empty", &self.allow_empty),
        ]
        .into_iter()
        .filter_map(|(option, value)| Some((option, value.clone()?)))
        .collect()
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
            util::quote_literal,
        },
    };

    fn db() -> Database {
        DatabaseConfig::default().open_in_memory().unwrap()
    }

    fn write_avro(dir: &Path, name: &str, n: usize) -> PathBuf {
        let path = dir.join(name);
        let db = db();
        db.execute("LOAD avro", []).unwrap();
        db.execute(
            &format!(
                "COPY (SELECT range AS n, 'v' || range AS s FROM range({n})) TO {} (FORMAT avro)",
                quote_literal(path.to_str().unwrap())
            ),
            [],
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

    fn token() -> Token<extensions::Avro> {
        if let Some(token) = wait(&IsInstalled::new().run::<extensions::Avro>()).unwrap() {
            return token;
        }
        wait(&Install::new().run::<extensions::Avro>()).unwrap();
        wait(&IsInstalled::new().run::<extensions::Avro>())
            .unwrap()
            .expect("avro should be installed now")
    }

    #[test]
    #[ignore = "needs the avro extension, which may be downloaded"]
    fn options_are_empty_by_default() {
        assert!(Avro::new(token()).options().is_empty());
    }

    #[test]
    #[ignore = "needs the avro extension, which may be downloaded"]
    fn options_follow_setters() {
        let avro = Avro::new(token())
            .with_filename(true)
            .with_hive_partitioning(true)
            .with_allow_empty(false);
        assert_eq!(
            avro.options(),
            vec![
                ("filename", Value::Boolean(true)),
                ("hive_partitioning", Value::Boolean(true)),
                ("allow_empty", Value::Boolean(false)),
            ]
        );
    }

    #[test]
    #[ignore = "needs the avro extension, which may be downloaded"]
    fn import_reads_rows() {
        let token = token();
        let dir = tempfile::tempdir().unwrap();
        let path = write_avro(dir.path(), "events.avro", 4);
        let mut db = db();
        Avro::new(token).import(&mut db, &path).unwrap();
        assert_eq!(text(&db, "SELECT count(*)::VARCHAR FROM events"), "4");
        assert_eq!(
            text(&db, "SELECT string_agg(s, ',' ORDER BY n) FROM events"),
            "v0,v1,v2,v3"
        );
    }

    #[test]
    #[ignore = "needs the avro extension, which may be downloaded"]
    fn import_adds_filename() {
        let token = token();
        let dir = tempfile::tempdir().unwrap();
        let path = write_avro(dir.path(), "named.avro", 2);
        let mut db = db();
        Avro::new(token)
            .with_filename(true)
            .import(&mut db, &path)
            .unwrap();
        assert_eq!(
            text(&db, "SELECT DISTINCT filename FROM named"),
            path.to_str().unwrap()
        );
    }

    #[test]
    #[ignore = "needs the avro extension, which may be downloaded"]
    fn every_option_is_accepted() {
        let token = token();
        let dir = tempfile::tempdir().unwrap();
        let partition = dir.path().join("year=2024");
        std::fs::create_dir(&partition).unwrap();
        let path = write_avro(&partition, "all_options.avro", 2);
        let mut db = db();
        Avro::new(token)
            .with_filename(true)
            .with_hive_partitioning(true)
            .with_hive_types(vec![("year".to_owned(), "INTEGER".to_owned())])
            .with_hive_types_autocast(false)
            .with_allow_empty(true)
            .import(&mut db, &path)
            .unwrap();
        assert_eq!(text(&db, "SELECT count(*)::VARCHAR FROM all_options"), "2");
        assert_eq!(
            text(
                &db,
                "SELECT DISTINCT typeof(year) || ' ' || year FROM all_options"
            ),
            "INTEGER 2024"
        );
    }

    #[test]
    #[ignore = "needs the avro extension, which may be downloaded"]
    fn import_missing_file_fails() {
        let token = token();
        let dir = tempfile::tempdir().unwrap();
        assert!(
            Avro::new(token)
                .import(&mut db(), &dir.path().join("missing.avro"))
                .is_err()
        );
    }

    #[test]
    #[ignore = "needs the avro extension, which may be downloaded"]
    fn run_imports_through_the_worker() {
        let token = token();
        let dir = tempfile::tempdir().unwrap();
        let path = write_avro(dir.path(), "avro_run_through_worker.avro", 2);
        wait(&Avro::new(token).run(&path)).unwrap();
    }
}
