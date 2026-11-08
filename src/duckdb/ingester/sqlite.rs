use crate::{
    AppResult,
    duckdb::{ingester::FileIngester, session::SessionConn},
};
use std::path::PathBuf;

pub struct SqliteIngester {
    path: PathBuf,
}

impl SqliteIngester {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }
}

impl FileIngester for SqliteIngester {
    fn ingest(&mut self, _conn: SessionConn, _alias: &str) -> AppResult<()> {
        todo!("sqlite {:?} into alias.data", self.path)
    }
}
