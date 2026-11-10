use crate::{
    AppResult,
    duckdb::{
        ingester::{FileIngester, IngestProgress, progress::CountProgress},
        session::SessionConn,
    },
};
use std::{path::PathBuf, sync::Arc};

pub struct ArrowIngester {
    path: PathBuf,
}

impl ArrowIngester {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }
}

impl FileIngester for ArrowIngester {
    fn progress(&self) -> Arc<dyn IngestProgress> {
        CountProgress::new()
    }
    fn ingest(&mut self, _conn: SessionConn, _alias: &str) -> AppResult<()> {
        todo!("arrow {:?} into alias.data", self.path)
    }
}
