use crate::{
    AppResult,
    duckdb::{
        ingester::{FileIngester, IngestProgress, progress::CountProgress},
        session::SessionConn,
        sql::quote,
    },
};
use arrow_ipc::reader::FileReader;
use duckdb::{
    Connection, arrow::record_batch::RecordBatch, params, vtab::arrow::ArrowBatchRegistration,
};
use std::{fs::File, path::PathBuf, sync::Arc};

pub struct ArrowIngester {
    path: PathBuf,
    progress: Arc<CountProgress>,
}

impl ArrowIngester {
    pub fn new(path: PathBuf) -> Self {
        Self {
            path,
            progress: CountProgress::new(),
        }
    }
}

impl FileIngester for ArrowIngester {
    fn progress(&self) -> Arc<dyn IngestProgress> {
        self.progress.clone()
    }
    fn ingest(&mut self, conn: SessionConn, alias: &str) -> AppResult<()> {
        let reader = FileReader::try_new_buffered(File::open(&self.path)?, None)?;
        self.progress.total(reader.num_batches() as u64);
        write_batches(&conn, alias, reader.inspect(|_| self.progress.add(1)))
    }
}

pub fn write_batches<E: std::error::Error + Send + Sync + 'static>(
    conn: &Connection,
    alias: &str,
    batches: impl Iterator<Item = Result<RecordBatch, E>>,
) -> AppResult<()> {
    let table = format!("{}.data", quote(alias));
    for (idx, batch) in batches.enumerate() {
        let batch = ArrowBatchRegistration::new(batch?);
        let sql = if idx == 0 {
            format!("CREATE OR REPLACE TABLE {table} AS SELECT * FROM arrow_batch(?)")
        } else {
            format!("INSERT INTO {table} SELECT * FROM arrow_batch(?)")
        };
        conn.execute(&sql, params![&batch])?;
    }
    Ok(())
}
