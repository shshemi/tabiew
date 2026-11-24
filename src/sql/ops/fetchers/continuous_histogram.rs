use anyhow::anyhow;

use crate::{
    AppResult,
    sql::{
        db::{Database, ValueRef},
        ops::{
            reply::{Receiver, channel},
            util::quote_ident,
        },
        worker::submit,
    },
};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct ContinuousHistogram {
    buckets: Vec<Bucket>,
}

impl ContinuousHistogram {
    pub fn new(buckets: Vec<Bucket>) -> Self {
        Self { buckets }
    }

    pub fn buckets(&self) -> &[Bucket] {
        &self.buckets
    }

    pub fn len(&self) -> usize {
        self.buckets.len()
    }

    pub fn is_empty(&self) -> bool {
        self.buckets.is_empty()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Bucket {
    start: f64,
    end: f64,
    count: u64,
}

impl Bucket {
    pub fn start(&self) -> f64 {
        self.start
    }

    pub fn end(&self) -> f64 {
        self.end
    }

    pub fn count(&self) -> u64 {
        self.count
    }
}

#[derive(Debug, Clone)]
pub struct Fetcher {
    sql: String,
    column: String,
    buckets: usize,
}

impl Fetcher {
    pub fn new(sql: impl Into<String>, column: impl Into<String>, buckets: usize) -> Self {
        Self {
            sql: sql.into(),
            column: column.into(),
            buckets,
        }
    }

    pub fn run(&self) -> Receiver<ContinuousHistogram> {
        let this = self.clone();
        let (send, recv) = channel();
        submit(move |db| send.send(this.fetch(db)));
        recv
    }

    fn fetch(&self, db: &Database) -> AppResult<ContinuousHistogram> {
        if self.buckets == 0 {
            return Err(anyhow!("bucket count must be greater than zero"));
        }
        let sql = self.sql.trim().trim_end_matches(';');
        let column = quote_ident(&self.column);
        let mut stmt = db.prepare(&format!(
            "WITH \
             v AS (SELECT {column}::DOUBLE AS v FROM ({sql}) WHERE isfinite({column}::DOUBLE)), \
             r AS (SELECT MIN(v) AS lo, MAX(v) AS hi FROM v) \
             SELECT lo, hi, \
             LEAST(FLOOR((v - lo) / CASE WHEN hi > lo THEN (hi - lo) / ? ELSE 1 END)::BIGINT, ?) AS idx, \
             COUNT(*) \
             FROM v, r GROUP BY lo, hi, idx"
        ))?;
        let buckets = self.buckets as i64;
        let mut rows = stmt.query(duckdb::params![buckets, buckets - 1])?;
        let mut range = None;
        let mut counts = vec![0; self.buckets];
        while let Some(row) = rows.next()? {
            match (
                row.get_ref(0)?,
                row.get_ref(1)?,
                row.get_ref(2)?,
                row.get_ref(3)?,
            ) {
                (
                    ValueRef::Double(min),
                    ValueRef::Double(max),
                    ValueRef::BigInt(idx),
                    ValueRef::BigInt(count),
                ) => {
                    range = Some((min, max));
                    counts[usize::try_from(idx)?] = u64::try_from(count)?;
                }
                other => return Err(anyhow!("unexpected histogram bucket: {other:?}")),
            }
        }
        let Some((min, max)) = range else {
            return Ok(ContinuousHistogram::new(Vec::new()));
        };

        let width = (max - min) / self.buckets as f64;
        Ok(ContinuousHistogram::new(
            counts
                .into_iter()
                .enumerate()
                .map(|(idx, count)| Bucket {
                    start: min + idx as f64 * width,
                    end: min + (idx + 1) as f64 * width,
                    count,
                })
                .collect(),
        ))
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

    fn buckets(histogram: ContinuousHistogram) -> Vec<(f64, f64, u64)> {
        histogram
            .buckets()
            .iter()
            .map(|bucket| (bucket.start(), bucket.end(), bucket.count()))
            .collect()
    }

    #[test]
    fn numbers_are_split_into_equal_buckets() {
        let histogram = Fetcher::new("SELECT range AS n FROM range(10)", "n", 3)
            .fetch(&db())
            .unwrap();
        assert_eq!(
            buckets(histogram),
            [(0.0, 3.0, 3), (3.0, 6.0, 3), (6.0, 9.0, 4)]
        );
    }

    #[test]
    fn empty_buckets_are_kept() {
        let histogram = Fetcher::new("SELECT * FROM (VALUES (0.0), (10.0)) v(n)", "n", 4)
            .fetch(&db())
            .unwrap();
        assert_eq!(
            buckets(histogram),
            [(0.0, 2.5, 1), (2.5, 5.0, 0), (5.0, 7.5, 0), (7.5, 10.0, 1)]
        );
    }

    #[test]
    fn nulls_and_non_finite_values_are_skipped() {
        let histogram = Fetcher::new(
            "SELECT * FROM (VALUES (1.0), (NULL), ('nan'::DOUBLE), ('inf'::DOUBLE), (3.0)) v(n)",
            "n",
            2,
        )
        .fetch(&db())
        .unwrap();
        assert_eq!(buckets(histogram), [(1.0, 2.0, 1), (2.0, 3.0, 1)]);
    }

    #[test]
    fn single_value_lands_in_the_first_bucket() {
        let histogram = Fetcher::new("SELECT 5 AS n FROM range(3)", "n", 2)
            .fetch(&db())
            .unwrap();
        assert_eq!(buckets(histogram), [(5.0, 5.0, 3), (5.0, 5.0, 0)]);
    }

    #[test]
    fn decimals_are_continuous() {
        let histogram = Fetcher::new("SELECT 1.5::DECIMAL(4, 1) AS n", "n", 1)
            .fetch(&db())
            .unwrap();
        assert_eq!(buckets(histogram), [(1.5, 1.5, 1)]);
    }

    #[test]
    fn all_null_numbers_have_no_buckets() {
        let histogram = Fetcher::new("SELECT NULL::INTEGER AS n", "n", 3)
            .fetch(&db())
            .unwrap();
        assert!(histogram.is_empty());
    }

    #[test]
    fn zero_buckets_fail() {
        assert!(Fetcher::new("SELECT 1 AS n", "n", 0).fetch(&db()).is_err());
    }

    #[test]
    fn run_returns_the_histogram_through_the_worker() {
        let histogram =
            wait(&Fetcher::new("SELECT range AS n FROM range(4)", "n", 2).run()).unwrap();
        assert_eq!(buckets(histogram), [(0.0, 1.5, 2), (1.5, 3.0, 2)]);
    }
}
