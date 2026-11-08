mod cache;
mod config;
mod file_db;
mod helper;
mod ingester;
pub mod reader;
mod session;

use crate::{AppResult, args::Format};
use anyhow::anyhow;
use cache::CacheManager;
use config::Config;
use duckdb::ProgressHandle;
use helper::{SplitSql, random_name, split_sql};
use ingester::Ingester;
use session::Session;
use std::path::Path;
use tokio::sync::oneshot::{Receiver, error::TryRecvError};

pub use config::Config as DuckDBConfig;

pub struct DuckDB {
    cache_manager: CacheManager,
    config: Config,
    session: Session,
    ingesting: Vec<(String, Receiver<AppResult<()>>)>,
    pending_queries: Vec<(String, Option<String>)>,
}

pub struct Flushed {
    pub tables: Vec<(String, AppResult<()>)>,
    pub queries: Vec<AppResult<String>>,
}

impl DuckDB {
    pub fn new(config: Config) -> AppResult<Self> {
        let cache_manager = CacheManager::new(
            &config.cache_dir,
            config.max_cache_size,
            config.lru_capacity.unwrap_or(0),
        )?;
        let session = Session::new(config.allotted_ram)?;
        Ok(Self {
            cache_manager,
            config,
            session,
            ingesting: Vec::new(),
            pending_queries: Vec::new(),
        })
    }
    pub fn load(
        &mut self,
        path: &Path,
        table_name: Option<&str>,
        format: Option<Format>,
    ) -> AppResult<(String, Option<ProgressHandle>)> {
        let stem = path
            .file_stem()
            .map(|stem| stem.to_string_lossy().into_owned());
        let table_name = table_name.or(stem.as_deref());
        if let Some(file_db) = self.cache_manager.check_existing_cache_db(path)? {
            let name = self.attach(table_name, file_db)?;
            self.session.create_view(&name)?;
            return Ok((name, None));
        }
        let file_db = self.cache_manager.create_new_db(path)?;
        let name = self.attach(table_name, file_db)?;
        match self.ingest(&name, path, format) {
            Ok(progress) => Ok((name, Some(progress))),
            Err(e) => {
                self.session.detach(&name)?;
                Err(e)
            }
        }
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
    pub fn flush(&mut self) -> AppResult<Flushed> {
        let mut flushed = Flushed {
            tables: Vec::new(),
            queries: Vec::new(),
        };
        let mut i = 0;
        while i < self.ingesting.len() {
            let result = match self.ingesting[i].1.try_recv() {
                Err(TryRecvError::Empty) => {
                    i += 1;
                    continue;
                }
                Ok(result) => result,
                Err(TryRecvError::Closed) => Err(anyhow!("Ingester stopped without a result")),
            };
            let (name, _) = self.ingesting.swap_remove(i);
            let result = result.and_then(|()| self.finish(&name));
            if result.is_err() {
                self.session.detach(&name)?;
            }
            flushed.tables.push((name, result));
        }
        if self.ingesting.is_empty() {
            for (sql, fallback) in std::mem::take(&mut self.pending_queries) {
                flushed.queries.push(self.query(&sql, fallback));
            }
        }
        self.session.flush()?;
        Ok(flushed)
    }
    fn attach(&mut self, table_name: Option<&str>, file_db: file_db::FileDB) -> AppResult<String> {
        self.session
            .attach(table_name, Some(file_db))?
            .ok_or_else(|| anyhow!("Failed to attach the table"))
    }
    fn ingest(
        &mut self,
        name: &str,
        path: &Path,
        format: Option<Format>,
    ) -> AppResult<ProgressHandle> {
        let alias = self
            .session
            .alias(name)
            .ok_or_else(|| anyhow!("Unknown table {name}"))?;
        let ingester = Ingester::new(self.session.session()?, alias, path.to_path_buf(), format)?;
        let (progress, result) = ingester.ingest()?;
        self.ingesting.push((name.to_string(), result));
        Ok(progress)
    }
    fn query(&mut self, sql: &str, fallback: Option<String>) -> AppResult<String> {
        let SplitSql { setup, name, query } = split_sql(sql);
        let name = name.or(fallback).unwrap_or_else(random_name);
        self.session.create_query(&name, &setup, &query)
    }
    fn finish(&mut self, name: &str) -> AppResult<()> {
        self.session
            .file_db_mut(name)
            .ok_or_else(|| anyhow!("Unknown table {name}"))?
            .mark_complete()?;
        self.session.create_view(name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::duckdb::{file_db::FileDBMetadata, helper::quote};
    use tempfile::TempDir;
    use tokio::sync::oneshot::{self, Sender};

    fn duckdb(dir: &TempDir) -> DuckDB {
        DuckDB::new(Config {
            cache_dir: dir.path().join("cache"),
            ..Config::default()
        })
        .unwrap()
    }

    fn source(dir: &TempDir, name: &str) -> std::path::PathBuf {
        let path = dir.path().join(name);
        std::fs::write(&path, "id\n1\n").unwrap();
        path
    }

    fn start(db: &mut DuckDB, path: &Path) -> (String, Sender<AppResult<()>>) {
        let file_db = db.cache_manager.create_new_db(path).unwrap();
        let name = db.attach(Some("sales"), file_db).unwrap();
        let alias = db.session.alias(&name).unwrap();
        db.session
            .session()
            .unwrap()
            .execute_batch(&format!(
                "CREATE TABLE {}.data AS SELECT range AS id FROM range(10)",
                quote(&alias)
            ))
            .unwrap();
        let (sender, receiver) = oneshot::channel();
        db.ingesting.push((name.clone(), receiver));
        (name, sender)
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
        let (name, sender) = start(&mut db, &source(&dir, "sales.csv"));
        assert!(db.flush().unwrap().tables.is_empty());
        sender.send(Ok(())).unwrap();
        let flushed = db.flush().unwrap();
        assert_eq!(flushed.tables.len(), 1);
        assert_eq!(flushed.tables[0].0, name);
        assert!(flushed.tables[0].1.is_ok());
        assert_eq!(count(&db, &name), 10);
        let file_db = db.session.file_db_mut(&name).unwrap();
        assert!(FileDBMetadata::is_complete(file_db.file_path()));
    }

    #[test]
    fn failed_or_dropped_ingest_detaches_the_table() {
        let dir = TempDir::new().unwrap();
        let mut db = duckdb(&dir);
        let (failed, sender) = start(&mut db, &source(&dir, "one.csv"));
        sender.send(Err(anyhow!("broken file"))).unwrap();
        let (dropped, sender) = start(&mut db, &source(&dir, "two.csv"));
        drop(sender);
        let flushed = db.flush().unwrap();
        assert_eq!(flushed.tables.len(), 2);
        assert!(flushed.tables.iter().all(|(_, result)| result.is_err()));
        assert!(db.session.alias(&failed).is_none());
        assert!(db.session.alias(&dropped).is_none());
    }

    #[test]
    fn complete_cache_loads_without_ingesting() {
        let dir = TempDir::new().unwrap();
        let path = source(&dir, "sales.csv");
        let mut db = duckdb(&dir);
        let (name, sender) = start(&mut db, &path);
        sender.send(Ok(())).unwrap();
        db.flush().unwrap();
        db.detach(&name).unwrap();
        db.flush().unwrap();
        let (name, progress) = db.load(&path, None, None).unwrap();
        assert_eq!(name, "sales");
        assert!(progress.is_none());
        assert_eq!(count(&db, &name), 10);
    }

    #[test]
    fn unsupported_format_errors_and_releases_the_cache_file() {
        let dir = TempDir::new().unwrap();
        let path = source(&dir, "sales.xyz");
        let mut db = duckdb(&dir);
        assert!(db.load(&path, None, None).is_err());
        db.flush().unwrap();
        let error = db.load(&path, None, None).unwrap_err();
        assert!(error.to_string().contains("Unsupported"));
    }

    #[test]
    fn queries_wait_for_every_ingest() {
        let dir = TempDir::new().unwrap();
        let mut db = duckdb(&dir);
        let (name, sender) = start(&mut db, &source(&dir, "sales.csv"));
        db.load_query(&format!("-- name: total\nSELECT count(*) AS n FROM {name}"));
        assert!(db.flush().unwrap().queries.is_empty());
        sender.send(Ok(())).unwrap();
        let flushed = db.flush().unwrap();
        assert_eq!(flushed.tables.len(), 1);
        assert_eq!(flushed.queries.len(), 1);
        assert_eq!(flushed.queries[0].as_deref().unwrap(), "total");
        let n: i64 = db
            .session
            .session()
            .unwrap()
            .query_row("SELECT n FROM total", [], |r| r.get(0))
            .unwrap();
        assert_eq!(n, 10);
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
        let queries = db.flush().unwrap().queries;
        let names: Vec<_> = queries.into_iter().map(Result::unwrap).collect();
        assert_eq!(names, ["top_sales", "best"]);
        assert!(db.load_sql(&dir.path().join("missing.sql")).is_err());
    }

    #[test]
    fn bad_query_is_reported_once() {
        let dir = TempDir::new().unwrap();
        let mut db = duckdb(&dir);
        db.load_query("-- name: bad\nSELECT * FROM nowhere");
        let queries = db.flush().unwrap().queries;
        assert_eq!(queries.len(), 1);
        assert!(queries[0].is_err());
        assert!(db.flush().unwrap().queries.is_empty());
    }
}
