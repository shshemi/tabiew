use crate::{
    AppResult,
    duckdb::{DuckDB, cache::file_db::ColumnStats, reader::ViewSource, sql::quote},
    reader::RowSource,
};
use anyhow::anyhow;
use duckdb::Connection;
use std::path::PathBuf;
use tokio::sync::oneshot::{self, Receiver, error::TryRecvError};

pub struct TableEntry {
    pub name: String,
    pub source: Option<PathBuf>,
    pub rows: usize,
    pub columns: Vec<(String, String)>,
    pub stats: Option<Vec<ColumnStats>>,
}

pub(super) struct StatsJob {
    view: String,
    result: Receiver<AppResult<Vec<ColumnStats>>>,
}

fn stats(conn: &Connection, view: &str, columns: &[String]) -> AppResult<Vec<ColumnStats>> {
    let query = |extremes: bool| {
        let fields = columns
            .iter()
            .map(|column| {
                let column = quote(column);
                match extremes {
                    true => format!(
                        "count({column}), CAST(min({column}) AS VARCHAR), CAST(max({column}) AS VARCHAR)"
                    ),
                    false => format!("count({column}), NULL, NULL"),
                }
            })
            .collect::<Vec<_>>()
            .join(", ");
        conn.query_row(
            &format!("SELECT count(*), {fields} FROM {}", quote(view)),
            [],
            |r| {
                let rows = r.get::<_, u64>(0)?;
                (0..columns.len())
                    .map(|idx| {
                        Ok(ColumnStats {
                            nulls: rows - r.get::<_, u64>(idx * 3 + 1)?,
                            min: r.get(idx * 3 + 2)?,
                            max: r.get(idx * 3 + 3)?,
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
        names
            .into_iter()
            .filter_map(|name| {
                let view = ViewSource::new(self.session.session().ok()?, &name).ok()?;
                if !self.stats.contains_key(&name)
                    && let Some(stats) = self.session.stored_stats(&name)
                {
                    self.stats.insert(name.clone(), stats);
                }
                Some(TableEntry {
                    source: self.session.source_path(&name),
                    rows: view.row_count(),
                    columns: view.columns(),
                    stats: self.stats.get(&name).cloned(),
                    name,
                })
            })
            .collect()
    }
    pub fn request_stats(&mut self, view: &str) -> AppResult<()> {
        if self.stats.contains_key(view) || self.stats_jobs.iter().any(|job| job.view == view) {
            return Ok(());
        }
        if !self.session.names().any(|name| name == view) {
            return Err(anyhow!("Unknown table {view}"));
        }
        let conn = self.session.session()?;
        let columns = ViewSource::new(self.session.session()?, view)?
            .columns()
            .into_iter()
            .map(|(name, _)| name)
            .collect::<Vec<_>>();
        let (sender, result) = oneshot::channel();
        let name = view.to_string();
        std::thread::spawn(move || sender.send(stats(&conn, &name, &columns)));
        self.stats_jobs.push(StatsJob {
            view: view.to_string(),
            result,
        });
        Ok(())
    }
    pub fn forget_stats(&mut self, view: &str) {
        self.stats.remove(view);
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
        assert_eq!((tables[0].name.as_str(), tables[0].rows), ("sales", 3));
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
            },
            ColumnStats {
                nulls: 1,
                min: Some("a".to_string()),
                max: Some("b".to_string()),
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
        db.load(&other, None, &IngestOptions::default(), None).unwrap();
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
