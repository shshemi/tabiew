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
pub struct Parquet {
    binary_as_string: Option<Value>,
    can_have_nan: Option<Value>,
    int96_as: Option<Value>,
    utf8_validation: Option<Value>,
    encryption_config: Option<Value>,

    explicit_cardinality: Option<Value>,
    prefetch_strategy: Option<Value>,

    filename: Option<Value>,
    file_row_number: Option<Value>,
    union_by_name: Option<Value>,
    hive_partitioning: Option<Value>,
    hive_types: Option<Value>,
    hive_types_autocast: Option<Value>,
    allow_empty: Option<Value>,
}

impl Parquet {
    pub fn new(_token: Token<extensions::Parquet>) -> Self {
        Self {
            binary_as_string: None,
            can_have_nan: None,
            int96_as: None,
            utf8_validation: None,
            encryption_config: None,
            explicit_cardinality: None,
            prefetch_strategy: None,
            filename: None,
            file_row_number: None,
            union_by_name: None,
            hive_partitioning: None,
            hive_types: None,
            hive_types_autocast: None,
            allow_empty: None,
        }
    }

    pub fn with_binary_as_string(mut self, binary_as_string: bool) -> Self {
        self.binary_as_string = Some(Value::Boolean(binary_as_string));
        self
    }

    pub fn with_can_have_nan(mut self, can_have_nan: bool) -> Self {
        self.can_have_nan = Some(Value::Boolean(can_have_nan));
        self
    }

    pub fn with_int96_as(mut self, int96_as: Int96As) -> Self {
        self.int96_as = Some(Value::Text(int96_as.as_sql().to_owned()));
        self
    }

    pub fn with_utf8_validation(mut self, utf8_validation: Utf8Validation) -> Self {
        self.utf8_validation = Some(Value::Text(utf8_validation.as_sql().to_owned()));
        self
    }

    pub fn with_encryption_config(mut self, footer_key: impl Into<String>) -> Self {
        self.encryption_config =
            pairs_to_duck_db_value(vec![("footer_key".to_owned(), footer_key.into())]);
        self
    }

    pub fn with_explicit_cardinality(mut self, explicit_cardinality: u64) -> Self {
        self.explicit_cardinality = Some(Value::UBigInt(explicit_cardinality));
        self
    }

    pub fn with_prefetch_strategy(mut self, prefetch_strategy: PrefetchStrategy) -> Self {
        self.prefetch_strategy = Some(Value::Text(prefetch_strategy.as_sql().to_owned()));
        self
    }

    pub fn with_filename(mut self, filename: bool) -> Self {
        self.filename = Some(Value::Boolean(filename));
        self
    }

    pub fn with_file_row_number(mut self, file_row_number: bool) -> Self {
        self.file_row_number = Some(Value::Boolean(file_row_number));
        self
    }

    pub fn with_union_by_name(mut self, union_by_name: bool) -> Self {
        self.union_by_name = Some(Value::Boolean(union_by_name));
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

        let mut sql = format!(
            "CREATE TABLE {} AS SELECT * FROM read_parquet(?",
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
            ("binary_as_string", &self.binary_as_string),
            ("can_have_nan", &self.can_have_nan),
            ("int96_as", &self.int96_as),
            ("utf8_validation", &self.utf8_validation),
            ("encryption_config", &self.encryption_config),
            ("explicit_cardinality", &self.explicit_cardinality),
            ("prefetch_strategy", &self.prefetch_strategy),
            ("filename", &self.filename),
            ("file_row_number", &self.file_row_number),
            ("union_by_name", &self.union_by_name),
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Int96As {
    Timestamp,
    TimestampNs,
    Struct,
}

impl Int96As {
    pub fn as_sql(&self) -> &'static str {
        match self {
            Int96As::Timestamp => "timestamp",
            Int96As::TimestampNs => "timestamp_ns",
            Int96As::Struct => "struct",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Utf8Validation {
    Strict,
    Replace,
    Ignore,
}

impl Utf8Validation {
    pub fn as_sql(&self) -> &'static str {
        match self {
            Utf8Validation::Strict => "strict",
            Utf8Validation::Replace => "replace",
            Utf8Validation::Ignore => "ignore",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrefetchStrategy {
    Auto,
    WholeGroup,
}

impl PrefetchStrategy {
    pub fn as_sql(&self) -> &'static str {
        match self {
            PrefetchStrategy::Auto => "auto",
            PrefetchStrategy::WholeGroup => "whole_group",
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{
        path::{Path, PathBuf},
        time::{Duration, Instant},
    };

    use duckdb::types::OrderedMap;

    use super::*;
    use crate::sql::{
        db::{DatabaseConfig, ValueRef},
        ops::{Next, extensions::IsInstalled},
    };

    fn db() -> Database {
        DatabaseConfig::default().open_in_memory().unwrap()
    }

    /// Writes `n` rows (n BIGINT, s VARCHAR) to a parquet file using DuckDB itself.
    fn write_parquet(dir: &Path, name: &str, n: usize) -> PathBuf {
        let path = dir.join(name);
        db().execute(
            &format!(
                "COPY (SELECT range AS n, 'v' || range AS s FROM range({n})) TO '{}' (FORMAT parquet)",
                path.display()
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

    /// parquet is statically linked, so the check always yields a token.
    fn token() -> Token<extensions::Parquet> {
        wait(&IsInstalled::new().run::<extensions::Parquet>())
            .unwrap()
            .expect("parquet is built in")
    }

    #[test]
    fn enums_as_sql() {
        assert_eq!(Int96As::Timestamp.as_sql(), "timestamp");
        assert_eq!(Int96As::TimestampNs.as_sql(), "timestamp_ns");
        assert_eq!(Int96As::Struct.as_sql(), "struct");
        assert_eq!(Utf8Validation::Strict.as_sql(), "strict");
        assert_eq!(Utf8Validation::Replace.as_sql(), "replace");
        assert_eq!(Utf8Validation::Ignore.as_sql(), "ignore");
        assert_eq!(PrefetchStrategy::Auto.as_sql(), "auto");
        assert_eq!(PrefetchStrategy::WholeGroup.as_sql(), "whole_group");
    }

    #[test]
    fn options_are_empty_by_default() {
        assert!(Parquet::new(token()).options().is_empty());
    }

    #[test]
    fn options_follow_setters() {
        let parquet = Parquet::new(token())
            .with_binary_as_string(true)
            .with_int96_as(Int96As::TimestampNs)
            .with_encryption_config("key1")
            .with_explicit_cardinality(10)
            .with_file_row_number(true);
        assert_eq!(
            parquet.options(),
            vec![
                ("binary_as_string", Value::Boolean(true)),
                ("int96_as", Value::Text("timestamp_ns".to_owned())),
                (
                    "encryption_config",
                    Value::Struct(OrderedMap::from(vec![(
                        "footer_key".to_owned(),
                        Value::Text("key1".to_owned())
                    )]))
                ),
                ("explicit_cardinality", Value::UBigInt(10)),
                ("file_row_number", Value::Boolean(true)),
            ]
        );
    }

    #[test]
    fn import_reads_rows() {
        let dir = tempfile::tempdir().unwrap();
        let path = write_parquet(dir.path(), "numbers.parquet", 5);
        let mut db = db();
        Parquet::new(token()).import(&mut db, &path).unwrap();
        assert_eq!(text(&db, "SELECT count(*)::VARCHAR FROM numbers"), "5");
        assert_eq!(
            text(&db, "SELECT string_agg(s, ',' ORDER BY n) FROM numbers"),
            "v0,v1,v2,v3,v4"
        );
    }

    #[test]
    fn import_adds_file_row_number_and_filename() {
        let dir = tempfile::tempdir().unwrap();
        let path = write_parquet(dir.path(), "extra.parquet", 3);
        let mut db = db();
        Parquet::new(token())
            .with_file_row_number(true)
            .with_filename(true)
            .import(&mut db, &path)
            .unwrap();
        assert_eq!(
            text(&db, "SELECT max(file_row_number)::VARCHAR FROM extra"),
            "2"
        );
        assert!(text(&db, "SELECT any_value(filename) FROM extra").ends_with("extra.parquet"));
    }

    #[test]
    fn import_missing_file_fails() {
        let dir = tempfile::tempdir().unwrap();
        let error = Parquet::new(token())
            .import(&mut db(), &dir.path().join("missing.parquet"))
            .unwrap_err();
        assert!(error.to_string().contains("No files found"), "{error}");
    }

    #[test]
    fn run_imports_through_the_worker() {
        let dir = tempfile::tempdir().unwrap();
        let path = write_parquet(dir.path(), "parquet_run_through_worker.parquet", 2);
        wait(&Parquet::new(token()).run(&path)).unwrap();
    }
}
