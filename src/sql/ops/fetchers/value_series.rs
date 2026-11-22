use std::collections::BTreeMap;

use crate::{
    AppResult,
    sql::{
        db::{Database, Value},
        ops::reply::{Receiver, channel},
        worker::submit,
    },
};

#[derive(Debug, Default)]
pub struct ValueSeries {
    idx: BTreeMap<usize, Value>,
}

impl ValueSeries {
    pub fn get(&self, row: usize) -> Option<&Value> {
        self.idx.get(&row)
    }

    pub fn insert(&mut self, row: usize, value: impl Into<Value>) -> bool {
        self.idx.insert(row, value.into()).is_some()
    }

    pub fn len(&self) -> usize {
        self.idx.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn mem_usage(&self) -> usize {
        self.idx.len() * size_of::<(usize, Value)>()
            + self.idx.values().map(heap_size).sum::<usize>()
    }

    pub fn merge(&mut self, other: ValueSeries) {
        self.idx.extend(other.idx);
    }
}

fn heap_size(value: &Value) -> usize {
    match value {
        Value::Null
        | Value::Boolean(_)
        | Value::TinyInt(_)
        | Value::SmallInt(_)
        | Value::Int(_)
        | Value::BigInt(_)
        | Value::HugeInt(_)
        | Value::UHugeInt(_)
        | Value::UTinyInt(_)
        | Value::USmallInt(_)
        | Value::UInt(_)
        | Value::UBigInt(_)
        | Value::Float(_)
        | Value::Double(_)
        | Value::Decimal(_)
        | Value::Timestamp(..)
        | Value::Date32(_)
        | Value::Time64(..)
        | Value::Interval { .. } => 0,
        Value::Text(s) | Value::Enum(s) => s.capacity(),
        Value::Blob(b) | Value::Geometry(b) => b.capacity(),
        Value::List(items) | Value::Array(items) => {
            items.capacity() * size_of::<Value>() + items.iter().map(heap_size).sum::<usize>()
        }
        Value::Struct(fields) => {
            fields.capacity() * size_of::<(String, Value)>()
                + fields
                    .iter()
                    .map(|(name, value)| name.capacity() + heap_size(value))
                    .sum::<usize>()
        }
        Value::Map(entries) => {
            entries.capacity() * size_of::<(Value, Value)>()
                + entries
                    .iter()
                    .map(|(key, value)| heap_size(key) + heap_size(value))
                    .sum::<usize>()
        }
        Value::Union(inner) => size_of::<Value>() + heap_size(inner),
    }
}

#[derive(Debug, Clone)]
pub struct Fetcher {
    sql: String,
    offset: Option<u64>,
    limit: Option<u64>,
}

impl Fetcher {
    pub fn new(sql: impl Into<String>) -> Self {
        Self {
            sql: sql.into(),
            offset: None,
            limit: None,
        }
    }

    pub fn with_offset(mut self, offset: u64) -> Self {
        self.offset = Some(offset);
        self
    }

    pub fn with_limit(mut self, limit: u64) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn run(&self) -> Receiver<ValueSeries> {
        let this = self.clone();
        let (send, recv) = channel();
        submit(move |db| send.send(this.fetch(db)));
        recv
    }

    fn fetch(&self, db: &Database) -> AppResult<ValueSeries> {
        let mut sql = format!("SELECT * FROM ({})", self.sql.trim().trim_end_matches(';'));
        let mut params = Vec::new();
        if let Some(limit) = self.limit {
            sql.push_str(" LIMIT ?");
            params.push(duckdb::types::Value::UBigInt(limit));
        }
        if let Some(offset) = self.offset {
            sql.push_str(" OFFSET ?");
            params.push(duckdb::types::Value::UBigInt(offset));
        }

        let mut stmt = db.prepare(&sql)?;
        let mut rows = stmt.query(duckdb::params_from_iter(params))?;
        let names = rows.column_names();
        let mut series = ValueSeries::default();
        let mut row_idx = self.offset.unwrap_or(0) as usize;
        while let Some(row) = rows.next()? {
            let mut fields = Vec::with_capacity(names.len());
            for (col, name) in names.iter().enumerate() {
                fields.push((name.clone(), row.get_ref(col)?.into()));
            }
            series.insert(row_idx, Value::Struct(fields));
            row_idx += 1;
        }
        Ok(series)
    }
}

#[cfg(test)]
mod tests {
    use std::time::{Duration, Instant};

    use super::*;
    use crate::sql::{
        db::{DatabaseConfig, ValueRef},
        ops::Next,
    };

    fn insert_sql(expr: &str) -> Value {
        let db = DatabaseConfig::default().open_in_memory().unwrap();
        let mut stmt = db.prepare(&format!("SELECT {expr}")).unwrap();
        let mut rows = stmt.query([]).unwrap();
        let row = rows.next().unwrap().unwrap();
        let mut series = ValueSeries::default();
        series.insert(0, row.get_ref(0).unwrap());
        series.get(0).unwrap().clone()
    }

    #[test]
    fn scalars() {
        assert_eq!(insert_sql("42"), Value::Int(42));
        assert_eq!(insert_sql("true"), Value::Boolean(true));
        assert_eq!(insert_sql("'x'"), Value::Text("x".into()));
        assert_eq!(insert_sql("NULL::INTEGER"), Value::Null);
    }

    #[test]
    fn composites() {
        assert_eq!(
            insert_sql("[1, 2]"),
            Value::List(vec![Value::Int(1), Value::Int(2)])
        );
        assert_eq!(
            insert_sql("{'a': 1, 'b': 'x'}"),
            Value::Struct(vec![
                ("a".into(), Value::Int(1)),
                ("b".into(), Value::Text("x".into())),
            ])
        );
        assert_eq!(
            insert_sql("MAP {'k': 1}"),
            Value::Map(vec![(Value::Text("k".into()), Value::Int(1))])
        );
    }

    #[test]
    fn insert_accepts_value_and_value_ref() {
        let mut series = ValueSeries::default();
        series.insert(0, Value::Int(1));
        series.insert(1, ValueRef::Text(b"a"));
        assert_eq!(series.get(0), Some(&Value::Int(1)));
        assert_eq!(series.get(1), Some(&Value::Text("a".into())));
        assert_eq!(series.get(2), None);
        assert_eq!(series.len(), 2);
    }

    #[test]
    fn insert_reports_overwrites() {
        let mut series = ValueSeries::default();
        assert!(!series.insert(0, Value::Int(1)));
        assert!(series.insert(0, Value::Int(2)));
        assert_eq!(series.get(0), Some(&Value::Int(2)));
    }

    #[test]
    fn merge_combines_rows() {
        let mut a = ValueSeries::default();
        a.insert(0, Value::Int(1));
        a.insert(1, Value::Int(2));
        let mut b = ValueSeries::default();
        b.insert(1, Value::Int(20));
        b.insert(2, Value::Int(30));
        a.merge(b);
        assert_eq!(a.get(0), Some(&Value::Int(1)));
        assert_eq!(a.get(1), Some(&Value::Int(20)));
        assert_eq!(a.get(2), Some(&Value::Int(30)));
        assert_eq!(a.len(), 3);
    }

    #[test]
    fn mem_usage_counts_heap_data() {
        let mut small = ValueSeries::default();
        small.insert(0, Value::Text(String::new()));
        let mut large = ValueSeries::default();
        large.insert(0, Value::Text("x".repeat(1000)));
        assert!(large.mem_usage() >= small.mem_usage() + 1000);
    }

    fn db() -> Database {
        DatabaseConfig::default().open_in_memory().unwrap()
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

    fn dict(fields: &[(&str, Value)]) -> Value {
        Value::Struct(
            fields
                .iter()
                .map(|(name, value)| (name.to_string(), value.clone()))
                .collect(),
        )
    }

    #[test]
    fn each_row_is_a_dict_of_its_columns() {
        let series = Fetcher::new("SELECT 1 AS a, 'x' AS b, [1, 2] AS c")
            .fetch(&db())
            .unwrap();
        assert_eq!(
            series.get(0),
            Some(&dict(&[
                ("a", Value::Int(1)),
                ("b", Value::Text("x".into())),
                ("c", Value::List(vec![Value::Int(1), Value::Int(2)])),
            ]))
        );
    }

    #[test]
    fn rows_are_stored_by_row_number() {
        let series = Fetcher::new("SELECT range AS n FROM range(3)")
            .fetch(&db())
            .unwrap();
        assert_eq!(series.len(), 3);
        assert_eq!(series.get(2), Some(&dict(&[("n", Value::BigInt(2))])));
        assert_eq!(series.get(3), None);
    }

    #[test]
    fn nulls_are_kept_in_the_dict() {
        let series = Fetcher::new("SELECT NULL::INTEGER AS a, 1 AS b")
            .fetch(&db())
            .unwrap();
        assert_eq!(
            series.get(0),
            Some(&dict(&[("a", Value::Null), ("b", Value::Int(1))]))
        );
    }

    #[test]
    fn duplicate_column_names_get_unique_keys() {
        let series = Fetcher::new("SELECT 1 AS a, 2 AS a").fetch(&db()).unwrap();
        assert_eq!(
            series.get(0),
            Some(&dict(&[("a", Value::Int(1)), ("a_1", Value::Int(2))]))
        );
    }

    #[test]
    fn limit_and_offset_keep_absolute_row_numbers() {
        let series = Fetcher::new("SELECT range AS n FROM range(10)")
            .with_offset(3)
            .with_limit(2)
            .fetch(&db())
            .unwrap();
        assert_eq!(series.len(), 2);
        assert_eq!(series.get(3), Some(&dict(&[("n", Value::BigInt(3))])));
        assert_eq!(series.get(4), Some(&dict(&[("n", Value::BigInt(4))])));
        assert_eq!(series.get(0), None);
    }

    #[test]
    fn trailing_semicolon_is_ignored() {
        let series = Fetcher::new("SELECT 42 AS n;  ").fetch(&db()).unwrap();
        assert_eq!(series.get(0), Some(&dict(&[("n", Value::Int(42))])));
    }

    #[test]
    fn empty_result_is_an_empty_series() {
        let series = Fetcher::new("SELECT 1 WHERE false").fetch(&db()).unwrap();
        assert!(series.is_empty());
    }

    #[test]
    fn invalid_sql_fails() {
        assert!(Fetcher::new("SELEC 1").fetch(&db()).is_err());
    }

    #[test]
    fn run_returns_the_series_through_the_worker() {
        let series = wait(&Fetcher::new("SELECT 'hello' AS s").run()).unwrap();
        assert_eq!(
            series.get(0),
            Some(&dict(&[("s", Value::Text("hello".into()))]))
        );
    }
}
