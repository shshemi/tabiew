use anyhow::anyhow;

use crate::{
    AppResult,
    sql::{
        db::{Database, ValueRef},
        ops::reply::{Receiver, channel},
        worker::submit,
    },
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Shape {
    height: usize,
    width: usize,
}

impl Shape {
    pub fn new(height: usize, width: usize) -> Self {
        Self { height, width }
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn numel(&self) -> usize {
        self.height * self.width
    }
}

#[derive(Debug, Clone)]
pub struct Fetcher {
    sql: String,
}

impl Fetcher {
    pub fn new(sql: impl Into<String>) -> Self {
        Self { sql: sql.into() }
    }

    pub fn run(&self) -> Receiver<Shape> {
        let this = self.clone();
        let (send, recv) = channel();
        submit(move |db| send.send(this.fetch(db)));
        recv
    }

    fn fetch(&self, db: &Database) -> AppResult<Shape> {
        let sql = self.sql.trim().trim_end_matches(';');
        Ok(Shape::new(self.height(db, sql)?, self.width(db, sql)?))
    }

    fn height(&self, db: &Database, sql: &str) -> AppResult<usize> {
        let mut stmt = db.prepare(&format!("SELECT COUNT(*) FROM ({sql})"))?;
        let mut rows = stmt.query([])?;
        let row = rows
            .next()?
            .ok_or_else(|| anyhow!("row count query returned no rows"))?;
        match row.get_ref(0)? {
            ValueRef::BigInt(count) => Ok(usize::try_from(count)?),
            other => Err(anyhow!("unexpected row count: {other:?}")),
        }
    }

    fn width(&self, db: &Database, sql: &str) -> AppResult<usize> {
        let mut stmt = db.prepare(&format!("SELECT * FROM ({sql}) LIMIT 0"))?;
        let rows = stmt.query([])?;
        Ok(rows.column_count())
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
    fn numel_is_height_times_width() {
        assert_eq!(Shape::new(4, 3).numel(), 12);
        assert_eq!(Shape::new(0, 3).numel(), 0);
    }

    #[test]
    fn shape_counts_rows_and_columns() {
        let shape = Fetcher::new("SELECT range AS a, range * 2 AS b, 'x' AS c FROM range(7)")
            .fetch(&db())
            .unwrap();
        assert_eq!(shape.height(), 7);
        assert_eq!(shape.width(), 3);
    }

    #[test]
    fn empty_result_has_columns_but_no_rows() {
        let shape = Fetcher::new("SELECT 1 AS a, 2 AS b WHERE false")
            .fetch(&db())
            .unwrap();
        assert_eq!(shape.height(), 0);
        assert_eq!(shape.width(), 2);
    }

    #[test]
    fn duplicate_column_names_are_counted() {
        let shape = Fetcher::new("SELECT 1 AS a, 2 AS a").fetch(&db()).unwrap();
        assert_eq!(shape.height(), 1);
        assert_eq!(shape.width(), 2);
    }

    #[test]
    fn shape_matches_the_text_grid() {
        let sql = "SELECT range AS a, range * 10 AS b FROM range(4)";
        let shape = Fetcher::new(sql).fetch(&db()).unwrap();
        let grid = crate::sql::ops::fetchers::text_grid::Fetcher::new(sql).run();
        let grid = wait(&grid).unwrap();
        assert_eq!(grid.len(), shape.numel());
    }

    #[test]
    fn trailing_semicolon_is_ignored() {
        let shape = Fetcher::new("SELECT 42 AS n;  ").fetch(&db()).unwrap();
        assert_eq!(shape.height(), 1);
        assert_eq!(shape.width(), 1);
    }

    #[test]
    fn invalid_sql_fails() {
        assert!(Fetcher::new("SELEC 1").fetch(&db()).is_err());
    }

    #[test]
    fn run_returns_the_shape_through_the_worker() {
        let shape = wait(&Fetcher::new("SELECT * FROM range(5)").run()).unwrap();
        assert_eq!(shape.height(), 5);
        assert_eq!(shape.width(), 1);
    }
}
