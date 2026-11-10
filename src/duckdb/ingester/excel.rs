use crate::{
    AppResult,
    duckdb::{
        ingester::{FileIngester, IngestProgress, progress::CountProgress},
        session::SessionConn,
    },
};
use std::{path::PathBuf, sync::Arc};

pub struct ExcelIngester {
    path: PathBuf,
}

impl ExcelIngester {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }
}

impl FileIngester for ExcelIngester {
    fn progress(&self) -> Arc<dyn IngestProgress> {
        CountProgress::new()
    }
    fn ingest(&mut self, _conn: SessionConn, _alias: &str) -> AppResult<()> {
        todo!("excel {:?} into alias.data", self.path)
    }
}
