use crate::{
    AppResult,
    sql::{
        db::{Database, Value},
        ops::reply::{Receiver, channel},
        worker::submit,
    },
};

#[derive(Debug, Clone)]
pub struct ValueSeries {
    sql: String,
    offset: Option<u64>,
    limit: Option<u64>,
}

impl ValueSeries {
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

    pub fn run(&self) -> Receiver<crate::sql::collections::ValueSeries> {
        let this = self.clone();
        let (send, recv) = channel();
        submit(move |db| send.send(this.fetch(db)));
        recv
    }

    fn fetch(&self, db: &Database) -> AppResult<crate::sql::collections::ValueSeries> {
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
        let mut series = crate::sql::collections::ValueSeries::default();
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
    use crate::sql::{db::DatabaseConfig, ops::Next};

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
        let series = ValueSeries::new("SELECT 1 AS a, 'x' AS b, [1, 2] AS c")
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
        let series = ValueSeries::new("SELECT range AS n FROM range(3)")
            .fetch(&db())
            .unwrap();
        assert_eq!(series.len(), 3);
        assert_eq!(series.get(2), Some(&dict(&[("n", Value::BigInt(2))])));
        assert_eq!(series.get(3), None);
    }

    #[test]
    fn nulls_are_kept_in_the_dict() {
        let series = ValueSeries::new("SELECT NULL::INTEGER AS a, 1 AS b")
            .fetch(&db())
            .unwrap();
        assert_eq!(
            series.get(0),
            Some(&dict(&[("a", Value::Null), ("b", Value::Int(1))]))
        );
    }

    #[test]
    fn duplicate_column_names_get_unique_keys() {
        let series = ValueSeries::new("SELECT 1 AS a, 2 AS a")
            .fetch(&db())
            .unwrap();
        assert_eq!(
            series.get(0),
            Some(&dict(&[("a", Value::Int(1)), ("a_1", Value::Int(2))]))
        );
    }

    #[test]
    fn limit_and_offset_keep_absolute_row_numbers() {
        let series = ValueSeries::new("SELECT range AS n FROM range(10)")
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
        let series = ValueSeries::new("SELECT 42 AS n;  ").fetch(&db()).unwrap();
        assert_eq!(series.get(0), Some(&dict(&[("n", Value::Int(42))])));
    }

    #[test]
    fn empty_result_is_an_empty_series() {
        let series = ValueSeries::new("SELECT 1 WHERE false")
            .fetch(&db())
            .unwrap();
        assert!(series.is_empty());
    }

    #[test]
    fn invalid_sql_fails() {
        assert!(ValueSeries::new("SELEC 1").fetch(&db()).is_err());
    }

    #[test]
    fn run_returns_the_series_through_the_worker() {
        let series = wait(&ValueSeries::new("SELECT 'hello' AS s").run()).unwrap();
        assert_eq!(
            series.get(0),
            Some(&dict(&[("s", Value::Text("hello".into()))]))
        );
    }
}
