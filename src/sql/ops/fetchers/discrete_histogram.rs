use anyhow::anyhow;

use crate::{
    AppResult,
    sql::{
        db::{Database, Query, Value, ValueRef},
        ops::{
            reply::{Receiver, channel},
            util::quote_ident,
        },
        worker::submit,
    },
};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct DiscreteHistogram {
    categories: Vec<Category>,
}

impl DiscreteHistogram {
    pub fn new(categories: Vec<Category>) -> Self {
        Self { categories }
    }

    pub fn categories(&self) -> &[Category] {
        &self.categories
    }

    pub fn len(&self) -> usize {
        self.categories.len()
    }

    pub fn is_empty(&self) -> bool {
        self.categories.is_empty()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Category {
    value: Value,
    count: u64,
}

impl Category {
    pub fn value(&self) -> &Value {
        &self.value
    }

    pub fn count(&self) -> u64 {
        self.count
    }
}

#[derive(Debug, Clone)]
pub struct Fetcher {
    query: Query,
    column: String,
}

impl Fetcher {
    pub fn new(query: Query, column: impl Into<String>) -> Self {
        Self {
            query,
            column: column.into(),
        }
    }

    pub fn run(&self) -> Receiver<DiscreteHistogram> {
        let this = self.clone();
        let (send, recv) = channel();
        submit(move |db| send.send(this.fetch(db)));
        recv
    }

    fn fetch(&self, db: &Database) -> AppResult<DiscreteHistogram> {
        let sql = self.query.as_str();
        let column = quote_ident(&self.column);
        let mut stmt = db.prepare(&format!(
            "SELECT {column}, COUNT(*) AS count FROM ({sql}) \
             GROUP BY {column} ORDER BY count DESC, {column} NULLS LAST"
        ))?;
        let mut rows = stmt.query([])?;
        let mut categories = Vec::new();
        while let Some(row) = rows.next()? {
            let count = match row.get_ref(1)? {
                ValueRef::BigInt(count) => u64::try_from(count)?,
                other => return Err(anyhow!("unexpected histogram count: {other:?}")),
            };
            categories.push(Category {
                value: row.get_ref(0)?.into(),
                count,
            });
        }
        Ok(DiscreteHistogram::new(categories))
    }
}

#[cfg(test)]
mod tests {
    use std::time::{Duration, Instant};

    use super::*;
    use crate::sql::{db::DatabaseConfig, ops::Next};

    fn query(sql: &str) -> Query {
        Query::new(sql).unwrap()
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

    fn categories(histogram: DiscreteHistogram) -> Vec<(Value, u64)> {
        histogram
            .categories()
            .iter()
            .map(|category| (category.value().clone(), category.count()))
            .collect()
    }

    #[test]
    fn text_is_counted_by_value() {
        let histogram = Fetcher::new(
            query("SELECT * FROM (VALUES ('b'), ('a'), ('b'), (NULL), ('c'), ('a'), ('b')) v(s)"),
            "s",
        )
        .fetch(&db())
        .unwrap();
        assert_eq!(
            categories(histogram),
            [
                (Value::Text("b".into()), 3),
                (Value::Text("a".into()), 2),
                (Value::Text("c".into()), 1),
                (Value::Null, 1),
            ]
        );
    }

    #[test]
    fn booleans_are_counted_by_value() {
        let histogram = Fetcher::new(
            query("SELECT * FROM (VALUES (true), (false), (true)) v(b)"),
            "b",
        )
        .fetch(&db())
        .unwrap();
        assert_eq!(
            categories(histogram),
            [(Value::Boolean(true), 2), (Value::Boolean(false), 1)]
        );
    }

    #[test]
    fn quoted_column_names_work() {
        let histogram = Fetcher::new(query("SELECT 'x' AS \"a b\""), "a b")
            .fetch(&db())
            .unwrap();
        assert_eq!(categories(histogram), [(Value::Text("x".into()), 1)]);
    }

    #[test]
    fn missing_column_fails() {
        assert!(
            Fetcher::new(query("SELECT 1 AS n"), "missing")
                .fetch(&db())
                .is_err()
        );
    }

    #[test]
    fn numbers_are_counted_by_value() {
        let histogram = Fetcher::new(query("SELECT * FROM (VALUES (1), (2), (1)) v(n)"), "n")
            .fetch(&db())
            .unwrap();
        assert_eq!(
            categories(histogram),
            [(Value::Int(1), 2), (Value::Int(2), 1)]
        );
    }

    #[test]
    fn run_returns_the_histogram_through_the_worker() {
        let histogram = wait(&Fetcher::new(query("SELECT 'x' AS s"), "s").run()).unwrap();
        assert_eq!(categories(histogram), [(Value::Text("x".into()), 1)]);
    }
}
