mod cache;
mod ingester;
#[cfg(debug_assertions)]
mod inspect;
mod load;
mod parts;
pub mod reader;
mod results;
mod session;
mod source;
mod spool;
mod sql;

use crate::{AppResult, reader::ReaderConfig};
use anyhow::anyhow;
use cache::{CacheManager, config::Config};
use parts::Group;
use results::{Materializing, Scratch};
use session::Session;
use spool::Spool;
use sql::{SplitSql, random_name, split_sql};
use std::{
    path::{Path, PathBuf},
    sync::{Arc, atomic::AtomicBool},
};
use tokio::sync::oneshot::{Receiver, error::TryRecvError};

pub use cache::config::Config as DuckDBConfig;
pub(crate) use ingester::{cut_text, sample_columns};
pub use ingester::{CsvOptions, FwfOptions, IngestOptions, IngestProgress};
#[cfg(debug_assertions)]
pub use inspect::inspect;
pub use results::{Opened, Store};
pub use source::DataSource;

pub struct DuckDB {
    cache_manager: CacheManager,
    session: Session,
    spool: Spool,
    ingesting: Vec<Ingesting>,
    pending_queries: Vec<(String, Option<String>)>,
    groups: Vec<Group>,
    part_of: Option<usize>,
    reader: ReaderConfig,
    results: usize,
    materializing: Vec<Materializing>,
    _scratch: Scratch,
}

struct Ingesting {
    source: PathBuf,
    spooled: Option<PathBuf>,
    group: Option<usize>,
    alias: String,
    name: Option<String>,
    limit: Option<usize>,
    result: Receiver<AppResult<()>>,
}

pub enum Load {
    Ready(Vec<String>),
    Ingesting(Ingest),
}

pub struct Ingest {
    pub source: PathBuf,
    pub progress: Arc<dyn IngestProgress>,
    pub file: Option<PathBuf>,
    pub complete: Arc<AtomicBool>,
}

pub struct Finished {
    pub tables: Vec<(PathBuf, AppResult<Vec<String>>)>,
    pub queries: Vec<AppResult<String>>,
    pub results: Vec<String>,
    pub failed: Vec<(String, String)>,
}

impl DuckDB {
    pub fn new(config: Config) -> AppResult<Self> {
        let cache_manager = CacheManager::new(
            &config.cache_dir,
            config.max_cache_size,
            config.lru_capacity.unwrap_or(0),
        )?;
        let session = Session::new(config.allotted_ram)?;
        let scratch = Scratch::new(
            &config
                .temp_dir
                .unwrap_or_else(|| config.cache_dir.join("tmp")),
        )?;
        session.set_temp_dir(scratch.dir())?;
        session.execute_batch(&scratch.attach_sql())?;
        let spool = Spool::new(&config.cache_dir)?;
        Ok(Self {
            cache_manager,
            session,
            spool,
            ingesting: Vec::new(),
            pending_queries: Vec::new(),
            groups: Vec::new(),
            part_of: None,
            reader: ReaderConfig {
                segment_rows: config.segment_rows,
                segments: config.segments,
            },
            results: 0,
            materializing: Vec::new(),
            _scratch: scratch,
        })
    }
    pub fn load_sql(&mut self, path: &Path) -> AppResult<()> {
        let sql = std::fs::read_to_string(path)
            .map_err(|e| anyhow!("Failed to read {}: {}", path.display(), e))?;
        let stem = path
            .file_stem()
            .map(|stem| stem.to_string_lossy().into_owned());
        self.pending_queries.push((sql, stem));
        Ok(())
    }
    pub fn load_query(&mut self, sql: &str) {
        self.pending_queries.push((sql.to_string(), None));
    }
    pub fn detach(&mut self, name: &str) -> AppResult<()> {
        self.session.detach(name)
    }
    pub fn poll(&mut self) -> AppResult<Finished> {
        let (results, failed) = self.poll_results();
        let mut finished = Finished {
            tables: Vec::new(),
            queries: Vec::new(),
            results,
            failed,
        };
        let mut i = 0;
        while i < self.ingesting.len() {
            let result = match self.ingesting[i].result.try_recv() {
                Err(TryRecvError::Empty) => {
                    i += 1;
                    continue;
                }
                Ok(result) => result,
                Err(TryRecvError::Closed) => Err(anyhow!("Ingester stopped without a result")),
            };
            let ingesting = self.ingesting.swap_remove(i);
            if let Some(spooled) = &ingesting.spooled {
                let _ = std::fs::remove_file(spooled);
            }
            if let Some(id) = ingesting.group {
                let result = result.and_then(|()| self.complete(&ingesting.alias));
                if let Some(group) = self.groups.iter_mut().find(|group| group.id == id) {
                    group.waiting -= 1;
                    if let Err(e) = result {
                        group.error.get_or_insert(e.to_string());
                    }
                }
                continue;
            }
            let names = result.and_then(|()| self.finish(&ingesting));
            self.session.unpin(&ingesting.alias);
            finished.tables.push((ingesting.source, names));
        }
        while let Some(index) = self.groups.iter().position(|group| group.waiting == 0) {
            let group = self.groups.remove(index);
            let names = self.complete_group(&group);
            if group.reported {
                finished.tables.push((group.source, names));
            }
        }
        if self.ingesting.is_empty() {
            for (sql, fallback) in std::mem::take(&mut self.pending_queries) {
                finished.queries.push(self.query(&sql, fallback));
            }
        }
        self.session.flush()?;
        Ok(finished)
    }
    fn views(
        &mut self,
        alias: &str,
        name: Option<&str>,
        limit: Option<usize>,
    ) -> AppResult<Vec<String>> {
        let tables = self.session.tables(alias)?;
        if tables.is_empty() {
            return Err(anyhow!("No tables were loaded"));
        }
        let single = tables.len() == 1 && tables[0] == "data";
        tables
            .iter()
            .map(|table| {
                let view = if single { name.unwrap_or(table) } else { table };
                self.session.create_view(view, alias, table, limit)
            })
            .collect()
    }
    fn query(&mut self, sql: &str, fallback: Option<String>) -> AppResult<String> {
        let SplitSql { setup, name, query } = split_sql(sql);
        let name = name.or(fallback).unwrap_or_else(random_name);
        self.session.create_query(&name, &setup, &query)
    }
    fn complete(&mut self, alias: &str) -> AppResult<()> {
        self.session
            .file_db_mut(alias)
            .ok_or_else(|| anyhow!("Unknown alias {alias}"))?
            .mark_complete()
    }
    fn finish(&mut self, ingesting: &Ingesting) -> AppResult<Vec<String>> {
        self.complete(&ingesting.alias)?;
        self.views(&ingesting.alias, ingesting.name.as_deref(), ingesting.limit)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::duckdb::{cache::file_db::FileDBMetadata, sql::quote};
    use tempfile::TempDir;
    use tokio::sync::oneshot::{self, Sender};

    fn duckdb(dir: &TempDir) -> DuckDB {
        DuckDB::new(Config {
            cache_dir: dir.path().join("cache"),
            ..Config::default()
        })
        .unwrap()
    }

    fn source(dir: &TempDir, name: &str) -> PathBuf {
        let path = dir.path().join(name);
        std::fs::write(&path, "id\n1\n").unwrap();
        path.canonicalize().unwrap()
    }

    fn start(db: &mut DuckDB, path: &Path, tables: &[&str]) -> (String, Sender<AppResult<()>>) {
        let key = serde_json::to_value(IngestOptions::default()).unwrap();
        let file_db = db.cache_manager.create_new_db(path, &key).unwrap();
        let alias = db.session.attach(Some(file_db)).unwrap();
        for table in tables {
            db.session
                .session()
                .unwrap()
                .execute_batch(&format!(
                    "CREATE TABLE {}.{} AS SELECT range AS id FROM range(10)",
                    quote(&alias),
                    quote(table)
                ))
                .unwrap();
        }
        let (sender, result) = oneshot::channel();
        db.ingesting.push(Ingesting {
            source: path.to_path_buf(),
            spooled: None,
            group: None,
            alias: alias.clone(),
            name: Some("sales".to_string()),
            limit: None,
            result,
        });
        (alias, sender)
    }

    fn count(db: &DuckDB, name: &str) -> i64 {
        db.session
            .session()
            .unwrap()
            .query_row(&format!("SELECT count(*) FROM {}", quote(name)), [], |r| {
                r.get(0)
            })
            .unwrap()
    }

    #[test]
    fn finished_ingest_is_marked_complete_and_gets_a_view() {
        let dir = TempDir::new().unwrap();
        let mut db = duckdb(&dir);
        let path = source(&dir, "sales.csv");
        let (alias, sender) = start(&mut db, &path, &["data"]);
        assert!(db.poll().unwrap().tables.is_empty());
        sender.send(Ok(())).unwrap();
        let finished = db.poll().unwrap();
        assert_eq!(finished.tables.len(), 1);
        assert_eq!(finished.tables[0].0, path);
        assert_eq!(finished.tables[0].1.as_ref().unwrap(), &["sales"]);
        assert_eq!(count(&db, "sales"), 10);
        let file_db = db.session.file_db_mut(&alias).unwrap();
        assert!(FileDBMetadata::is_complete(file_db.file_path()));
    }

    #[test]
    fn every_ingested_table_gets_its_own_view() {
        let dir = TempDir::new().unwrap();
        let mut db = duckdb(&dir);
        let (_, sender) = start(&mut db, &source(&dir, "shop.sqlite"), &["users", "orders"]);
        sender.send(Ok(())).unwrap();
        let finished = db.poll().unwrap();
        assert_eq!(finished.tables[0].1.as_ref().unwrap(), &["orders", "users"]);
        assert_eq!(count(&db, "orders"), 10);
    }

    #[test]
    fn failed_dropped_or_empty_ingest_detaches_the_db() {
        let dir = TempDir::new().unwrap();
        let mut db = duckdb(&dir);
        let (failed, sender) = start(&mut db, &source(&dir, "one.csv"), &["data"]);
        sender.send(Err(anyhow!("broken file"))).unwrap();
        let (dropped, sender) = start(&mut db, &source(&dir, "two.csv"), &["data"]);
        drop(sender);
        let (empty, sender) = start(&mut db, &source(&dir, "three.csv"), &[]);
        sender.send(Ok(())).unwrap();
        let finished = db.poll().unwrap();
        assert_eq!(finished.tables.len(), 3);
        assert!(finished.tables.iter().all(|(_, result)| result.is_err()));
        for alias in [failed, dropped, empty] {
            assert!(db.session.file_db_mut(&alias).is_none());
        }
    }

    #[test]
    fn complete_cache_loads_without_ingesting() {
        let dir = TempDir::new().unwrap();
        let path = source(&dir, "sales.csv");
        let mut db = duckdb(&dir);
        let (_, sender) = start(&mut db, &path, &["data"]);
        sender.send(Ok(())).unwrap();
        db.poll().unwrap();
        db.detach("sales").unwrap();
        db.poll().unwrap();
        let Load::Ready(names) = db
            .load(&path, None, &IngestOptions::default(), None)
            .unwrap()
        else {
            panic!("expected a cache hit");
        };
        assert_eq!(names, ["sales"]);
        assert_eq!(count(&db, "sales"), 10);
    }

    #[test]
    fn queries_wait_for_every_ingest() {
        let dir = TempDir::new().unwrap();
        let mut db = duckdb(&dir);
        let (_, sender) = start(&mut db, &source(&dir, "sales.csv"), &["data"]);
        db.load_query("-- name: total\nSELECT count(*) AS n FROM sales");
        assert!(db.poll().unwrap().queries.is_empty());
        sender.send(Ok(())).unwrap();
        let finished = db.poll().unwrap();
        assert_eq!(finished.tables.len(), 1);
        assert_eq!(finished.queries.len(), 1);
        assert_eq!(finished.queries[0].as_deref().unwrap(), "total");
        assert_eq!(count(&db, "total"), 1);
    }

    #[test]
    fn sql_files_are_named_by_the_sql_then_the_file_stem() {
        let dir = TempDir::new().unwrap();
        let mut db = duckdb(&dir);
        let stem = dir.path().join("top_sales.sql");
        std::fs::write(&stem, "SELECT 42 AS n").unwrap();
        let named = dir.path().join("other.sql");
        std::fs::write(&named, "CREATE VIEW best AS SELECT 1").unwrap();
        db.load_sql(&stem).unwrap();
        db.load_sql(&named).unwrap();
        let queries = db.poll().unwrap().queries;
        let names: Vec<_> = queries.into_iter().map(Result::unwrap).collect();
        assert_eq!(names, ["top_sales", "best"]);
        assert!(db.load_sql(&dir.path().join("missing.sql")).is_err());
    }

    #[test]
    fn bad_query_is_reported_once() {
        let dir = TempDir::new().unwrap();
        let mut db = duckdb(&dir);
        db.load_query("-- name: bad\nSELECT * FROM nowhere");
        let queries = db.poll().unwrap().queries;
        assert_eq!(queries.len(), 1);
        assert!(queries[0].is_err());
        assert!(db.poll().unwrap().queries.is_empty());
    }
}
