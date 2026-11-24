use anyhow::anyhow;

use crate::{
    AppResult,
    sql::{
        db::{Database, Value},
        ops::{
            reply::{Receiver, channel},
            util::quote_ident,
        },
        worker::submit,
    },
};

#[derive(Debug, Clone, PartialEq)]
pub struct MinMax {
    min: Value,
    max: Value,
}

impl MinMax {
    pub fn new(min: Value, max: Value) -> Self {
        Self { min, max }
    }

    pub fn min(&self) -> &Value {
        &self.min
    }

    pub fn max(&self) -> &Value {
        &self.max
    }

    pub fn into_inner(self) -> (Value, Value) {
        (self.min, self.max)
    }
}

#[derive(Debug, Clone)]
pub struct Fetcher {
    sql: String,
    column: String,
}

impl Fetcher {
    pub fn new(sql: impl Into<String>, column: impl Into<String>) -> Self {
        Self {
            sql: sql.into(),
            column: column.into(),
        }
    }

    pub fn run(&self) -> Receiver<MinMax> {
        let this = self.clone();
        let (send, recv) = channel();
        submit(move |db| send.send(this.fetch(db)));
        recv
    }

    pub(super) fn fetch(&self, db: &Database) -> AppResult<MinMax> {
        let sql = self.sql.trim().trim_end_matches(';');
        let column = quote_ident(&self.column);
        let mut stmt = db.prepare(&format!("SELECT MIN({column}), MAX({column}) FROM ({sql})"))?;
        let mut rows = stmt.query([])?;
        let row = rows
            .next()?
            .ok_or_else(|| anyhow!("min max query returned no rows"))?;
        Ok(MinMax::new(row.get_ref(0)?.into(), row.get_ref(1)?.into()))
    }
}

#[cfg(test)]
mod tests {
    use std::time::{Duration, Instant};

    use super::*;
    use crate::sql::{db::DatabaseConfig, ops::Next};

    #[test]
    fn keeps_min_and_max() {
        let min_max = MinMax::new(Value::Int(1), Value::Int(9));
        assert_eq!(min_max.min(), &Value::Int(1));
        assert_eq!(min_max.max(), &Value::Int(9));
    }

    #[test]
    fn into_inner_returns_min_then_max() {
        let min_max = MinMax::new(Value::Int(1), Value::Int(9));
        assert_eq!(min_max.into_inner(), (Value::Int(1), Value::Int(9)));
    }

    fn db() -> Database {
        let db = DatabaseConfig::default().open_in_memory().unwrap();
        db.execute(
            "CREATE TABLE t AS SELECT * FROM (VALUES \
             (3, 'b', DATE '2024-02-01', NULL::INTEGER), \
             (1, 'c', DATE '2024-01-01', NULL::INTEGER), \
             (NULL, 'a', DATE '2024-03-01', NULL::INTEGER)) \
             v(n, s, d, e)",
            [],
        )
        .unwrap();
        db
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

    #[test]
    fn numbers_skip_nulls() {
        let min_max = Fetcher::new("SELECT * FROM t", "n").fetch(&db()).unwrap();
        assert_eq!(min_max.into_inner(), (Value::Int(1), Value::Int(3)));
    }

    #[test]
    fn text_is_ordered_lexically() {
        let min_max = Fetcher::new("SELECT * FROM t", "s").fetch(&db()).unwrap();
        assert_eq!(
            min_max.into_inner(),
            (Value::Text("a".into()), Value::Text("c".into()))
        );
    }

    #[test]
    fn dates_keep_their_type() {
        let min_max = Fetcher::new("SELECT * FROM t", "d").fetch(&db()).unwrap();
        assert!(matches!(min_max.min(), Value::Date32(_)));
        assert!(matches!(min_max.max(), Value::Date32(_)));
        assert_ne!(min_max.min(), min_max.max());
    }

    #[test]
    fn all_null_column_is_null() {
        let min_max = Fetcher::new("SELECT * FROM t", "e").fetch(&db()).unwrap();
        assert_eq!(min_max.into_inner(), (Value::Null, Value::Null));
    }

    #[test]
    fn empty_table_is_null() {
        let db = db();
        db.execute("CREATE TABLE empty (n INTEGER)", []).unwrap();
        let min_max = Fetcher::new("SELECT * FROM empty", "n").fetch(&db).unwrap();
        assert_eq!(min_max.into_inner(), (Value::Null, Value::Null));
    }

    #[test]
    fn quoted_identifiers_work() {
        let db = db();
        db.execute(
            "CREATE TABLE \"my \"\"table\" AS SELECT range AS \"a b\" FROM range(5)",
            [],
        )
        .unwrap();
        let min_max = Fetcher::new("SELECT * FROM \"my \"\"table\"", "a b")
            .fetch(&db)
            .unwrap();
        assert_eq!(min_max.into_inner(), (Value::BigInt(0), Value::BigInt(4)));
    }

    #[test]
    fn missing_column_fails() {
        assert!(
            Fetcher::new("SELECT * FROM t", "missing")
                .fetch(&db())
                .is_err()
        );
    }

    #[test]
    fn invalid_sql_fails() {
        assert!(Fetcher::new("SELEC 1", "n").fetch(&db()).is_err());
    }

    #[test]
    fn expressions_and_table_functions_work() {
        let min_max = Fetcher::new("SELECT range * 2 AS doubled FROM range(5)", "doubled")
            .fetch(&db())
            .unwrap();
        assert_eq!(min_max.into_inner(), (Value::BigInt(0), Value::BigInt(8)));
    }

    #[test]
    fn trailing_semicolon_is_ignored() {
        let min_max = Fetcher::new("SELECT 42 AS n;  ", "n").fetch(&db()).unwrap();
        assert_eq!(min_max.into_inner(), (Value::Int(42), Value::Int(42)));
    }

    #[test]
    fn run_returns_min_max_through_the_worker() {
        let (send, recv) = channel();
        submit(move |db| {
            send.send(db.execute(
                "CREATE OR REPLACE TABLE min_max_run AS SELECT range AS n FROM range(4)",
                [],
            ))
        });
        wait(&recv).unwrap();
        let min_max = wait(&Fetcher::new("SELECT * FROM min_max_run", "n").run()).unwrap();
        assert_eq!(min_max.into_inner(), (Value::BigInt(0), Value::BigInt(3)));
    }
}
