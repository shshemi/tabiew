use crate::{
    AppResult,
    duckdb::{helper::quote, ingester::FileIngester, session::SessionConn},
};
use duckdb::params;
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
    fn ingest(&mut self, conn: SessionConn, alias: &str) -> AppResult<()> {
        conn.execute(
            &format!(
                "CREATE OR REPLACE TABLE {}.data AS SELECT * FROM read_parquet(?)",
                quote(alias)
            ),
            params![self.path.to_string_lossy().into_owned()],
        )?;
        Ok(())
    }
}
