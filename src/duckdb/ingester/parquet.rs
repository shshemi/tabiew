use crate::{
    AppResult,
    duckdb::{ingester::FileIngester, session::SessionConn},
};
use std::path::PathBuf;

pub struct ParquetIngester {
    path: PathBuf,
}

impl ParquetIngester {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }
}

impl FileIngester for ParquetIngester {
    fn ingest(&mut self, _conn: SessionConn, _alias: &str) -> AppResult<()> {
        todo!("parquet {:?} into alias.data", self.path)
    }
}
