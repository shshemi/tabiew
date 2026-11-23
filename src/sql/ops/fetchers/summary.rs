use anyhow::anyhow;

use crate::{
    AppResult,
    sql::{
        db::{Database, Row, ValueRef},
        ops::{
            reply::{Receiver, channel},
            util::quote_ident,
        },
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
    min: Option<String>,
    max: Option<String>,
    approx_unique: u64,
    avg: Option<String>,
    std: Option<String>,
    q25: Option<String>,
    q50: Option<String>,
    q75: Option<String>,
    count: u64,
    null_percentage: f64,
}

impl ColumnSummary {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn dtype(&self) -> &str {
        &self.dtype
    }

    pub fn min(&self) -> Option<&str> {
        self.min.as_deref()
    }

    pub fn max(&self) -> Option<&str> {
        self.max.as_deref()
    }

    pub fn approx_unique(&self) -> u64 {
        self.approx_unique
    }

    pub fn avg(&self) -> Option<&str> {
        self.avg.as_deref()
    }

    pub fn std(&self) -> Option<&str> {
        self.std.as_deref()
    }

    pub fn q25(&self) -> Option<&str> {
        self.q25.as_deref()
    }

    pub fn q50(&self) -> Option<&str> {
        self.q50.as_deref()
    }

    pub fn q75(&self) -> Option<&str> {
        self.q75.as_deref()
    }

    pub fn count(&self) -> u64 {
        self.count
    }

    pub fn null_percentage(&self) -> f64 {
        self.null_percentage
    }

    fn from_row(row: &Row) -> AppResult<Self> {
        Ok(Self {
            name: text(row, 0)?.ok_or_else(|| anyhow!("summary has no column name"))?,
            dtype: text(row, 1)?.ok_or_else(|| anyhow!("summary has no column type"))?,
            min: text(row, 2)?,
            max: text(row, 3)?,
            approx_unique: count(row, 4)?,
            avg: text(row, 5)?,
            std: text(row, 6)?,
            q25: text(row, 7)?,
            q50: text(row, 8)?,
            q75: text(row, 9)?,
            count: count(row, 10)?,
            null_percentage: percentage(row, 11)?,
        })
    }
}

#[derive(Debug, Clone)]
pub struct Fetcher {
    table: String,
    columns: Option<Vec<String>>,
}

impl Fetcher {
    pub fn new(table: impl Into<String>) -> Self {
        Self {
            table: table.into(),
            columns: None,
        }
    }

    pub fn with_columns(mut self, columns: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.columns = Some(columns.into_iter().map(Into::into).collect());
        self
    }

    pub fn run(&self) -> Receiver<Summary> {
        let this = self.clone();
        let (send, recv) = channel();
        submit(move |db| send.send(this.fetch(db)));
        recv
    }

    fn fetch(&self, db: &Database) -> AppResult<Summary> {
        let columns = match &self.columns {
            Some(columns) => columns
                .iter()
                .map(|column| quote_ident(column))
                .collect::<Vec<_>>()
                .join(", "),
            None => "*".to_owned(),
        };
        let mut stmt = db.prepare(&format!(
            "SELECT column_name, column_type, min::VARCHAR, max::VARCHAR, approx_unique, \
             avg::VARCHAR, std::VARCHAR, q25::VARCHAR, q50::VARCHAR, q75::VARCHAR, \
             count, null_percentage::DOUBLE \
             FROM (SUMMARIZE SELECT {columns} FROM {})",
            quote_ident(&self.table)
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

fn percentage(row: &Row, idx: usize) -> AppResult<f64> {
    match row.get_ref(idx)? {
        ValueRef::Double(percentage) => Ok(percentage),
        ValueRef::Null => Ok(0.0),
        other => Err(anyhow!(
            "expected a percentage in summary column {idx}, got {other:?}"
        )),
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
        let summary = Fetcher::new("t").fetch(&db()).unwrap();
        assert_eq!(summary.len(), 3);
        assert_eq!(
            summary.iter().map(ColumnSummary::name).collect::<Vec<_>>(),
            ["n", "s", "x y"]
        );
    }

    #[test]
    fn numeric_column_has_full_stats() {
        let summary = Fetcher::new("t").with_columns(["n"]).fetch(&db()).unwrap();
        assert_eq!(summary.len(), 1);
        let n = summary.get(0).unwrap();
        assert_eq!(n.name(), "n");
        assert_eq!(n.dtype(), "INTEGER");
        assert_eq!(n.min(), Some("1"));
        assert_eq!(n.max(), Some("4"));
        assert_eq!(n.avg(), Some("2.5"));
        assert!(n.std().is_some());
        assert!(n.q25().is_some());
        assert!(n.q50().is_some());
        assert!(n.q75().is_some());
        assert_eq!(n.approx_unique(), 4);
        assert_eq!(n.count(), 4);
        assert_eq!(n.null_percentage(), 0.0);
    }

    #[test]
    fn text_column_has_no_numeric_stats() {
        let summary = Fetcher::new("t").with_columns(["s"]).fetch(&db()).unwrap();
        let s = summary.find("s").unwrap();
        assert_eq!(s.dtype(), "VARCHAR");
        assert_eq!(s.min(), Some("a"));
        assert_eq!(s.max(), Some("c"));
        assert_eq!(s.avg(), None);
        assert_eq!(s.std(), None);
        assert_eq!(s.q50(), None);
        assert_eq!(s.count(), 4);
        assert_eq!(s.null_percentage(), 25.0);
    }

    #[test]
    fn selected_columns_keep_their_order() {
        let summary = Fetcher::new("t")
            .with_columns(["x y", "n"])
            .fetch(&db())
            .unwrap();
        assert_eq!(
            summary.iter().map(ColumnSummary::name).collect::<Vec<_>>(),
            ["x y", "n"]
        );
        assert_eq!(summary.find("x y").unwrap().null_percentage(), 75.0);
    }

    #[test]
    fn empty_table_has_zero_counts() {
        let db = db();
        db.execute("CREATE TABLE empty (n INTEGER)", []).unwrap();
        let summary = Fetcher::new("empty").fetch(&db).unwrap();
        let n = summary.find("n").unwrap();
        assert_eq!(n.count(), 0);
        assert_eq!(n.min(), None);
        assert_eq!(n.max(), None);
    }

    #[test]
    fn missing_column_fails() {
        assert!(
            Fetcher::new("t")
                .with_columns(["missing"])
                .fetch(&db())
                .is_err()
        );
    }

    #[test]
    fn missing_table_fails() {
        assert!(Fetcher::new("missing").fetch(&db()).is_err());
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
        let summary = wait(&Fetcher::new("summary_run").run()).unwrap();
        assert_eq!(summary.find("n").unwrap().count(), 4);
    }
}
