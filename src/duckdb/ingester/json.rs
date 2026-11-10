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

pub struct JsonIngester {
    path: PathBuf,
    lines: bool,
    progress: Arc<StepProgress>,
}

impl JsonIngester {
    pub fn new(path: PathBuf, lines: bool) -> Self {
        Self {
            path,
            lines,
            progress: StepProgress::new(1),
        }
    }
}

impl FileIngester for JsonIngester {
    fn progress(&self) -> Arc<dyn IngestProgress> {
        self.progress.clone()
    }
    fn ingest(&mut self, conn: SessionConn, alias: &str) -> AppResult<()> {
        self.progress.start(&conn);
        let format = if self.lines {
            "newline_delimited"
        } else {
            "auto"
        };
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
