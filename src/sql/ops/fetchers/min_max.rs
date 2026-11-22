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

#[derive(Debug, Clone)]
pub struct MinMax {
    table: String,
    column: String,
}

impl MinMax {
    pub fn new(table: impl Into<String>, column: impl Into<String>) -> Self {
        Self {
            table: table.into(),
            column: column.into(),
        }
    }

    pub fn run(&self) -> Receiver<(Value, Value)> {
        let this = self.clone();
        let (send, recv) = channel();
        submit(move |db| send.send(this.fetch(db)));
        recv
    }

    fn fetch(&self, db: &Database) -> AppResult<(Value, Value)> {
        let column = quote_ident(&self.column);
        let mut stmt = db.prepare(&format!(
            "SELECT MIN({column}), MAX({column}) FROM {}",
            quote_ident(&self.table)
        ))?;
        let mut rows = stmt.query([])?;
        let row = rows
            .next()?
            .ok_or_else(|| anyhow!("min max query returned no rows"))?;
        Ok((row.get_ref(0)?.into(), row.get_ref(1)?.into()))
    }
}

#[cfg(test)]
mod tests {
    use std::time::{Duration, Instant};

    use super::*;
    use crate::sql::{db::DatabaseConfig, ops::Next};

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
        let min_max = MinMax::new("t", "n").fetch(&db()).unwrap();
        assert_eq!(min_max, (Value::Int(1), Value::Int(3)));
    }

    #[test]
    fn text_is_ordered_lexically() {
        let min_max = MinMax::new("t", "s").fetch(&db()).unwrap();
        assert_eq!(min_max, (Value::Text("a".into()), Value::Text("c".into())));
    }

    #[test]
    fn dates_keep_their_type() {
        let (min, max) = MinMax::new("t", "d").fetch(&db()).unwrap();
        assert!(matches!(min, Value::Date32(_)));
        assert!(matches!(max, Value::Date32(_)));
        assert_ne!(min, max);
    }

    #[test]
    fn all_null_column_is_null() {
        let min_max = MinMax::new("t", "e").fetch(&db()).unwrap();
        assert_eq!(min_max, (Value::Null, Value::Null));
    }

    #[test]
    fn empty_table_is_null() {
        let db = db();
        db.execute("CREATE TABLE empty (n INTEGER)", []).unwrap();
        let min_max = MinMax::new("empty", "n").fetch(&db).unwrap();
        assert_eq!(min_max, (Value::Null, Value::Null));
    }

    #[test]
    fn quoted_identifiers_work() {
        let db = db();
        db.execute(
            "CREATE TABLE \"my \"\"table\" AS SELECT range AS \"a b\" FROM range(5)",
            [],
        )
        .unwrap();
        let min_max = MinMax::new("my \"table", "a b").fetch(&db).unwrap();
        assert_eq!(min_max, (Value::BigInt(0), Value::BigInt(4)));
    }

    #[test]
    fn missing_column_fails() {
        assert!(MinMax::new("t", "missing").fetch(&db()).is_err());
    }

    #[test]
    fn missing_table_fails() {
        assert!(MinMax::new("missing", "n").fetch(&db()).is_err());
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
        let min_max = wait(&MinMax::new("min_max_run", "n").run()).unwrap();
        assert_eq!(min_max, (Value::BigInt(0), Value::BigInt(3)));
    }
}
