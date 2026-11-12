use crate::{
    AppResult,
    duckdb::{
        DuckDB,
        ingester::{IngestProgress, progress::StepProgress},
        reader::{Count, ViewSource, is_float},
        skim,
        sql::{SplitSql, quote, split_sql},
    },
    misc::workers,
    reader::{Reader, RowSource},
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
    count: Option<Count>,
    result: Receiver<AppResult<()>>,
}

pub(super) struct Polled {
    pub(super) ready: Vec<String>,
    pub(super) swapped: Vec<String>,
    pub(super) failed: Vec<(String, String)>,
}

fn bind(query: &str, current: Option<&str>) -> String {
    let query = query.trim().trim_end_matches(';');
    match current {
        Some(current) => format!(
            "WITH _ AS (SELECT * FROM {}) SELECT * FROM ({query})",
            quote(current)
        ),
        None => query.to_string(),
    }
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
        self.open(sql, current, store, &format!("Running {sql}"), None)
    }
    fn open(
        &mut self,
        sql: &str,
        current: Option<&str>,
        store: Store,
        title: &str,
        stored: Option<&str>,
    ) -> AppResult<Opened> {
        let bind = |query: &str| bind(query, current);
        let SplitSql { setup, query, .. } = split_sql(sql);
        let query = bind(&query);
        self.results += 1;
        let view = format!("{RESULT_PREFIX}{}", self.results);
        let select = stored
            .map(bind)
            .unwrap_or_else(|| format!("SELECT * FROM {}", quote(&view)));
        let mut batch = setup;
        batch.push(format!("CREATE VIEW {} AS {query}", quote(&view)));
        self.session.execute_batch(&batch.join(";\n"))?;
        let opened = match store {
            Store::View => self
                .reader(&view)
                .map(|reader| Opened::Ready(Box::new(reader))),
            Store::Background => self.growing_reader(&view).and_then(|(reader, count)| {
                self.materialize(&view, title, &select, Some(count))?;
                Ok(Opened::Ready(Box::new(reader)))
            }),
            Store::Wait => self
                .materialize(&view, title, &select, None)
                .map(|progress| Opened::Pending(view.clone(), progress)),
        };
        if opened.is_err() {
            self.drop_result(&view);
        }
        opened
    }
    pub fn open_search(&mut self, view: &str, pattern: &str, fuzzy: bool) -> AppResult<Opened> {
        let columns = ViewSource::growing(self.session.session()?, view)?.columns();
        if columns.is_empty() {
            return Err(anyhow!("Nothing to search in {view}"));
        }
        let literal = |text: &str| format!("'{}'", text.replace('\'', "''"));
        let like = pattern.chars().fold(String::from("%"), |mut like, c| {
            if matches!(c, '%' | '_' | '!') {
                like.push('!');
            }
            like.push(c);
            like.push('%');
            like
        });
        let cells = |test: &dyn Fn(String) -> String, separator: &str| {
            columns
                .iter()
                .map(|(name, data_type)| {
                    test(match is_float(data_type) {
                        true => format!("{}({})", skim::FLOAT_TEXT, quote(name)),
                        false => format!("CAST({} AS VARCHAR)", quote(name)),
                    })
                })
                .collect::<Vec<_>>()
                .join(separator)
        };
        let (matched, stored) = match fuzzy {
            true => {
                let matched = cells(
                    &|text| format!("{text} ILIKE {} ESCAPE '!'", literal(&like)),
                    " OR ",
                );
                let score = cells(
                    &|text| format!("{}({text}, {})", skim::NAME, literal(pattern)),
                    ", ",
                );
                let stored = format!(
                    "WITH _scored AS MATERIALIZED (SELECT *, greatest({score}) AS _score FROM _ WHERE {matched}), _numbered AS (SELECT *, row_number() OVER () AS _row FROM _scored WHERE _score IS NOT NULL) SELECT * EXCLUDE (_score, _row) FROM _numbered ORDER BY _score DESC, _row"
                );
                (matched, stored)
            }
            false => {
                let matched = cells(
                    &|text| format!("contains({text}, {})", literal(pattern)),
                    " OR ",
                );
                let exact = cells(&|text| format!("{text} = {}", literal(pattern)), " OR ");
                let stored = format!(
                    "WITH _matched AS MATERIALIZED (SELECT *, coalesce({exact}, false) AS _exact FROM _ WHERE {matched}) SELECT * EXCLUDE (_exact) FROM _matched WHERE _exact UNION ALL SELECT * EXCLUDE (_exact) FROM _matched WHERE NOT _exact"
                );
                (matched, stored)
            }
        };
        self.open(
            &format!("SELECT * FROM _ WHERE {matched}"),
            Some(view),
            Store::Background,
            &format!("Searching {pattern}"),
            Some(&stored),
        )
    }
    pub(super) fn materialize(
        &mut self,
        view: &str,
        title: &str,
        select: &str,
        count: Option<Count>,
    ) -> AppResult<Arc<dyn IngestProgress>> {
        let conn = self.session.session()?;
        conn.set_query_progress_tracking(true)?;
        let progress = StepProgress::new(1);
        progress.start(&conn);
        let interrupt = conn.interrupt_handle();
        let sql = format!("CREATE TABLE {} AS {select}", table(view));
        let (sender, result) = oneshot::channel();
        workers::spawn(move || {
            let _ = sender.send(conn.execute_batch(&sql).map_err(Into::into));
        });
        self.materializing.push(Materializing {
            view: view.to_string(),
            title: title.to_string(),
            progress: progress.clone(),
            interrupt,
            dropped: false,
            waiting: count.is_none(),
            count,
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
            .chain(
                self.exporting
                    .iter()
                    .map(|exporting| (exporting.title.as_str(), exporting.progress.fraction())),
            )
            .chain(
                self.plotting
                    .iter()
                    .map(|plotting| (plotting.title.as_str(), plotting.progress.fraction())),
            )
    }
    pub fn result_progress(&self, view: &str) -> Option<Arc<dyn IngestProgress>> {
        self.materializing
            .iter()
            .find(|materializing| materializing.view == view && !materializing.dropped)
            .map(|materializing| materializing.progress.clone() as Arc<dyn IngestProgress>)
    }
    pub fn query_shape(&self, sql: &str, current: Option<&str>) -> AppResult<(usize, usize)> {
        let query = bind(&split_sql(sql).query, current);
        let conn = self.session.session()?;
        let width = conn
            .prepare(&format!("DESCRIBE SELECT * FROM ({query})"))?
            .query_map([], |_| Ok(()))?
            .count();
        let height = conn.query_row(&format!("SELECT count(*) FROM ({query})"), [], |r| {
            r.get::<_, i64>(0)
        })? as usize;
        Ok((height, width))
    }
    pub fn rows_json(&self, view: &str, offset: usize, count: usize) -> AppResult<String> {
        let rows = self
            .session
            .session()?
            .prepare(&format!(
                "SELECT CAST(to_json(t) AS VARCHAR) FROM (SELECT * FROM {} LIMIT {count} OFFSET {offset}) t",
                quote(view)
            ))?
            .query_map([], |r| r.get::<_, String>(0))?
            .collect::<Result<Vec<_>, _>>()?;
        Ok(format!("[{}]", rows.join(",")))
    }
    pub fn scratch_path(&self, name: &str) -> PathBuf {
        self._scratch.dir().join(name)
    }
    pub fn register(&mut self, name: &str, current: &str) -> AppResult<String> {
        let name = name.trim();
        if name.is_empty() || name == "_" || name.starts_with(RESULT_PREFIX) {
            return Err(anyhow!("'{name}' is not a valid name"));
        }
        if self
            .session
            .names()
            .any(|existing| existing.eq_ignore_ascii_case(name))
        {
            return Err(anyhow!("Table name '{name}' already exists"));
        }
        let select = format!("SELECT * FROM {}", quote(current));
        let name = self.session.create_query(name, &[], &select)?;
        self.materialize(
            &name,
            &format!("Registering {name}"),
            &select,
            Some(Count::default()),
        )?;
        Ok(name)
    }
    pub(super) fn scratch_size(&self, name: &str) -> Option<u64> {
        self.session
            .session()
            .ok()?
            .query_row(
                &format!(
                    "SELECT count(DISTINCT block_id) * (SELECT block_size FROM pragma_database_size() WHERE database_name = '{SCRATCH}') FROM pragma_storage_info(?) WHERE persistent"
                ),
                [table(name)],
                |r| r.get::<_, Option<i64>>(0),
            )
            .ok()
            .flatten()
            .filter(|bytes| *bytes > 0)
            .map(|bytes| bytes as u64)
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
    pub(super) fn poll_results(&mut self) -> Polled {
        let mut polled = Polled {
            ready: Vec::new(),
            swapped: Vec::new(),
            failed: Vec::new(),
        };
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
                count,
                ..
            } = self.materializing.swap_remove(i);
            let swapped = match dropped {
                true => Err(anyhow!("Dropped")),
                false => result.and_then(|()| {
                    self.session.execute_batch(&format!(
                        "CREATE OR REPLACE VIEW {} AS SELECT * FROM {}; CHECKPOINT {SCRATCH}",
                        quote(&view),
                        table(&view)
                    ))
                }),
            };
            match swapped {
                Ok(()) if waiting => polled.ready.push(view),
                Ok(()) => {
                    if let Some(count) = count
                        && let Ok(rows) = self.session.session().and_then(|conn| {
                            Ok(conn.query_row(
                                &format!("SELECT count(*) FROM {}", table(&view)),
                                [],
                                |r| r.get::<_, i64>(0),
                            )?)
                        })
                    {
                        let _ = count.set(rows as usize);
                    }
                    polled.swapped.push(view);
                }
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
                        polled.failed.push((view, format!("{title}: {err}")));
                    }
                }
            }
        }
        polled
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::duckdb::DuckDBConfig;
    use fuzzy_matcher::{FuzzyMatcher, skim::SkimMatcherV2};
    use std::{
        cmp::Reverse,
        time::{Duration, Instant},
    };
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
        assert!(!ordered.source().complete());
        assert_eq!(first(&mut ordered), expected);
        assert_eq!(ordered.row_count(), 500);
        assert!(ordered.source().complete());
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

    #[test]
    fn search_matches_any_column_with_exact_cells_first() {
        let dir = TempDir::new().unwrap();
        let mut db = DuckDB::new(DuckDBConfig {
            cache_dir: dir.path().join("cache"),
            ..DuckDBConfig::default()
        })
        .unwrap();
        let Opened::Ready(base) = db
            .open_query(
                "SELECT * FROM (VALUES (1, 'it''s 50% off'), (2, 'Xan'), (3, 'an'), (40, NULL), (5, 'b_n'), (6, 'a big name'), (7, 'banana'), (8, 'nan'), (9, 'A N')) t(id, note)",
                None,
                Store::View,
            )
            .unwrap()
        else {
            panic!("pending");
        };
        let view = base.source().view().unwrap().to_string();
        let Opened::Ready(floats) = db
            .open_query(
                "SELECT * FROM (VALUES (1, 77.5::DOUBLE), (2, 77.0), (3, 1e20), (4, 0.00001)) t(id, x)",
                None,
                Store::View,
            )
            .unwrap()
        else {
            panic!("pending");
        };
        let floats = floats.source().view().unwrap().to_string();
        let mut early = Vec::new();
        let mut ids = |view: &str, pattern: &str, fuzzy: bool| {
            let Opened::Ready(mut reader) = db.open_search(view, pattern, fuzzy).unwrap() else {
                panic!("pending");
            };
            let read = |reader: &mut Reader<ViewSource>| {
                reader.reset();
                reader
                    .rows(0..20)
                    .unwrap()
                    .map(|row| row[0].clone().unwrap())
                    .collect::<Vec<_>>()
            };
            early.push(read(&mut reader));
            let name = reader.source().view().unwrap().to_string();
            let start = Instant::now();
            while !db.poll().unwrap().swapped.contains(&name) {
                assert!(start.elapsed() < Duration::from_secs(10));
                std::thread::sleep(Duration::from_millis(5));
            }
            read(&mut reader)
        };
        assert_eq!(ids(&view, "an", false), ["3", "2", "7", "8"]);
        assert_eq!(ids(&view, "4", false), ["40"]);
        assert_eq!(ids(&view, "it's 50%", false), ["1"]);
        assert_eq!(ids(&view, "AN", false), [""; 0]);
        let rows = [
            (1, Some("it's 50% off")),
            (2, Some("Xan")),
            (3, Some("an")),
            (40, None),
            (5, Some("b_n")),
            (6, Some("a big name")),
            (7, Some("banana")),
            (8, Some("nan")),
            (9, Some("A N")),
        ];
        let matcher = SkimMatcherV2::default();
        for pattern in ["an", "AN", "xn", "b_n", "5%f", "n", "A", "a n", "zz"] {
            let mut scored = rows
                .iter()
                .enumerate()
                .filter_map(|(index, (id, note))| {
                    [Some(id.to_string()), note.map(String::from)]
                        .into_iter()
                        .flatten()
                        .filter_map(|cell| match cell == pattern {
                            true => Some(i64::MAX),
                            false => matcher.fuzzy_match(&cell, pattern),
                        })
                        .max()
                        .map(|score| (Reverse(score), index, id.to_string()))
                })
                .collect::<Vec<_>>();
            scored.sort();
            let expected = scored.into_iter().map(|(.., id)| id).collect::<Vec<_>>();
            assert_eq!(ids(&view, pattern, true), expected, "{pattern}");
        }
        assert_eq!(ids(&floats, "77", false), ["2", "1"]);
        assert_eq!(ids(&floats, "77.0", false), [""; 0]);
        assert_eq!(ids(&floats, "000", true), ["4", "3"]);
        assert_eq!(early[0], ["2", "3", "7", "8"]);
    }
}
