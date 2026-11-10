mod cache;
mod config;
mod file_db;
mod helper;
mod ingester;
#[cfg(debug_assertions)]
mod inspect;
pub mod reader;
mod session;
mod spool;

use crate::{AppResult, net::http, readers::DataSource};
use anyhow::anyhow;
use cache::CacheManager;
use config::Config;
use file_db::FileDB;
use helper::{SplitSql, random_name, split_sql};
use ingester::{Ingester, progress::Parts};
use session::Session;
use spool::Spool;
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};
use tokio::sync::oneshot::{Receiver, error::TryRecvError};
use url::Url;

pub use config::Config as DuckDBConfig;
pub use ingester::{CsvOptions, IngestOptions, IngestProgress};
#[cfg(debug_assertions)]
pub use inspect::inspect;

pub struct DuckDB {
    cache_manager: CacheManager,
    config: Config,
    session: Session,
    spool: Spool,
    ingesting: Vec<Ingesting>,
    pending_queries: Vec<(String, Option<String>)>,
    groups: Vec<Group>,
    part_of: Option<usize>,
}

struct Group {
    id: usize,
    source: PathBuf,
    limit: Option<usize>,
    parts: Vec<(String, Option<String>)>,
    waiting: usize,
    error: Option<String>,
    reported: bool,
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
    Ingesting(Arc<dyn IngestProgress>),
}

pub struct Flushed {
    pub tables: Vec<(PathBuf, AppResult<Vec<String>>)>,
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
        let spool = Spool::new(&config.cache_dir)?;
        Ok(Self {
            cache_manager,
            config,
            session,
            spool,
            ingesting: Vec::new(),
            pending_queries: Vec::new(),
            groups: Vec::new(),
            part_of: None,
        })
    }
    pub fn load(
        &mut self,
        path: &Path,
        table_name: Option<&str>,
        options: &IngestOptions,
        max_rows: Option<usize>,
    ) -> AppResult<Load> {
        let path = &path
            .canonicalize()
            .map_err(|e| anyhow!("Failed to open {}: {}", path.display(), e))?;
        let key = serde_json::to_value(options)?;
        let name = table_name.map(String::from).or_else(|| {
            path.file_stem()
                .map(|stem| stem.to_string_lossy().into_owned())
        });
        if let Some(file_db) = self.cache_manager.check_existing_cache_db(path, &key)? {
            return self.ready(file_db, name.as_deref(), max_rows);
        }
        let file_db = self.cache_manager.create_new_db(path, &key)?;
        self.ingest(path.clone(), None, file_db, name, options, max_rows)
    }
    pub fn load_stdin(
        &mut self,
        table_name: Option<&str>,
        options: &IngestOptions,
        max_rows: Option<usize>,
    ) -> AppResult<Load> {
        let path = self.spool.write("stdin", std::io::stdin().lock())?;
        let file_db = FileDB::create(
            &path.with_extension("duckdb"),
            &path,
            &serde_json::to_value(options)?,
        )?;
        let name = table_name.unwrap_or("Stdin").to_string();
        self.ingest(
            PathBuf::from("-"),
            Some(path),
            file_db,
            Some(name),
            options,
            max_rows,
        )
    }
    pub fn load_url(
        &mut self,
        url: &Url,
        table_name: Option<&str>,
        options: &IngestOptions,
        max_rows: Option<usize>,
    ) -> AppResult<Load> {
        let key = serde_json::to_value(options)?;
        let file_name = url
            .path_segments()
            .and_then(|mut segments| segments.next_back())
            .filter(|segment| !segment.is_empty())
            .unwrap_or("download")
            .to_string();
        let name = table_name.map(String::from).or_else(|| {
            Path::new(&file_name)
                .file_stem()
                .map(|stem| stem.to_string_lossy().into_owned())
        });
        let head = http::head(url)
            .header("Accept-Encoding", "identity")
            .call()
            .ok();
        let header = |name: &str| {
            head.as_ref()
                .and_then(|response| response.headers().get(name))
                .and_then(|value| value.to_str().ok())
                .map(String::from)
        };
        let validators = header("ETag").or_else(|| {
            header("Last-Modified").map(|modified| {
                format!(
                    "{modified} {}",
                    header("Content-Length").unwrap_or_default()
                )
            })
        });
        if let Some(validators) = &validators
            && let Some(file_db) =
                self.cache_manager
                    .check_url_db(url.as_str(), validators, &key)?
        {
            return self.ready(file_db, name.as_deref(), max_rows);
        }
        let response = http::get(url).call()?;
        let file = self
            .spool
            .write(&file_name, response.into_body().into_reader())?;
        let file_db = match &validators {
            Some(validators) => {
                let size = std::fs::metadata(&file)?.len();
                self.cache_manager
                    .create_url_db(url.as_str(), validators, size, &key)?
            }
            None => FileDB::create(&file.with_extension("duckdb"), &file, &key)?,
        };
        self.ingest(
            PathBuf::from(url.as_str()),
            Some(file),
            file_db,
            name,
            options,
            max_rows,
        )
    }
    pub fn load_parts(
        &mut self,
        sources: &[DataSource],
        options: &IngestOptions,
        max_rows: Option<usize>,
    ) -> AppResult<Load> {
        let id = self.groups.last().map(|group| group.id + 1).unwrap_or(0);
        self.groups.push(Group {
            id,
            source: sources
                .first()
                .map(|source| PathBuf::from(source.display_path().as_ref()))
                .unwrap_or_default(),
            limit: max_rows,
            parts: Vec::new(),
            waiting: 0,
            error: None,
            reported: true,
        });
        self.part_of = Some(id);
        let mut running = Vec::new();
        let mut failed = None;
        for source in sources {
            let loaded = match source {
                DataSource::Stdin => self.load_stdin(None, options, None),
                DataSource::File(path) => self.load(path, None, options, None),
                DataSource::Url(url) => self.load_url(url, None, options, None),
            };
            match loaded {
                Ok(Load::Ingesting(progress)) => running.push(progress),
                Ok(Load::Ready(_)) => {}
                Err(e) => {
                    failed = Some(e);
                    break;
                }
            }
        }
        self.part_of = None;
        let index = self.groups.len() - 1;
        if let Some(e) = failed {
            self.groups[index].error = Some(e.to_string());
            self.groups[index].reported = false;
            return Err(e);
        }
        if self.groups[index].waiting == 0 {
            let group = self.groups.remove(index);
            return self.complete_group(&group).map(Load::Ready);
        }
        Ok(Load::Ingesting(Arc::new(Parts {
            done: self.groups[index].parts.len() - running.len(),
            parts: running,
        })))
    }
    fn join(&mut self, alias: &str, name: Option<&str>, running: bool) -> bool {
        let Some(group) = self
            .part_of
            .and_then(|id| self.groups.iter_mut().find(|group| group.id == id))
        else {
            return false;
        };
        group
            .parts
            .push((alias.to_string(), name.map(String::from)));
        group.waiting += running as usize;
        true
    }
    fn complete_group(&mut self, group: &Group) -> AppResult<Vec<String>> {
        let names = self.union_views(group);
        for (alias, _) in &group.parts {
            self.session.unpin(alias);
        }
        names
    }
    fn union_views(&mut self, group: &Group) -> AppResult<Vec<String>> {
        if let Some(error) = &group.error {
            return Err(anyhow!("{error}"));
        }
        let mut sets: Vec<(Vec<(String, String)>, String, Vec<(String, String)>)> = Vec::new();
        for (alias, name) in &group.parts {
            for table in self.session.tables(alias)? {
                let schema = self.session.columns(alias, &table)?;
                let view = match name {
                    Some(name) if table == "data" => name.clone(),
                    _ => table.clone(),
                };
                let member = (alias.clone(), table);
                match sets.iter_mut().find(|(columns, _, _)| *columns == schema) {
                    Some((_, _, members)) => members.push(member),
                    None => sets.push((schema, view, vec![member])),
                }
            }
        }
        if sets.is_empty() {
            return Err(anyhow!("No tables were loaded"));
        }
        sets.iter()
            .map(|(_, view, members)| self.session.create_union(view, members, group.limit))
            .collect()
    }
    fn ready(
        &mut self,
        file_db: FileDB,
        name: Option<&str>,
        max_rows: Option<usize>,
    ) -> AppResult<Load> {
        let alias = self.session.attach(Some(file_db))?;
        if self.join(&alias, name, false) {
            return Ok(Load::Ready(Vec::new()));
        }
        let names = self.views(&alias, name, max_rows);
        self.session.unpin(&alias);
        names.map(Load::Ready)
    }
    fn ingest(
        &mut self,
        source: PathBuf,
        spooled: Option<PathBuf>,
        file_db: FileDB,
        name: Option<String>,
        options: &IngestOptions,
        max_rows: Option<usize>,
    ) -> AppResult<Load> {
        let alias = self.session.attach(Some(file_db))?;
        let file = spooled.clone().unwrap_or_else(|| source.clone());
        let ingested = Ingester::new(self.session.session()?, alias.clone(), file, options)
            .and_then(Ingester::ingest);
        match ingested {
            Ok((progress, result)) => {
                let group = self.part_of;
                self.join(&alias, name.as_deref(), true);
                self.ingesting.push(Ingesting {
                    source,
                    spooled,
                    group,
                    alias,
                    name,
                    limit: max_rows,
                    result,
                });
                Ok(Load::Ingesting(progress))
            }
            Err(e) => {
                self.session.unpin(&alias);
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
            flushed.tables.push((ingesting.source, names));
        }
        while let Some(index) = self.groups.iter().position(|group| group.waiting == 0) {
            let group = self.groups.remove(index);
            let names = self.complete_group(&group);
            if group.reported {
                flushed.tables.push((group.source, names));
            }
        }
        if self.ingesting.is_empty() {
            for (sql, fallback) in std::mem::take(&mut self.pending_queries) {
                flushed.queries.push(self.query(&sql, fallback));
            }
        }
        self.session.flush()?;
        Ok(flushed)
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
        assert!(db.flush().unwrap().tables.is_empty());
        sender.send(Ok(())).unwrap();
        let flushed = db.flush().unwrap();
        assert_eq!(flushed.tables.len(), 1);
        assert_eq!(flushed.tables[0].0, path);
        assert_eq!(flushed.tables[0].1.as_ref().unwrap(), &["sales"]);
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
        let flushed = db.flush().unwrap();
        assert_eq!(flushed.tables[0].1.as_ref().unwrap(), &["orders", "users"]);
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
        let flushed = db.flush().unwrap();
        assert_eq!(flushed.tables.len(), 3);
        assert!(flushed.tables.iter().all(|(_, result)| result.is_err()));
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
        db.flush().unwrap();
        db.detach("sales").unwrap();
        db.flush().unwrap();
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
        assert!(db.flush().unwrap().queries.is_empty());
        sender.send(Ok(())).unwrap();
        let flushed = db.flush().unwrap();
        assert_eq!(flushed.tables.len(), 1);
        assert_eq!(flushed.queries.len(), 1);
        assert_eq!(flushed.queries[0].as_deref().unwrap(), "total");
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
