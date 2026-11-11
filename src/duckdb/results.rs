use crate::{
    AppResult,
    duckdb::{
        DuckDB,
        ingester::{IngestProgress, progress::StepProgress},
        reader::ViewSource,
        sql::{SplitSql, quote, split_sql},
    },
    reader::Reader,
};
use anyhow::anyhow;
use duckdb::InterruptHandle;
use std::{
    fs::File,
    path::{Path, PathBuf},
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
};
use tokio::sync::oneshot::{self, Receiver, error::TryRecvError};

const SCRATCH: &str = "_scratch";
const DIR_PREFIX: &str = "tabiew-";
const RESULT_PREFIX: &str = "_result_";

static INSTANCES: AtomicUsize = AtomicUsize::new(0);

pub(super) struct Scratch {
    dir: PathBuf,
    _lock: File,
}

impl Scratch {
    pub(super) fn new(base: &Path) -> AppResult<Self> {
        for dir in std::fs::read_dir(base)
            .into_iter()
            .flatten()
            .flatten()
            .map(|entry| entry.path())
            .filter(|path| {
                path.file_name()
                    .is_some_and(|name| name.to_string_lossy().starts_with(DIR_PREFIX))
            })
        {
            if File::open(dir.join("lock")).is_ok_and(|lock| lock.try_lock().is_ok()) {
                let _ = std::fs::remove_dir_all(&dir);
            }
        }
        let dir = base.join(format!(
            "{DIR_PREFIX}{}-{}",
            std::process::id(),
            INSTANCES.fetch_add(1, Ordering::Relaxed)
        ));
        std::fs::create_dir_all(&dir)
            .map_err(|e| anyhow!("Failed to create {}: {}", dir.display(), e))?;
        let lock = File::create(dir.join("lock"))?;
        lock.try_lock()
            .map_err(|e| anyhow!("Failed to lock {}: {}", dir.display(), e))?;
        Ok(Self { dir, _lock: lock })
    }
    pub(super) fn dir(&self) -> &Path {
        &self.dir
    }
    pub(super) fn attach_sql(&self) -> String {
        format!(
            "ATTACH '{}' AS {SCRATCH}",
            self.dir
                .join("scratch.duckdb")
                .to_string_lossy()
                .replace('\'', "''")
        )
    }
}

impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.dir);
    }
}

pub enum Store {
    View,
    Background,
    Wait,
}

pub enum Opened {
    Ready(Box<Reader<ViewSource>>),
    Pending(String, Arc<dyn IngestProgress>),
}

pub(super) struct Materializing {
    view: String,
    title: String,
    progress: Arc<StepProgress>,
    interrupt: Arc<InterruptHandle>,
    dropped: bool,
    waiting: bool,
    result: Receiver<AppResult<()>>,
}

fn table(view: &str) -> String {
    format!("{SCRATCH}.main.{}", quote(view))
}

impl DuckDB {
    pub fn open_query(
        &mut self,
        sql: &str,
        current: Option<&str>,
        store: Store,
    ) -> AppResult<Opened> {
        let SplitSql { setup, query, .. } = split_sql(sql);
        let query = query.trim().trim_end_matches(';');
        let query = match current {
            Some(current) => format!(
                "WITH _ AS (SELECT * FROM {}) SELECT * FROM ({query})",
                quote(current)
            ),
            None => query.to_string(),
        };
        self.results += 1;
        let view = format!("{RESULT_PREFIX}{}", self.results);
        let mut batch = setup;
        batch.push(format!("CREATE VIEW {} AS {query}", quote(&view)));
        self.session.execute_batch(&batch.join(";\n"))?;
        let opened = match store {
            Store::View => self
                .reader(&view)
                .map(|reader| Opened::Ready(Box::new(reader))),
            Store::Background => self.reader(&view).and_then(|reader| {
                self.materialize(&view, sql, false)?;
                Ok(Opened::Ready(Box::new(reader)))
            }),
            Store::Wait => self
                .materialize(&view, sql, true)
                .map(|progress| Opened::Pending(view.clone(), progress)),
        };
        if opened.is_err() {
            self.drop_result(&view);
        }
        opened
    }
    fn materialize(
        &mut self,
        view: &str,
        title: &str,
        waiting: bool,
    ) -> AppResult<Arc<dyn IngestProgress>> {
        let conn = self.session.session()?;
        conn.set_query_progress_tracking(true)?;
        let progress = StepProgress::new(1);
        progress.start(&conn);
        let interrupt = conn.interrupt_handle();
        let sql = format!(
            "CREATE TABLE {} AS SELECT * FROM {}",
            table(view),
            quote(view)
        );
        let (sender, result) = oneshot::channel();
        std::thread::spawn(move || sender.send(conn.execute_batch(&sql).map_err(Into::into)));
        self.materializing.push(Materializing {
            view: view.to_string(),
            title: title.to_string(),
            progress: progress.clone(),
            interrupt,
            dropped: false,
            waiting,
            result,
        });
        Ok(progress)
    }
    pub fn materializing(&self) -> impl Iterator<Item = (&str, Option<f64>)> {
        self.materializing
            .iter()
            .filter(|materializing| !materializing.dropped && !materializing.waiting)
            .map(|materializing| {
                (
                    materializing.title.as_str(),
                    materializing.progress.fraction(),
                )
            })
    }
    pub fn drop_result(&mut self, view: &str) {
        if !view.starts_with(RESULT_PREFIX) {
            return;
        }
        let _ = self
            .session
            .execute_batch(&format!("DROP VIEW IF EXISTS {}", quote(view)));
        match self
            .materializing
            .iter_mut()
            .find(|materializing| materializing.view == view)
        {
            Some(materializing) => {
                materializing.dropped = true;
                materializing.interrupt.interrupt();
            }
            None => {
                let _ = self
                    .session
                    .execute_batch(&format!("DROP TABLE IF EXISTS {}", table(view)));
            }
        }
    }
    pub(super) fn poll_results(&mut self) -> (Vec<String>, Vec<(String, String)>) {
        let mut ready = Vec::new();
        let mut errors = Vec::new();
        let mut i = 0;
        while i < self.materializing.len() {
            let result = match self.materializing[i].result.try_recv() {
                Err(TryRecvError::Empty) => {
                    i += 1;
                    continue;
                }
                Ok(result) => result,
                Err(TryRecvError::Closed) => Err(anyhow!("Query stopped without a result")),
            };
            let Materializing {
                view,
                title,
                dropped,
                waiting,
                ..
            } = self.materializing.swap_remove(i);
            let swapped = match dropped {
                true => Err(anyhow!("Dropped")),
                false => result.and_then(|()| {
                    self.session.execute_batch(&format!(
                        "CREATE OR REPLACE VIEW {} AS SELECT * FROM {}",
                        quote(&view),
                        table(&view)
                    ))
                }),
            };
            match swapped {
                Ok(()) if waiting => ready.push(view),
                Ok(()) => (),
                Err(err) => {
                    let _ = self
                        .session
                        .execute_batch(&format!("DROP TABLE IF EXISTS {}", table(&view)));
                    if waiting {
                        let _ = self
                            .session
                            .execute_batch(&format!("DROP VIEW IF EXISTS {}", quote(&view)));
                    }
                    if !dropped {
                        errors.push((view, format!("{title}: {err}")));
                    }
                }
            }
        }
        (ready, errors)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{duckdb::DuckDBConfig, reader::RowSource};
    use std::time::{Duration, Instant};
    use tempfile::TempDir;

    #[test]
    fn query_results_are_served_from_a_view_then_from_a_scratch_table() {
        let dir = TempDir::new().unwrap();
        let tmp = dir.path().join("cache/tmp");
        let stale = tmp.join("tabiew-stale");
        std::fs::create_dir_all(&stale).unwrap();
        File::create(stale.join("lock")).unwrap();
        let mut db = DuckDB::new(DuckDBConfig {
            cache_dir: dir.path().join("cache"),
            ..DuckDBConfig::default()
        })
        .unwrap();
        assert!(!stale.exists());
        let scratch = db._scratch.dir().to_path_buf();
        let ready = |opened| match opened {
            Opened::Ready(reader) => *reader,
            Opened::Pending(..) => panic!("pending"),
        };
        let mut base = ready(
            db.open_query("SELECT range AS id FROM range(1000)", None, Store::View)
                .unwrap(),
        );
        let view = base.source().view().unwrap().to_string();
        assert!(db.materializing().next().is_none());
        let query = "SELECT id FROM _ WHERE id % 2 = 1 ORDER BY id DESC";
        let mut ordered = ready(
            db.open_query(query, Some(&view), Store::Background)
                .unwrap(),
        );
        let first = |reader: &mut Reader<ViewSource>| {
            reader
                .rows(0..2)
                .unwrap()
                .map(<[_]>::to_vec)
                .collect::<Vec<_>>()
        };
        let expected = [[Some("999".to_string())], [Some("997".to_string())]];
        assert_eq!(ordered.row_count(), 500);
        assert_eq!(first(&mut ordered), expected);
        assert_eq!(db.materializing().count(), 1);
        let start = Instant::now();
        while db.materializing().next().is_some() {
            assert!(start.elapsed() < Duration::from_secs(10));
            assert!(db.poll().unwrap().failed.is_empty());
            std::thread::sleep(Duration::from_millis(10));
        }
        let name = ordered.source().view().unwrap().to_string();
        let definition: String = db
            .session
            .session()
            .unwrap()
            .query_row(
                "SELECT sql FROM duckdb_views() WHERE view_name = ?",
                [&name],
                |r| r.get(0),
            )
            .unwrap();
        assert!(definition.contains(SCRATCH));
        ordered.reset();
        assert_eq!(first(&mut ordered), expected);
        db.drop_result(&name);
        ordered.reset();
        assert!(ordered.rows(0..2).is_err());
        assert_eq!(first(&mut base)[1], [Some("1".to_string())]);
        assert!(
            db.open_query("SELECT nope FROM _", Some(&view), Store::Wait)
                .is_err()
        );
        let Opened::Pending(pending, _) = db.open_query(query, Some(&view), Store::Wait).unwrap()
        else {
            panic!("ready");
        };
        assert!(db.materializing().next().is_none());
        let start = Instant::now();
        let ready = loop {
            assert!(start.elapsed() < Duration::from_secs(10));
            let finished = db.poll().unwrap();
            assert!(finished.failed.is_empty());
            if let Some(view) = finished.results.into_iter().next() {
                break view;
            }
            std::thread::sleep(Duration::from_millis(10));
        };
        assert_eq!(ready, pending);
        assert_eq!(first(&mut db.reader(&ready).unwrap()), expected);
        drop((base, ordered, db));
        assert!(!scratch.exists());
    }
}
