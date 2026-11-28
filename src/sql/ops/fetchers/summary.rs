use anyhow::anyhow;

use crate::{
    AppResult,
    sql::{
        db::{Database, Query, Row, Value, ValueRef},
        ops::reply::{Receiver, channel},
        worker::submit,
    },
};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Summary {
    columns: Vec<ColumnSummary>,
}

impl Summary {
    pub fn new(columns: Vec<ColumnSummary>) -> Self {
        Self { columns }
    }

    pub fn get(&self, idx: usize) -> Option<&ColumnSummary> {
        self.columns.get(idx)
    }

    pub fn find(&self, name: &str) -> Option<&ColumnSummary> {
        self.columns.iter().find(|column| column.name() == name)
    }

    pub fn iter(&self) -> impl Iterator<Item = &ColumnSummary> {
        self.columns.iter()
    }

    pub fn len(&self) -> usize {
        self.columns.len()
    }

    pub fn is_empty(&self) -> bool {
        self.columns.is_empty()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ColumnSummary {
    name: String,
    dtype: String,
    min: Value,
    max: Value,
    approx_unique: u64,
    avg: Value,
    std: Value,
    q25: Value,
    q50: Value,
    q75: Value,
    count: u64,
    null_percentage: Value,
}

impl ColumnSummary {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn dtype(&self) -> &str {
        &self.dtype
    }

    pub fn min(&self) -> &Value {
        &self.min
    }

    pub fn max(&self) -> &Value {
        &self.max
    }

    pub fn approx_unique(&self) -> u64 {
        self.approx_unique
    }

    pub fn avg(&self) -> &Value {
        &self.avg
    }

    pub fn std(&self) -> &Value {
        &self.std
    }

    pub fn q25(&self) -> &Value {
        &self.q25
    }

    pub fn q50(&self) -> &Value {
        &self.q50
    }

    pub fn q75(&self) -> &Value {
        &self.q75
    }

    pub fn count(&self) -> u64 {
        self.count
    }

    pub fn null_percentage(&self) -> &Value {
        &self.null_percentage
    }

    fn from_row(row: &Row) -> AppResult<Self> {
        Ok(Self {
            name: text(row, 0)?.ok_or_else(|| anyhow!("summary has no column name"))?,
            dtype: text(row, 1)?.ok_or_else(|| anyhow!("summary has no column type"))?,
            min: row.get_ref(2)?.into(),
            max: row.get_ref(3)?.into(),
            approx_unique: count(row, 4)?,
            avg: row.get_ref(5)?.into(),
            std: row.get_ref(6)?.into(),
            q25: row.get_ref(7)?.into(),
            q50: row.get_ref(8)?.into(),
            q75: row.get_ref(9)?.into(),
            count: count(row, 10)?,
            null_percentage: row.get_ref(11)?.into(),
        })
    }
}

#[derive(Debug, Clone)]
pub struct Fetcher {
    query: Query,
}

impl Fetcher {
    pub fn new(query: Query) -> Self {
        Self { query }
    }

    pub fn run(&self) -> Receiver<Summary> {
        let this = self.clone();
        let (send, recv) = channel();
        submit(move |db| send.send(this.fetch(db)));
        recv
    }

    fn fetch(&self, db: &Database) -> AppResult<Summary> {
        let mut stmt = db.prepare(&format!(
            "SELECT column_name, column_type, min, max, approx_unique, avg, std, q25, q50, q75, \
             count, null_percentage FROM (SUMMARIZE ({}))",
            self.query.as_str()
        ))?;
        let mut rows = stmt.query([])?;
        let mut summaries = Vec::new();
        while let Some(row) = rows.next()? {
            summaries.push(ColumnSummary::from_row(&row)?);
        }
        Ok(Summary::new(summaries))
    }
}

fn text(row: &Row, idx: usize) -> AppResult<Option<String>> {
    match row.get_ref(idx)? {
        ValueRef::Text(bytes) => Ok(Some(String::from_utf8_lossy(bytes).into_owned())),
        ValueRef::Null => Ok(None),
        other => Err(anyhow!(
            "expected text in summary column {idx}, got {other:?}"
        )),
    }
}

fn count(row: &Row, idx: usize) -> AppResult<u64> {
    match row.get_ref(idx)? {
        ValueRef::BigInt(count) => Ok(u64::try_from(count)?),
        other => Err(anyhow!(
            "expected a count in summary column {idx}, got {other:?}"
        )),
    }
}

#[cfg(test)]
mod tests {
    use std::time::{Duration, Instant};

    use duckdb::types::Decimal;

    use super::*;
    use crate::sql::{db::DatabaseConfig, ops::Next};

    fn query(sql: &str) -> Query {
        Query::new(sql).unwrap()
    }

    fn text(value: &str) -> Value {
        Value::Text(value.into())
    }

    fn percent(value: i128) -> Value {
        Value::Decimal(Decimal::new(9, 2, value * 100).unwrap())
    }

    fn db() -> Database {
        let db = DatabaseConfig::default().open_in_memory().unwrap();
        db.execute(
            "CREATE TABLE t AS SELECT * FROM (VALUES \
             (1, 'b', 1.5), (2, 'a', NULL), (3, 'c', NULL), (4, NULL, NULL)) \
             v(n, s, \"x y\")",
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
    fn all_columns_are_summarized_in_order() {
        let summary = Fetcher::new(query("SELECT * FROM t")).fetch(&db()).unwrap();
        assert_eq!(summary.len(), 3);
        assert_eq!(
            summary.iter().map(ColumnSummary::name).collect::<Vec<_>>(),
            ["n", "s", "x y"]
        );
    }

    #[test]
    fn numeric_column_has_full_stats() {
        let summary = Fetcher::new(query("SELECT n FROM t")).fetch(&db()).unwrap();
        assert_eq!(summary.len(), 1);
        let n = summary.get(0).unwrap();
        assert_eq!(n.name(), "n");
        assert_eq!(n.dtype(), "INTEGER");
        assert_eq!(n.min(), &text("1"));
        assert_eq!(n.max(), &text("4"));
        assert_eq!(n.avg(), &text("2.5"));
        assert!(matches!(n.std(), Value::Text(_)));
        assert!(matches!(n.q25(), Value::Text(_)));
        assert!(matches!(n.q50(), Value::Text(_)));
        assert!(matches!(n.q75(), Value::Text(_)));
        assert_eq!(n.approx_unique(), 4);
        assert_eq!(n.count(), 4);
        assert_eq!(n.null_percentage(), &percent(0));
    }

    #[test]
    #[ignore = "duckdb-rs panics reading the untyped NULL stat columns"]
    fn text_column_has_no_numeric_stats() {
        let summary = Fetcher::new(query("SELECT s FROM t")).fetch(&db()).unwrap();
        let s = summary.find("s").unwrap();
        assert_eq!(s.dtype(), "VARCHAR");
        assert_eq!(s.min(), &text("a"));
        assert_eq!(s.max(), &text("c"));
        assert_eq!(s.avg(), &Value::Null);
        assert_eq!(s.std(), &Value::Null);
        assert_eq!(s.q50(), &Value::Null);
        assert_eq!(s.count(), 4);
        assert_eq!(s.null_percentage(), &percent(25));
    }

    #[test]
    fn selected_columns_keep_their_order() {
        let summary = Fetcher::new(query("SELECT \"x y\", n FROM t"))
            .fetch(&db())
            .unwrap();
        assert_eq!(
            summary.iter().map(ColumnSummary::name).collect::<Vec<_>>(),
            ["x y", "n"]
        );
        assert_eq!(summary.find("x y").unwrap().null_percentage(), &percent(75));
    }

    #[test]
    fn empty_table_has_zero_counts() {
        let db = db();
        db.execute("CREATE TABLE empty (n INTEGER)", []).unwrap();
        let summary = Fetcher::new(query("SELECT * FROM empty"))
            .fetch(&db)
            .unwrap();
        let n = summary.find("n").unwrap();
        assert_eq!(n.count(), 0);
        assert_eq!(n.min(), &Value::Null);
        assert_eq!(n.max(), &Value::Null);
    }

    #[test]
    fn missing_column_fails() {
        assert!(
            Fetcher::new(query("SELECT missing FROM t"))
                .fetch(&db())
                .is_err()
        );
    }

    #[test]
    fn expressions_and_table_functions_are_summarized() {
        let summary = Fetcher::new(query("SELECT range * 2 AS doubled FROM range(5)"))
            .fetch(&db())
            .unwrap();
        let doubled = summary.find("doubled").unwrap();
        assert_eq!(doubled.min(), &text("0"));
        assert_eq!(doubled.max(), &text("8"));
    }

    #[test]
    fn run_returns_the_summary_through_the_worker() {
        let (send, recv) = channel();
        submit(move |db| {
            send.send(db.execute(
                "CREATE OR REPLACE TABLE summary_run AS SELECT range AS n FROM range(4)",
                [],
            ))
        });
        wait(&recv).unwrap();
        let summary = wait(&Fetcher::new(query("SELECT * FROM summary_run")).run()).unwrap();
        assert_eq!(summary.find("n").unwrap().count(), 4);
    }
}
