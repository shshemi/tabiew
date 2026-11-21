use duckdb::types::Value;

use crate::{
    AppResult,
    sql::{
        db::Database,
        ops::reply::{Receiver, channel},
        worker::submit,
    },
};

#[derive(Debug, Clone)]
pub struct TextGrid {
    sql: String,
    offset: Option<u64>,
    limit: Option<u64>,
}

impl TextGrid {
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

    pub fn run(&self) -> Receiver<crate::sql::collections::TextGrid> {
        let this = self.clone();
        let (send, recv) = channel();
        submit(move |db| send.send(this.fetch(db)));
        recv
    }

    fn fetch(&self, db: &Database) -> AppResult<crate::sql::collections::TextGrid> {
        let mut sql = format!("SELECT * FROM ({})", self.sql.trim().trim_end_matches(';'));
        let mut params = Vec::new();
        if let Some(limit) = self.limit {
            sql.push_str(" LIMIT ?");
            params.push(Value::UBigInt(limit));
        }
        if let Some(offset) = self.offset {
            sql.push_str(" OFFSET ?");
            params.push(Value::UBigInt(offset));
        }

        let mut stmt = db.prepare(&sql)?;
        let mut rows = stmt.query(duckdb::params_from_iter(params))?;
        let columns = rows.column_count();
        let first_row = self.offset.unwrap_or(0) as usize;
        let mut pool = crate::sql::collections::TextGrid::default();
        let mut row_idx = first_row;
        while let Some(row) = rows.next()? {
            for col in 0..columns {
                let value = row.get_ref(col)?;
                pool.insert(row_idx, col, value);
            }
            row_idx += 1;
        }
        Ok(pool)
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

    #[test]
    fn columns_are_formatted_by_type() {
        let pool =
            TextGrid::new("SELECT 1 AS a, 'x' AS b, 2.5 AS c, DATE '2024-03-05' AS d, [1, 2] AS e")
                .fetch(&db())
                .unwrap();
        assert_eq!(pool.get(0, 0), Some("1"));
        assert_eq!(pool.get(0, 1), Some("x"));
        assert_eq!(pool.get(0, 2), Some("2.5"));
        assert_eq!(pool.get(0, 3), Some("2024-03-05"));
        assert_eq!(pool.get(0, 4), Some("[2 items]"));
    }

    #[test]
    fn values_are_stored_by_row_and_column() {
        let pool = TextGrid::new("SELECT range AS n, range * 10 AS m FROM range(3)")
            .fetch(&db())
            .unwrap();
        assert_eq!(pool.count(), 6);
        assert_eq!(pool.get(2, 0), Some("2"));
        assert_eq!(pool.get(2, 1), Some("20"));
        assert_eq!(pool.get(3, 0), None);
    }

    #[test]
    fn nulls_are_empty() {
        let pool = TextGrid::new("SELECT NULL::INTEGER AS a, 1 AS b")
            .fetch(&db())
            .unwrap();
        assert_eq!(pool.count(), 2);
        assert_eq!(pool.get(0, 0), Some(""));
        assert_eq!(pool.get(0, 1), Some("1"));
    }

    #[test]
    fn limit_and_offset_keep_absolute_row_numbers() {
        let pool = TextGrid::new("SELECT range FROM range(10)")
            .with_offset(3)
            .with_limit(2)
            .fetch(&db())
            .unwrap();
        assert_eq!(pool.count(), 2);
        assert_eq!(pool.get(3, 0), Some("3"));
        assert_eq!(pool.get(4, 0), Some("4"));
        assert_eq!(pool.get(0, 0), None);
    }

    #[test]
    fn trailing_semicolon_is_ignored() {
        let pool = TextGrid::new("SELECT 42;  ").fetch(&db()).unwrap();
        assert_eq!(pool.get(0, 0), Some("42"));
    }

    #[test]
    fn empty_result_is_an_empty_pool() {
        let pool = TextGrid::new("SELECT 1 WHERE false").fetch(&db()).unwrap();
        assert_eq!(pool.count(), 0);
    }

    #[test]
    fn invalid_sql_fails() {
        assert!(TextGrid::new("SELEC 1").fetch(&db()).is_err());
    }

    #[test]
    fn run_returns_the_pool_through_the_worker() {
        let pool = wait(&TextGrid::new("SELECT 'hello'").run()).unwrap();
        assert_eq!(pool.get(0, 0), Some("hello"));
    }
}
