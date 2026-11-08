use crate::{
    AppResult,
    duckdb::{ingester::FileIngester, session::SessionConn},
};
use std::path::PathBuf;

pub struct JsonIngester {
    path: PathBuf,
    lines: bool,
}

impl JsonIngester {
    pub fn new(path: PathBuf, lines: bool) -> Self {
        Self { path, lines }
    }
}

impl FileIngester for JsonIngester {
    fn ingest(&mut self, _conn: SessionConn, _alias: &str) -> AppResult<()> {
        todo!(
            "read_json {:?} lines {} into alias.data",
            self.path,
            self.lines
        )
    }
}
