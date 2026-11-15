use std::path::Path;

use duckdb::types::Value;

use crate::{
    AppResult,
    sql::{
        db::Database,
        ops::{
            extensions::{self, Token},
            reply::{Receiver, channel},
            util::{quote_ident, table_name, utf8_path},
        },
        worker::submit,
    },
};

#[derive(Debug, Clone)]
pub struct Excel {
    sheet: Option<Value>,
    range: Option<Value>,
    header: Option<Value>,
    stop_at_empty: Option<Value>,

    all_varchar: Option<Value>,
    empty_as_varchar: Option<Value>,
    ignore_errors: Option<Value>,
    normalize_names: Option<Value>,
}

impl Excel {
    pub fn new(_token: Token<extensions::Excel>) -> Self {
        Self {
            sheet: None,
            range: None,
            header: None,
            stop_at_empty: None,
            all_varchar: None,
            empty_as_varchar: None,
            ignore_errors: None,
            normalize_names: None,
        }
    }

    pub fn with_sheet(mut self, sheet: impl Into<String>) -> Self {
        self.sheet = Some(Value::Text(sheet.into()));
        self
    }

    pub fn with_range(mut self, range: impl Into<String>) -> Self {
        self.range = Some(Value::Text(range.into()));
        self
    }

    pub fn with_header(mut self, header: bool) -> Self {
        self.header = Some(Value::Boolean(header));
        self
    }

    pub fn with_stop_at_empty(mut self, stop_at_empty: bool) -> Self {
        self.stop_at_empty = Some(Value::Boolean(stop_at_empty));
        self
    }

    pub fn with_all_varchar(mut self, all_varchar: bool) -> Self {
        self.all_varchar = Some(Value::Boolean(all_varchar));
        self
    }

    pub fn with_empty_as_varchar(mut self, empty_as_varchar: bool) -> Self {
        self.empty_as_varchar = Some(Value::Boolean(empty_as_varchar));
        self
    }

    pub fn with_ignore_errors(mut self, ignore_errors: bool) -> Self {
        self.ignore_errors = Some(Value::Boolean(ignore_errors));
        self
    }

    pub fn with_normalize_names(mut self, normalize_names: bool) -> Self {
        self.normalize_names = Some(Value::Boolean(normalize_names));
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

        db.prepare("LOAD excel")?.execute([])?;

        let mut sql = format!(
            "CREATE TABLE {} AS SELECT * FROM read_xlsx(?",
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
            ("sheet", &self.sheet),
            ("range", &self.range),
            ("header", &self.header),
            ("stop_at_empty", &self.stop_at_empty),
            ("all_varchar", &self.all_varchar),
            ("empty_as_varchar", &self.empty_as_varchar),
            ("ignore_errors", &self.ignore_errors),
            ("normalize_names", &self.normalize_names),
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

    /// Writes `n` rows (n BIGINT, s VARCHAR) to sheet `data` of an xlsx file using DuckDB itself.
    fn write_xlsx(dir: &Path, name: &str, n: usize) -> PathBuf {
        let path = dir.join(name);
        let db = db();
        db.execute("LOAD excel", []).unwrap();
        db.execute(
            &format!(
                "COPY (SELECT range AS n, 'v' || range AS s FROM range({n})) TO {} \
                 (FORMAT xlsx, HEADER true, SHEET 'data')",
                quote_literal(path.to_str().unwrap())
            ),
            [],
        )
        .unwrap();
        path
    }

    /// The first column of the first row, as text.
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

    /// Checks for the excel extension, installing it first if it is missing.
    fn token() -> Token<extensions::Excel> {
        if let Some(token) = wait(&IsInstalled::new().run::<extensions::Excel>()).unwrap() {
            return token;
        }
        wait(&Install::new().run::<extensions::Excel>()).unwrap();
        wait(&IsInstalled::new().run::<extensions::Excel>())
            .unwrap()
            .expect("excel should be installed now")
    }

    #[test]
    #[ignore = "needs the excel extension, which may be downloaded"]
    fn options_are_empty_by_default() {
        assert!(Excel::new(token()).options().is_empty());
    }

    #[test]
    #[ignore = "needs the excel extension, which may be downloaded"]
    fn options_follow_setters() {
        let excel = Excel::new(token())
            .with_sheet("data")
            .with_range("A1:B10")
            .with_header(true)
            .with_normalize_names(true);
        assert_eq!(
            excel.options(),
            vec![
                ("sheet", Value::Text("data".to_owned())),
                ("range", Value::Text("A1:B10".to_owned())),
                ("header", Value::Boolean(true)),
                ("normalize_names", Value::Boolean(true)),
            ]
        );
    }

    #[test]
    #[ignore = "needs the excel extension, which may be downloaded"]
    fn import_reads_the_sheet() {
        let token = token();
        let dir = tempfile::tempdir().unwrap();
        let path = write_xlsx(dir.path(), "book.xlsx", 4);
        let mut db = db();
        Excel::new(token)
            .with_sheet("data")
            .with_header(true)
            .import(&mut db, &path)
            .unwrap();
        assert_eq!(text(&db, "SELECT count(*)::VARCHAR FROM book"), "4");
        assert_eq!(
            text(&db, "SELECT string_agg(s, ',' ORDER BY n) FROM book"),
            "v0,v1,v2,v3"
        );
    }

    #[test]
    #[ignore = "needs the excel extension, which may be downloaded"]
    fn import_unknown_sheet_fails() {
        let token = token();
        let dir = tempfile::tempdir().unwrap();
        let path = write_xlsx(dir.path(), "sheets.xlsx", 1);
        let error = Excel::new(token)
            .with_sheet("missing")
            .import(&mut db(), &path)
            .unwrap_err();
        assert!(error.to_string().contains("not found"), "{error}");
    }

    #[test]
    #[ignore = "needs the excel extension, which may be downloaded"]
    fn import_fails_without_downloading_when_extension_is_gone() {
        let token = token();
        let dir = tempfile::tempdir().unwrap();
        let path = write_xlsx(dir.path(), "gone.xlsx", 1);
        let extension_dir = tempfile::tempdir().unwrap();
        let mut db = db();
        db.execute(
            &format!(
                "SET extension_directory = {}",
                quote_literal(extension_dir.path().to_str().unwrap())
            ),
            [],
        )
        .unwrap();
        let error = Excel::new(token).import(&mut db, &path).unwrap_err();
        assert!(error.to_string().contains("not found"), "{error}");
        assert_eq!(std::fs::read_dir(extension_dir.path()).unwrap().count(), 0);
    }

    #[test]
    #[ignore = "needs the excel extension, which may be downloaded"]
    fn run_imports_through_the_worker() {
        let token = token();
        let dir = tempfile::tempdir().unwrap();
        let path = write_xlsx(dir.path(), "excel_run_through_worker.xlsx", 2);
        wait(&Excel::new(token).with_sheet("data").run(&path)).unwrap();
    }
}
