use crate::{
    AppResult,
    duckdb::{
        ingester::{FileIngester, IngestProgress, TextTable, progress::CountProgress},
        session::SessionConn,
    },
};
use indexmap::IndexSet;
use logfmt_zerocopy::Logfmt;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
    sync::Arc,
};

pub struct LogfmtIngester {
    path: PathBuf,
    types: Vec<String>,
    progress: Arc<CountProgress>,
}

impl LogfmtIngester {
    pub fn new(path: PathBuf, types: Vec<String>) -> Self {
        Self {
            path,
            types,
            progress: CountProgress::new(),
        }
    }
}

impl FileIngester for LogfmtIngester {
    fn progress(&self) -> Arc<dyn IngestProgress> {
        self.progress.clone()
    }
    fn ingest(&mut self, conn: SessionConn, alias: &str) -> AppResult<()> {
        let file = File::open(&self.path)?;
        self.progress.total(file.metadata()?.len());
        let mut reader = BufReader::new(file);
        let mut table = TextTable::new(&conn, alias, "data".to_string(), &self.types);
        let mut keys = IndexSet::<String>::new();
        let mut line = Vec::new();
        loop {
            line.clear();
            let read = reader.read_until(b'\n', &mut line)?;
            if read == 0 {
                return table.finish();
            }
            self.progress.add(read as u64);
            let text = String::from_utf8_lossy(&line);
            let text = text.trim_end_matches(['\n', '\r']);
            if text.trim().is_empty() {
                continue;
            }
            let row = text
                .logfmt()
                .map(|(key, value)| {
                    let index = keys.get_index_of(key).unwrap_or_else(|| {
                        keys.insert(key.to_string());
                        table.column(key)
                    });
                    (index, value.to_string())
                })
                .collect();
            table.push(row)?;
        }
    }
}
