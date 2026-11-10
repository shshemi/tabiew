use crate::{
    AppResult,
    duckdb::{
        ingester::{FileIngester, TextTable},
        session::SessionConn,
    },
};
use indexmap::IndexSet;
use logfmt_zerocopy::Logfmt;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

pub struct LogfmtIngester {
    path: PathBuf,
}

impl LogfmtIngester {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }
}

impl FileIngester for LogfmtIngester {
    fn ingest(&mut self, conn: SessionConn, alias: &str) -> AppResult<()> {
        let mut reader = BufReader::new(File::open(&self.path)?);
        let mut table = TextTable::new(&conn, alias, "data".to_string());
        let mut keys = IndexSet::<String>::new();
        let mut line = Vec::new();
        loop {
            line.clear();
            if reader.read_until(b'\n', &mut line)? == 0 {
                return table.finish();
            }
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
