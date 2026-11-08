use crate::{
    AppResult,
    duckdb::{helper::quote, ingester::FileIngester, session::SessionConn},
};
use duckdb::params;
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
    fn ingest(&mut self, conn: SessionConn, alias: &str) -> AppResult<()> {
        let format = if self.lines { "newline_delimited" } else { "auto" };
        conn.execute(
            &format!(
                "CREATE OR REPLACE TABLE {}.data AS SELECT * FROM read_json(?, format = ?)",
                quote(alias)
            ),
            params![self.path.to_string_lossy().into_owned(), format],
        )?;
        Ok(())
    }
}
