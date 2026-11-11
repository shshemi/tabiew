use crate::{
    AppResult,
    duckdb::{
        ingester::{FileIngester, IngestProgress, TextTable, line_limit, progress::CountProgress},
        session::SessionConn,
    },
};
use anyhow::anyhow;
use bytesize::ByteSize;
use indexmap::IndexSet;
use logfmt_zerocopy::Logfmt;
use std::{
    fs::File,
    io::{BufRead, BufReader, Read},
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
        let limit = line_limit();
        for number in 1.. {
            line.clear();
            let read = (&mut reader)
                .take(limit as u64 + 1)
                .read_until(b'\n', &mut line)?;
            if read == 0 {
                break;
            }
            if read > limit {
                return Err(anyhow!(
                    "Line {number} is longer than process_ram_size / 4 ({}). Raise process_ram_size in the config",
                    ByteSize(limit as u64)
                ));
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
        table.finish()
    }
}
