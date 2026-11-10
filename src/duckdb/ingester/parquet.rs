use crate::{
    AppResult,
    duckdb::{
        helper::quote,
        ingester::{FileIngester, IngestProgress, progress::StepProgress},
        session::SessionConn,
    },
};
use duckdb::params;
use std::{path::PathBuf, sync::Arc};

pub struct ParquetIngester {
    path: PathBuf,
    progress: Arc<StepProgress>,
}

impl ParquetIngester {
    pub fn new(path: PathBuf) -> Self {
        Self {
            path,
            progress: StepProgress::new(1),
        }
    }
}

impl FileIngester for ParquetIngester {
    fn progress(&self) -> Arc<dyn IngestProgress> {
        self.progress.clone()
    }
    fn ingest(&mut self, conn: SessionConn, alias: &str) -> AppResult<()> {
        self.progress.start(&conn);
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
