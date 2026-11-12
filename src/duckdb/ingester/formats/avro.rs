use crate::{
    AppResult,
    duckdb::{
        ingester::{
            FileIngester, IngestProgress, formats::arrow::write_batches, progress::CountProgress,
        },
        session::SessionConn,
    },
};
use arrow_avro::reader::ReaderBuilder;
use std::{
    fs::File,
    io::{BufReader, Read},
    path::PathBuf,
    sync::Arc,
};

pub struct AvroIngester {
    path: PathBuf,
    progress: Arc<CountProgress>,
}

impl AvroIngester {
    pub fn new(path: PathBuf) -> Self {
        Self {
            path,
            progress: CountProgress::new(),
        }
    }
}

impl FileIngester for AvroIngester {
    fn progress(&self) -> Arc<dyn IngestProgress> {
        self.progress.clone()
    }
    fn ingest(&mut self, conn: SessionConn, alias: &str) -> AppResult<()> {
        let file = File::open(&self.path)?;
        self.progress.total(file.metadata()?.len());
        let counted = Counted {
            inner: file,
            progress: self.progress.clone(),
        };
        let reader = ReaderBuilder::new().build(BufReader::new(counted))?;
        let schema = reader.schema();
        write_batches(&conn, alias, schema, reader)
    }
}

struct Counted<R> {
    inner: R,
    progress: Arc<CountProgress>,
}

impl<R: Read> Read for Counted<R> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let read = self.inner.read(buf)?;
        self.progress.add(read as u64);
        Ok(read)
    }
}
