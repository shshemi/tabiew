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
        })
    }
    pub fn load(
        &mut self,
        path: &Path,
        table_name: Option<&str>,
        format: Option<Format>,
    ) -> AppResult<(String, Option<ProgressHandle>)> {
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
    pub fn detach(&mut self, name: &str) -> AppResult<()> {
        self.session.detach(name)
    }
    pub fn flush(&mut self) -> AppResult<Vec<(String, AppResult<()>)>> {
        let mut finished = Vec::new();
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
            finished.push((name, result));
        }
        self.session.flush()?;
        Ok(finished)
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
    fn finish(&mut self, name: &str) -> AppResult<()> {
        self.session
            .file_db_mut(name)
            .ok_or_else(|| anyhow!("Unknown table {name}"))?
            .mark_complete()?;
        self.session.create_view(name)
    }
}
