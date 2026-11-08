use crate::{
    AppResult,
    duckdb::{ingester::FileIngester, session::SessionConn},
};
use std::path::PathBuf;

pub struct MarkdownIngester {
    path: PathBuf,
}

impl MarkdownIngester {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }
}

impl FileIngester for MarkdownIngester {
    fn ingest(&mut self, _conn: SessionConn, _alias: &str) -> AppResult<()> {
        todo!("markdown {:?} into alias.data", self.path)
    }
}
