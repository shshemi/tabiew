use crate::{
    AppResult,
    duckdb::{ingester::FileIngester, session::SessionConn},
};
use std::path::PathBuf;

pub struct CsvIngester {
    path: PathBuf,
    delimiter: Option<char>,
}

impl CsvIngester {
    pub fn new(path: PathBuf, delimiter: Option<char>) -> Self {
        Self { path, delimiter }
    }
}

impl FileIngester for CsvIngester {
    fn ingest(&mut self, _conn: SessionConn, _alias: &str) -> AppResult<()> {
        todo!(
            "read_csv {:?} delim {:?} into alias.data",
            self.path,
            self.delimiter
        )
    }
}
