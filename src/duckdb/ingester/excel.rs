use crate::{
    AppResult,
    duckdb::{ingester::FileIngester, session::SessionConn},
};
use std::path::PathBuf;

pub struct ExcelIngester {
    path: PathBuf,
}

impl ExcelIngester {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }
}

impl FileIngester for ExcelIngester {
    fn ingest(&mut self, _conn: SessionConn, _alias: &str) -> AppResult<()> {
        todo!("excel {:?} into alias.data", self.path)
    }
}
