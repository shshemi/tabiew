use crate::{
    AppResult,
    duckdb::{
        DuckDB,
        cache::file_db::ColumnStats,
        cancel::Cancel,
        reader::{Count, ViewSource},
        results::RESULT_PREFIX,
        sql::quote,
    },
    misc::workers,
    reader::{Reader, RowSource},
};
use anyhow::anyhow;
use duckdb::Connection;
use std::path::PathBuf;
use tokio::sync::oneshot::{self, Receiver, error::TryRecvError};

pub struct TableEntry {
    pub name: String,
    pub source: Option<PathBuf>,
    pub rows: Option<usize>,
    pub disk: Option<u64>,
    pub disk_pending: bool,
    pub columns: Vec<(String, String)>,
    pub stats: Option<Vec<ColumnStats>>,
}

pub(super) struct CountJob {
    view: String,
    counts: Vec<Count>,
    result: Receiver<AppResult<usize>>,
    cancel: Cancel,
}

pub(super) struct StatsJob {
    view: String,
    result: Receiver<AppResult<Vec<ColumnStats>>>,
    cancel: Cancel,
}

fn width(data_type: &str) -> Option<u64> {
    match data_type {
        "BOOLEAN" | "TINYINT" | "UTINYINT" => Some(1),
        "SMALLINT" | "USMALLINT" => Some(2),
        "INTEGER" | "UINTEGER" | "FLOAT" | "DATE" => Some(4),
        "BIGINT" | "UBIGINT" | "DOUBLE" | "TIME" | "TIMESTAMP" | "TIMESTAMP WITH TIME ZONE" => {
            Some(8)
        }
        "HUGEINT" | "UHUGEINT" | "UUID" | "INTERVAL" => Some(16),
        _ if data_type.starts_with("DECIMAL") => Some(16),
        _ => None,
    }
}

fn stats(
    conn: &Connection,
    view: &str,
    columns: &[(String, String)],
) -> AppResult<Vec<ColumnStats>> {
    let query = |extremes: bool| {
        let fields = columns
            .iter()
            .map(|(column, data_type)| {
                let column = quote(column);
                let bytes = match (width(data_type), data_type.as_str()) {
                    (Some(_), _) => "NULL".to_string(),
                    (None, "BLOB") => format!("sum(octet_length({column}))"),
                    (None, "VARCHAR") => format!("sum(strlen({column}))"),
                    (None, _) => format!("sum(strlen(CAST({column} AS VARCHAR)))"),
                };
                match extremes {
                    true => format!(
                        "count({column}), CAST(min({column}) AS VARCHAR), CAST(max({column}) AS VARCHAR), {bytes}"
                    ),
                    false => format!("count({column}), NULL, NULL, {bytes}"),
                }
            })
            .collect::<Vec<_>>()
            .join(", ");
        conn.query_row(
            &format!("SELECT count(*), {fields} FROM {}", quote(view)),
            [],
            |r| {
                let rows = r.get::<_, u64>(0)?;
                columns
                    .iter()
                    .enumerate()
                    .map(|(idx, (_, data_type))| {
                        Ok(ColumnStats {
                            nulls: rows - r.get::<_, u64>(idx * 4 + 1)?,
                            min: r.get(idx * 4 + 2)?,
                            max: r.get(idx * 4 + 3)?,
                            bytes: Some(match width(data_type) {
                                Some(width) => width * rows,
                                None => r.get::<_, Option<u64>>(idx * 4 + 4)?.unwrap_or_default(),
                            }),
                        })
                    })
                    .collect()
            },
        )
    };
    Ok(query(true).or_else(|_| query(false))?)
}

impl DuckDB {
    pub fn tables(&mut self) -> Vec<TableEntry> {
        let names = self.session.names().map(str::to_string).collect::<Vec<_>>();
        names.iter().filter_map(|name| self.entry(name)).collect()
    }
    pub fn entry(&mut self, name: &str) -> Option<TableEntry> {
        let view = ViewSource::growing(self.session.session().ok()?, name).ok()?;
        let rows = self.counts.get(name).copied();
        if rows.is_none() {
            self.request_count(name, None).ok()?;
        }
        if !self.stats.contains_key(name)
            && let Some(stats) = self.session.stored_stats(name)
            && stats.iter().all(|stats| stats.bytes.is_some())
        {
            self.stats.insert(name.to_string(), stats);
        }
        let disk = match self.session.in_memory(name) {
            true => Some(0),
            false => self
                .session
                .stored_size(name)
                .or_else(|| self.scratch_size(name)),
        };
        Some(TableEntry {
            source: self.session.source_path(name),
            rows,
            disk,
            disk_pending: disk.is_none() && self.checkpoint_due && name.starts_with(RESULT_PREFIX),
            columns: view.columns().to_vec(),
            stats: self.stats.get(name).cloned(),
            name: name.to_string(),
        })
    }
    pub fn request_stats(&mut self, view: &str) -> AppResult<()> {
        if self.stats.contains_key(view) || self.stats_jobs.iter().any(|job| job.view == view) {
            return Ok(());
        }
        let conn = self.session.session()?;
        let columns = ViewSource::growing(self.session.session()?, view)?
            .columns()
            .to_vec();
        let cancel = Cancel::default();
        cancel.watch(&conn);
        let (sender, result) = oneshot::channel();
        let name = view.to_string();
        workers::spawn_short(move || {
            let _ = sender.send(stats(&conn, &name, &columns));
        });
        self.stats_jobs.push(StatsJob {
            view: view.to_string(),
            result,
            cancel,
        });
        Ok(())
    }
    pub fn forget_stats(&mut self, view: &str) {
        self.stats.remove(view);
        self.counts.remove(view);
    }
    pub fn counted_reader(&mut self, view: &str) -> AppResult<Reader<ViewSource>> {
        let (reader, count) = self.growing_reader(view)?;
        match self.counts.get(view) {
            Some(rows) => {
                let _ = count.set(*rows);
            }
            None => self.request_count(view, Some(count))?,
        }
        Ok(reader)
    }
    fn request_count(&mut self, view: &str, count: Option<Count>) -> AppResult<()> {
        if let Some(job) = self.count_jobs.iter_mut().find(|job| job.view == view) {
            job.counts.extend(count);
            return Ok(());
        }
        let conn = self.session.session()?;
        let cancel = Cancel::default();
        cancel.watch(&conn);
        let (sender, result) = oneshot::channel();
        let sql = format!("SELECT count(*) FROM {}", quote(view));
        workers::spawn_short(move || {
            let _ = sender.send(
                conn.query_row(&sql, [], |r| r.get::<_, i64>(0))
                    .map(|rows| rows as usize)
                    .map_err(Into::into),
            );
        });
        self.count_jobs.push(CountJob {
            view: view.to_string(),
            counts: count.into_iter().collect(),
            result,
            cancel,
        });
        Ok(())
    }
    pub fn cancel_jobs(&mut self, view: &str, dropped: bool) {
        self.cancel_plots(view);
        if !dropped && !view.starts_with(RESULT_PREFIX) {
            return;
        }
        for job in self.count_jobs.iter().filter(|job| job.view == view) {
            job.cancel.cancel();
        }
        for job in self.stats_jobs.iter().filter(|job| job.view == view) {
            job.cancel.cancel();
        }
    }
    pub(super) fn poll_counts(&mut self) -> bool {
        let mut changed = false;
        let mut i = 0;
        while i < self.count_jobs.len() {
            let result = match self.count_jobs[i].result.try_recv() {
                Err(TryRecvError::Empty) => {
                    i += 1;
                    continue;
                }
                Ok(result) => result,
                Err(TryRecvError::Closed) => Err(anyhow!("Count stopped without a result")),
            };
            let job = self.count_jobs.swap_remove(i);
            if let Ok(rows) = result {
                for count in &job.counts {
                    let _ = count.set(rows);
                }
                self.counts.insert(job.view, rows);
                changed = true;
            }
        }
        changed
    }
    pub(super) fn poll_stats(&mut self) -> bool {
        let mut changed = false;
        let mut i = 0;
        while i < self.stats_jobs.len() {
            let result = match self.stats_jobs[i].result.try_recv() {
                Err(TryRecvError::Empty) => {
                    i += 1;
                    continue;
                }
                Ok(result) => result,
                Err(TryRecvError::Closed) => Err(anyhow!("Stats stopped without a result")),
            };
            let job = self.stats_jobs.swap_remove(i);
            if let Ok(stats) = result {
                self.session.store_stats(&job.view, &stats);
                self.stats.insert(job.view, stats);
                changed = true;
            }
        }
        changed
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::duckdb::{DuckDBConfig, IngestOptions, Load};
    use std::time::{Duration, Instant};
    use tempfile::TempDir;

    fn wait(db: &mut DuckDB, done: impl Fn(&crate::duckdb::Finished) -> bool) {
        let start = Instant::now();
        loop {
            assert!(start.elapsed() < Duration::from_secs(10));
            if done(&db.poll().unwrap()) {
                return;
            }
            std::thread::sleep(Duration::from_millis(10));
        }
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn column_stats_are_computed_once_and_kept_in_the_cache_metadata() {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join("sales.csv");
        std::fs::write(&path, "id,name\n1,b\n2,\n3,a\n").unwrap();
        let config = || DuckDBConfig {
            cache_dir: dir.path().join("cache"),
            ..DuckDBConfig::default()
        };
        let mut db = DuckDB::new(config()).unwrap();
        assert!(matches!(
            db.load(&path, None, &IngestOptions::default(), None)
                .unwrap(),
            Load::Ingesting(_)
        ));
        wait(&mut db, |finished| !finished.tables.is_empty());
        let tables = db.tables();
        assert_eq!(tables.len(), 1);
        assert_eq!((tables[0].name.as_str(), tables[0].rows), ("sales", None));
        assert!(tables[0].disk.is_some_and(|disk| disk > 0));
        wait(&mut db, |finished| finished.stats);
        assert_eq!(db.tables()[0].rows, Some(3));
        assert_eq!(db.counted_reader("sales").unwrap().row_count(), 3);
        assert_eq!(
            tables[0].source.as_deref(),
            Some(path.canonicalize().unwrap().as_path())
        );
        assert_eq!(tables[0].columns[1].0, "name");
        assert!(tables[0].stats.is_none());
        db.request_stats("sales").unwrap();
        db.request_stats("sales").unwrap();
        assert_eq!(db.stats_jobs.len(), 1);
        assert!(db.request_stats("nope").is_err());
        wait(&mut db, |finished| finished.stats);
        let expected = vec![
            ColumnStats {
                nulls: 0,
                min: Some("1".to_string()),
                max: Some("3".to_string()),
                bytes: Some(24),
            },
            ColumnStats {
                nulls: 1,
                min: Some("a".to_string()),
                max: Some("b".to_string()),
                bytes: Some(2),
            },
        ];
        assert_eq!(db.tables()[0].stats.as_ref(), Some(&expected));
        drop(db);
        let mut db = DuckDB::new(config()).unwrap();
        assert!(matches!(
            db.load(&path, None, &IngestOptions::default(), None)
                .unwrap(),
            Load::Ready(_)
        ));
        assert_eq!(db.tables()[0].stats.as_ref(), Some(&expected));
        assert!(db.stats_jobs.is_empty());
        let other = dir.path().join("other.csv");
        std::fs::write(&other, "x\n1\n").unwrap();
        db.load(&other, None, &IngestOptions::default(), None)
            .unwrap();
        wait(&mut db, |finished| !finished.tables.is_empty());
        let reader = db.reader("sales").unwrap();
        db.detach("other").unwrap();
        db.forget_stats("other");
        let names = |db: &mut DuckDB| {
            db.tables()
                .into_iter()
                .map(|entry| entry.name)
                .collect::<Vec<_>>()
        };
        assert_eq!(names(&mut db), ["sales"]);
        db.poll().unwrap();
        assert_eq!(names(&mut db), ["sales"]);
        drop(reader);
        db.poll().unwrap();
        assert_eq!(names(&mut db), ["sales"]);
    }
}
