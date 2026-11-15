use std::path::Path;

use duckdb::types::Value;

use crate::{
    AppResult,
    sql::{
        db::Database,
        ops::{
            extensions::{self, Token},
            importers::Compression,
            reply::{Receiver, channel},
            util::{pairs_to_duck_db_value, quote_ident, table_name, utf8_path},
        },
        worker::submit,
    },
};

#[derive(Debug, Clone)]
pub struct Json {
    format: Option<Value>,
    records: Option<Value>,
    compression: Option<Value>,
    ignore_errors: Option<Value>,
    maximum_object_size: Option<Value>,

    columns: Option<Value>,
    date_format: Option<Value>,
    timestamp_format: Option<Value>,

    auto_detect: Option<Value>,
    sample_size: Option<Value>,
    maximum_sample_files: Option<Value>,
    maximum_depth: Option<Value>,
    field_appearance_threshold: Option<Value>,
    convert_strings_to_integers: Option<Value>,
    map_inference_threshold: Option<Value>,

    geojson: Option<Value>,

    filename: Option<Value>,
    file_row_number: Option<Value>,
    union_by_name: Option<Value>,
    hive_partitioning: Option<Value>,
    hive_types: Option<Value>,
    hive_types_autocast: Option<Value>,
    allow_empty: Option<Value>,
}

impl Json {
    pub fn new(_token: Token<extensions::Json>) -> Self {
        Self {
            format: None,
            records: None,
            compression: None,
            ignore_errors: None,
            maximum_object_size: None,
            columns: None,
            date_format: None,
            timestamp_format: None,
            auto_detect: None,
            sample_size: None,
            maximum_sample_files: None,
            maximum_depth: None,
            field_appearance_threshold: None,
            convert_strings_to_integers: None,
            map_inference_threshold: None,
            geojson: None,
            filename: None,
            file_row_number: None,
            union_by_name: None,
            hive_partitioning: None,
            hive_types: None,
            hive_types_autocast: None,
            allow_empty: None,
        }
    }

    pub fn with_format(mut self, format: JsonFormat) -> Self {
        self.format = Some(Value::Text(format.as_sql().to_owned()));
        self
    }

    pub fn with_records(mut self, records: bool) -> Self {
        self.records = Some(Value::Boolean(records));
        self
    }

    pub fn with_compression(mut self, compression: Compression) -> Self {
        self.compression = Some(Value::Text(compression.as_sql().to_owned()));
        self
    }

    pub fn with_ignore_errors(mut self, ignore_errors: bool) -> Self {
        self.ignore_errors = Some(Value::Boolean(ignore_errors));
        self
    }

    pub fn with_maximum_object_size(mut self, maximum_object_size: u32) -> Self {
        self.maximum_object_size = Some(Value::UInt(maximum_object_size));
        self
    }

    pub fn with_columns(mut self, columns: Vec<(String, String)>) -> Self {
        self.columns = pairs_to_duck_db_value(columns);
        self
    }

    pub fn with_date_format(mut self, date_format: impl Into<String>) -> Self {
        self.date_format = Some(Value::Text(date_format.into()));
        self
    }

    pub fn with_timestamp_format(mut self, timestamp_format: impl Into<String>) -> Self {
        self.timestamp_format = Some(Value::Text(timestamp_format.into()));
        self
    }

    pub fn with_auto_detect(mut self, auto_detect: bool) -> Self {
        self.auto_detect = Some(Value::Boolean(auto_detect));
        self
    }

    pub fn with_sample_size(mut self, sample_size: i64) -> Self {
        self.sample_size = Some(Value::BigInt(sample_size));
        self
    }

    pub fn with_maximum_sample_files(mut self, maximum_sample_files: i64) -> Self {
        self.maximum_sample_files = Some(Value::BigInt(maximum_sample_files));
        self
    }

    pub fn with_maximum_depth(mut self, maximum_depth: i64) -> Self {
        self.maximum_depth = Some(Value::BigInt(maximum_depth));
        self
    }

    pub fn with_field_appearance_threshold(mut self, field_appearance_threshold: f64) -> Self {
        self.field_appearance_threshold = Some(Value::Double(field_appearance_threshold));
        self
    }

    pub fn with_convert_strings_to_integers(mut self, convert_strings_to_integers: bool) -> Self {
        self.convert_strings_to_integers = Some(Value::Boolean(convert_strings_to_integers));
        self
    }

    pub fn with_map_inference_threshold(mut self, map_inference_threshold: i64) -> Self {
        self.map_inference_threshold = Some(Value::BigInt(map_inference_threshold));
        self
    }

    pub fn with_geojson(mut self, geojson: bool) -> Self {
        self.geojson = Some(Value::Boolean(geojson));
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
            "CREATE TABLE {} AS SELECT * FROM read_json(?",
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
            ("format", &self.format),
            ("records", &self.records),
            ("compression", &self.compression),
            ("ignore_errors", &self.ignore_errors),
            ("maximum_object_size", &self.maximum_object_size),
            ("columns", &self.columns),
            ("date_format", &self.date_format),
            ("timestamp_format", &self.timestamp_format),
            ("auto_detect", &self.auto_detect),
            ("sample_size", &self.sample_size),
            ("maximum_sample_files", &self.maximum_sample_files),
            ("maximum_depth", &self.maximum_depth),
            (
                "field_appearance_threshold",
                &self.field_appearance_threshold,
            ),
            (
                "convert_strings_to_integers",
                &self.convert_strings_to_integers,
            ),
            ("map_inference_threshold", &self.map_inference_threshold),
            ("geojson", &self.geojson),
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum JsonFormat {
    #[default]
    Auto,
    Array,
    NewlineDelimited,
    Unstructured,
}

impl JsonFormat {
    pub fn as_sql(&self) -> &'static str {
        match self {
            JsonFormat::Auto => "auto",
            JsonFormat::Array => "array",
            JsonFormat::NewlineDelimited => "newline_delimited",
            JsonFormat::Unstructured => "unstructured",
        }
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
        ops::{Next, extensions::IsInstalled},
    };

    fn db() -> Database {
        DatabaseConfig::default().open_in_memory().unwrap()
    }

    fn write(dir: &Path, name: &str, contents: &str) -> PathBuf {
        let path = dir.join(name);
        std::fs::write(&path, contents).unwrap();
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

    /// json is statically linked, so the check always yields a token.
    fn token() -> Token<extensions::Json> {
        wait(&IsInstalled::new().run::<extensions::Json>())
            .unwrap()
            .expect("json is built in")
    }

    #[test]
    fn json_format_as_sql() {
        assert_eq!(JsonFormat::Auto.as_sql(), "auto");
        assert_eq!(JsonFormat::Array.as_sql(), "array");
        assert_eq!(JsonFormat::NewlineDelimited.as_sql(), "newline_delimited");
        assert_eq!(JsonFormat::Unstructured.as_sql(), "unstructured");
    }

    #[test]
    fn options_are_empty_by_default() {
        assert!(Json::new(token()).options().is_empty());
    }

    #[test]
    fn options_follow_setters() {
        let json = Json::new(token())
            .with_format(JsonFormat::NewlineDelimited)
            .with_records(true)
            .with_compression(Compression::Zstd)
            .with_maximum_object_size(1024);
        assert_eq!(
            json.options(),
            vec![
                ("format", Value::Text("newline_delimited".to_owned())),
                ("records", Value::Boolean(true)),
                ("compression", Value::Text("zstd".to_owned())),
                ("maximum_object_size", Value::UInt(1024)),
            ]
        );
    }

    #[test]
    fn empty_columns_are_left_out() {
        assert!(Json::new(token()).with_columns(vec![]).options().is_empty());
    }

    #[test]
    fn import_array() {
        let dir = tempfile::tempdir().unwrap();
        let path = write(
            dir.path(),
            "people.json",
            r#"[{"id": 1, "name": "a"}, {"id": 2, "name": "b"}, {"id": 3, "name": null}]"#,
        );
        let mut db = db();
        Json::new(token()).import(&mut db, &path).unwrap();
        assert_eq!(text(&db, "SELECT count(*)::VARCHAR FROM people"), "3");
        assert_eq!(text(&db, "SELECT count(name)::VARCHAR FROM people"), "2");
        assert_eq!(
            text(&db, "SELECT any_value(typeof(id)) FROM people"),
            "BIGINT"
        );
    }

    #[test]
    fn import_newline_delimited_with_columns() {
        let dir = tempfile::tempdir().unwrap();
        let path = write(
            dir.path(),
            "events.jsonl",
            "{\"id\": \"1\", \"tags\": [\"x\"]}\n{\"id\": \"2\", \"tags\": []}\n",
        );
        let mut db = db();
        Json::new(token())
            .with_format(JsonFormat::NewlineDelimited)
            .with_columns(vec![
                ("id".to_owned(), "BIGINT".to_owned()),
                ("tags".to_owned(), "VARCHAR[]".to_owned()),
            ])
            .import(&mut db, &path)
            .unwrap();
        assert_eq!(text(&db, "SELECT count(*)::VARCHAR FROM events"), "2");
        assert_eq!(
            text(&db, "SELECT any_value(typeof(id)) FROM events"),
            "BIGINT"
        );
        assert_eq!(
            text(&db, "SELECT any_value(typeof(tags)) FROM events"),
            "VARCHAR[]"
        );
    }

    #[test]
    fn import_without_records_keeps_objects_whole() {
        let dir = tempfile::tempdir().unwrap();
        let path = write(dir.path(), "whole.json", r#"[{"id": 1, "name": "a"}]"#);
        let mut db = db();
        Json::new(token())
            .with_records(false)
            .import(&mut db, &path)
            .unwrap();
        assert!(text(&db, "SELECT any_value(typeof(json)) FROM whole").starts_with("STRUCT"));
    }

    #[test]
    fn import_missing_file_fails() {
        let dir = tempfile::tempdir().unwrap();
        let error = Json::new(token())
            .import(&mut db(), &dir.path().join("missing.json"))
            .unwrap_err();
        assert!(error.to_string().contains("No files found"), "{error}");
    }

    #[test]
    fn run_imports_through_the_worker() {
        let dir = tempfile::tempdir().unwrap();
        let path = write(dir.path(), "json_run_through_worker.json", r#"[{"a": 1}]"#);
        wait(&Json::new(token()).run(&path)).unwrap();
    }
}
