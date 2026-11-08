mod csv;

use crate::{
    AppResult,
    args::Format,
    duckdb::{CsvOptions, IngestOptions},
    misc::format::resolve_format,
};
use anyhow::anyhow;
use csv::CsvIndexer;
use std::{
    fs::File,
    io::{self, BufRead, BufReader, Read, Seek, SeekFrom},
    path::{Path, PathBuf},
    sync::{
        Arc, Mutex, RwLock,
        atomic::{AtomicBool, AtomicUsize, Ordering},
    },
};

pub const BLOCK: usize = 1024;

pub trait FileIndexer: Send + Sync {
    fn scan(&self, file: &mut dyn Read, emit: &mut dyn FnMut(u64) -> bool) -> io::Result<()>;
    fn read_record(&self, reader: &mut dyn BufRead, record: &mut Vec<u8>) -> io::Result<bool>;
    fn split(&self, record: &[u8]) -> Vec<String>;
    fn header(&self) -> bool;
}

#[derive(Default)]
struct Index {
    offsets: RwLock<Vec<u64>>,
    records: AtomicUsize,
    done: AtomicBool,
    error: Mutex<Option<String>>,
}

impl Index {
    fn push(&self, start: u64) {
        let records = self.records.load(Ordering::Relaxed);
        if records % BLOCK == 0 {
            self.offsets.write().unwrap().push(start);
        }
        self.records.store(records + 1, Ordering::Release);
    }
    fn nearest(&self, record: usize) -> (usize, u64) {
        let offsets = self.offsets.read().unwrap();
        let block = (record / BLOCK).min(offsets.len().saturating_sub(1));
        offsets
            .get(block)
            .map(|&offset| (block * BLOCK, offset))
            .unwrap_or((0, 0))
    }
}

pub struct Indexer {
    path: PathBuf,
    file_indexer: Arc<dyn FileIndexer>,
    index: Arc<Index>,
}

impl Indexer {
    pub fn new(source: PathBuf, options: &IngestOptions) -> AppResult<Self> {
        let file_indexer = Self::file_indexer(&source, options)?;
        let index = Arc::new(Index::default());
        let (task, scanner, path) = (index.clone(), file_indexer.clone(), source.clone());
        tokio::task::spawn_blocking(move || {
            let result = File::open(&path).and_then(|mut file| {
                scanner.scan(&mut file, &mut |start| {
                    task.push(start);
                    Arc::strong_count(&task) > 1
                })
            });
            if let Err(e) = result {
                *task.error.lock().unwrap() =
                    Some(format!("Failed to index {}: {}", path.display(), e));
            }
            task.done.store(true, Ordering::Release);
        });
        Ok(Self {
            path: source,
            file_indexer,
            index,
        })
    }
    fn file_indexer(source: &Path, options: &IngestOptions) -> AppResult<Arc<dyn FileIndexer>> {
        match resolve_format(options.format.as_ref(), source) {
            Some(Format::Csv | Format::Dsv) => Ok(Arc::new(CsvIndexer::new(&options.csv)?)),
            Some(Format::Tsv) => Ok(Arc::new(CsvIndexer::new(&CsvOptions {
                delimiter: '\t',
                ..options.csv.clone()
            })?)),
            _ => Err(anyhow!("No indexer for {}", source.display())),
        }
    }
    pub fn header(&self) -> AppResult<Option<Vec<String>>> {
        if !self.file_indexer.header() {
            return Ok(None);
        }
        Ok(self.records(0, 1)?.into_iter().next())
    }
    pub fn rows(&self, offset: usize, len: usize) -> AppResult<Vec<Vec<String>>> {
        self.records(offset + self.header_records(), len)
    }
    pub fn indexed_rows(&self) -> usize {
        self.index
            .records
            .load(Ordering::Acquire)
            .saturating_sub(self.header_records())
    }
    pub fn row_count(&self) -> Option<usize> {
        self.index
            .done
            .load(Ordering::Acquire)
            .then(|| self.indexed_rows())
    }
    pub fn error(&self) -> Option<String> {
        self.index.error.lock().unwrap().clone()
    }
    fn header_records(&self) -> usize {
        usize::from(self.file_indexer.header())
    }
    fn records(&self, first: usize, len: usize) -> AppResult<Vec<Vec<String>>> {
        let (mut record, offset) = self.index.nearest(first);
        let file = File::open(&self.path)
            .map_err(|e| anyhow!("Failed to open {}: {}", self.path.display(), e))?;
        let mut reader = BufReader::new(file);
        reader.seek(SeekFrom::Start(offset))?;
        let mut buf = Vec::new();
        while record < first {
            if !self.file_indexer.read_record(&mut reader, &mut buf)? {
                return Ok(Vec::new());
            }
            record += 1;
        }
        let mut rows = Vec::with_capacity(len);
        while rows.len() < len && self.file_indexer.read_record(&mut reader, &mut buf)? {
            rows.push(self.file_indexer.split(&buf));
        }
        Ok(rows)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{Duration, Instant};
    use tempfile::TempDir;

    fn file(dir: &TempDir, name: &str, data: &str) -> PathBuf {
        let path = dir.path().join(name);
        std::fs::write(&path, data).unwrap();
        path
    }

    fn numbered(rows: usize, row: impl Fn(usize) -> String) -> String {
        let mut data = String::from("id,name\n");
        for i in 0..rows {
            data.push_str(&row(i));
            data.push('\n');
        }
        data
    }

    fn wait(indexer: &Indexer) -> usize {
        let start = Instant::now();
        loop {
            if let Some(rows) = indexer.row_count() {
                return rows;
            }
            assert!(
                start.elapsed() < Duration::from_secs(10),
                "index never finished"
            );
            std::thread::sleep(Duration::from_millis(5));
        }
    }

    fn row(id: usize) -> Vec<String> {
        vec![id.to_string(), format!("n{id}")]
    }

    #[tokio::test]
    async fn rows_are_read_across_blocks() {
        let dir = TempDir::new().unwrap();
        let path = file(&dir, "sales.csv", &numbered(3000, |i| format!("{i},n{i}")));
        let indexer = Indexer::new(path, &IngestOptions::default()).unwrap();
        assert_eq!(wait(&indexer), 3000);
        assert_eq!(indexer.indexed_rows(), 3000);
        assert!(indexer.error().is_none());
        assert_eq!(indexer.header().unwrap().unwrap(), ["id", "name"]);
        assert_eq!(indexer.rows(0, 2).unwrap(), [row(0), row(1)]);
        assert_eq!(indexer.rows(1023, 2).unwrap(), [row(1023), row(1024)]);
        assert_eq!(indexer.rows(2048, 1).unwrap(), [row(2048)]);
        assert_eq!(indexer.rows(2998, 5).unwrap(), [row(2998), row(2999)]);
        assert!(indexer.rows(3000, 1).unwrap().is_empty());
    }

    #[tokio::test]
    async fn multi_line_records_count_as_one_row() {
        let dir = TempDir::new().unwrap();
        let path = file(
            &dir,
            "notes.csv",
            &numbered(2000, |i| format!("{i},\"a\nb{i}\"")),
        );
        let indexer = Indexer::new(path, &IngestOptions::default()).unwrap();
        assert_eq!(wait(&indexer), 2000);
        assert_eq!(
            indexer.rows(1500, 1).unwrap(),
            [vec!["1500".to_string(), "a\nb1500".to_string()]]
        );
    }

    #[tokio::test]
    async fn rows_past_the_indexed_part_read_from_the_nearest_block() {
        let dir = TempDir::new().unwrap();
        let path = file(&dir, "sales.csv", &numbered(3000, |i| format!("{i},n{i}")));
        let options = IngestOptions::default();
        let indexer = Indexer {
            file_indexer: Indexer::file_indexer(&path, &options).unwrap(),
            path,
            index: Arc::new(Index::default()),
        };
        assert_eq!(indexer.row_count(), None);
        assert_eq!(indexer.indexed_rows(), 0);
        assert_eq!(indexer.header().unwrap().unwrap(), ["id", "name"]);
        assert_eq!(indexer.rows(2500, 2).unwrap(), [row(2500), row(2501)]);
    }

    #[tokio::test]
    async fn no_header_and_tsv_and_unsupported_formats() {
        let dir = TempDir::new().unwrap();
        let mut options = IngestOptions::default();
        options.csv.header = false;
        let path = file(&dir, "plain.csv", "1,a\n2,b\n");
        let indexer = Indexer::new(path, &options).unwrap();
        assert_eq!(wait(&indexer), 2);
        assert!(indexer.header().unwrap().is_none());
        assert_eq!(indexer.rows(0, 1).unwrap(), [vec!["1", "a"]]);
        let path = file(&dir, "data.tsv", "id\tname\n1\tx,y\n");
        let indexer = Indexer::new(path, &IngestOptions::default()).unwrap();
        assert_eq!(wait(&indexer), 1);
        assert_eq!(indexer.rows(0, 1).unwrap(), [vec!["1", "x,y"]]);
        let path = file(&dir, "data.parquet", "");
        assert!(Indexer::new(path, &IngestOptions::default()).is_err());
    }

    #[tokio::test]
    async fn missing_file_reports_an_error() {
        let dir = TempDir::new().unwrap();
        let indexer = Indexer::new(dir.path().join("gone.csv"), &IngestOptions::default()).unwrap();
        assert_eq!(wait(&indexer), 0);
        assert!(indexer.error().unwrap().contains("gone.csv"));
        assert!(indexer.rows(0, 1).is_err());
    }
}
