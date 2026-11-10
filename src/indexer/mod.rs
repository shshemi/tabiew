mod csv;
mod fwf;
mod jsonl;
mod lines;
mod logfmt;

use crate::{
    AppResult,
    args::Format,
    duckdb::{CsvOptions, IngestOptions},
    misc::format::resolve_format,
    reader::{Datum, RowSource},
};
use anyhow::anyhow;
use csv::CsvIndexer;
use fwf::FwfIndexer;
use indexmap::IndexSet;
use jsonl::JsonlIndexer;
use logfmt::LogfmtIndexer;
use std::{
    fs::File,
    io::{self, BufRead, BufReader, Read, Seek, SeekFrom},
    path::{Path, PathBuf},
    sync::{
        Arc, Mutex, OnceLock, RwLock,
        atomic::{AtomicBool, AtomicUsize, Ordering},
    },
    time::Duration,
};

pub const BLOCK: usize = 1024;
const WAIT: Duration = Duration::from_millis(50);

#[derive(Debug, PartialEq)]
pub enum Fields {
    Values(Vec<String>),
    Pairs(Vec<(String, String)>),
}

pub trait FileIndexer: Send + Sync {
    fn scan(&self, file: &mut dyn Read, emit: &mut dyn FnMut(u64) -> bool) -> io::Result<()>;
    fn read_record(&self, reader: &mut dyn BufRead, record: &mut Vec<u8>) -> io::Result<bool>;
    fn split(&self, record: &[u8]) -> Fields;
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
        if records.is_multiple_of(BLOCK) {
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

struct Follow<'a> {
    file: File,
    complete: &'a AtomicBool,
    alive: &'a dyn Fn() -> bool,
}

impl Read for Follow<'_> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        loop {
            let done = self.complete.load(Ordering::Acquire);
            let read = self.file.read(buf)?;
            if read > 0 || done || !(self.alive)() {
                return Ok(read);
            }
            std::thread::sleep(WAIT);
        }
    }
}

fn open(path: &Path, complete: &AtomicBool, alive: &dyn Fn() -> bool) -> io::Result<File> {
    loop {
        let done = complete.load(Ordering::Acquire);
        match File::open(path) {
            Err(e) if e.kind() == io::ErrorKind::NotFound && !done && alive() => {
                std::thread::sleep(WAIT)
            }
            result => return result,
        }
    }
}

pub struct Indexer {
    path: PathBuf,
    file_indexer: Arc<dyn FileIndexer>,
    index: Arc<Index>,
    keys: RwLock<IndexSet<String>>,
    header: OnceLock<Vec<String>>,
}

impl Indexer {
    pub fn new(
        source: PathBuf,
        options: &IngestOptions,
        complete: Arc<AtomicBool>,
    ) -> AppResult<Self> {
        let file_indexer = Self::file_indexer(&source, options)?;
        let index = Arc::new(Index::default());
        let (task, scanner, path) = (index.clone(), file_indexer.clone(), source.clone());
        tokio::task::spawn_blocking(move || {
            let alive = || Arc::strong_count(&task) > 1;
            let result = open(&path, &complete, &alive).and_then(|file| {
                let mut reader = Follow {
                    file,
                    complete: &complete,
                    alive: &alive,
                };
                scanner.scan(&mut reader, &mut |start| {
                    task.push(start);
                    alive()
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
            keys: RwLock::default(),
            header: OnceLock::new(),
        })
    }
    fn file_indexer(source: &Path, options: &IngestOptions) -> AppResult<Arc<dyn FileIndexer>> {
        match resolve_format(options.format.as_ref(), source) {
            Format::Csv | Format::Dsv => Ok(Arc::new(CsvIndexer::new(&options.csv)?)),
            Format::Tsv => Ok(Arc::new(CsvIndexer::new(&CsvOptions {
                delimiter: '\t',
                ..options.csv.clone()
            })?)),
            Format::Jsonl => Ok(Arc::new(JsonlIndexer)),
            Format::Logfmt => Ok(Arc::new(LogfmtIndexer)),
            Format::Fwf => Ok(Arc::new(FwfIndexer::new(options.fwf.clone()))),
            _ => Err(anyhow!("No indexer for {}", source.display())),
        }
    }
    pub fn header(&self) -> AppResult<Option<Vec<String>>> {
        if self.file_indexer.header() {
            if let Some(header) = self.header.get() {
                return Ok(Some(header.clone()));
            }
            let header = self.records(0, 1)?.into_iter().next();
            if let Some(header) = &header {
                let _ = self.header.set(header.clone());
            }
            return Ok(header);
        }
        if self.keys.read().unwrap().is_empty() {
            self.records(0, 100)?;
        }
        let keys = self.keys.read().unwrap();
        Ok((!keys.is_empty()).then(|| keys.iter().cloned().collect()))
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
            rows.push(self.row(self.file_indexer.split(&buf)));
        }
        let width = self.keys.read().unwrap().len();
        for row in &mut rows {
            if row.len() < width {
                row.resize(width, String::new());
            }
        }
        Ok(rows)
    }
    fn row(&self, fields: Fields) -> Vec<String> {
        match fields {
            Fields::Values(values) => values,
            Fields::Pairs(pairs) => {
                let mut keys = self.keys.write().unwrap();
                let mut row = Vec::new();
                for (key, value) in pairs {
                    let index = keys.insert_full(key).0;
                    if index >= row.len() {
                        row.resize(index + 1, String::new());
                    }
                    row[index] = value;
                }
                row
            }
        }
    }
}

impl RowSource for Indexer {
    fn columns(&self) -> Vec<(String, String)> {
        self.header()
            .ok()
            .flatten()
            .unwrap_or_default()
            .into_iter()
            .map(|name| (name, "VARCHAR".to_string()))
            .collect()
    }
    fn row_count(&self) -> usize {
        self.indexed_rows()
    }
    fn complete(&self) -> bool {
        self.index.done.load(Ordering::Acquire)
    }
    fn fetch(&self, offset: usize, len: usize) -> AppResult<Vec<Vec<Datum>>> {
        Ok(self
            .rows(offset, len)?
            .into_iter()
            .map(|row| row.into_iter().map(Some).collect())
            .collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{io::Write, time::Instant};
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

    fn done() -> Arc<AtomicBool> {
        Arc::new(AtomicBool::new(true))
    }

    fn row(id: usize) -> Vec<String> {
        vec![id.to_string(), format!("n{id}")]
    }

    #[tokio::test]
    async fn rows_are_read_across_blocks() {
        let dir = TempDir::new().unwrap();
        let path = file(&dir, "sales.csv", &numbered(3000, |i| format!("{i},n{i}")));
        let indexer = Indexer::new(path, &IngestOptions::default(), done()).unwrap();
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
        let indexer = Indexer::new(path, &IngestOptions::default(), done()).unwrap();
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
            keys: RwLock::default(),
            header: OnceLock::new(),
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
        let indexer = Indexer::new(path, &options, done()).unwrap();
        assert_eq!(wait(&indexer), 2);
        assert!(indexer.header().unwrap().is_none());
        assert_eq!(indexer.rows(0, 1).unwrap(), [vec!["1", "a"]]);
        let path = file(&dir, "data.tsv", "id\tname\n1\tx,y\n");
        let indexer = Indexer::new(path, &IngestOptions::default(), done()).unwrap();
        assert_eq!(wait(&indexer), 1);
        assert_eq!(indexer.rows(0, 1).unwrap(), [vec!["1", "x,y"]]);
        let path = file(&dir, "data.parquet", "");
        assert!(Indexer::new(path, &IngestOptions::default(), done()).is_err());
    }

    #[tokio::test]
    async fn missing_file_reports_an_error() {
        let dir = TempDir::new().unwrap();
        let indexer = Indexer::new(
            dir.path().join("gone.csv"),
            &IngestOptions::default(),
            done(),
        )
        .unwrap();
        assert_eq!(wait(&indexer), 0);
        assert!(indexer.error().unwrap().contains("gone.csv"));
        assert!(indexer.rows(0, 1).is_err());
    }

    #[tokio::test]
    async fn growing_file_is_followed_until_complete() {
        let dir = TempDir::new().unwrap();
        let data = numbered(5000, |i| format!("{i},n{i}"));
        let (first, rest) = data.split_at(data.len() / 2);
        let path = file(&dir, "grow.csv", first);
        let complete = Arc::new(AtomicBool::new(false));
        let indexer =
            Indexer::new(path.clone(), &IngestOptions::default(), complete.clone()).unwrap();
        std::thread::sleep(Duration::from_millis(200));
        assert!(indexer.row_count().is_none());
        std::fs::OpenOptions::new()
            .append(true)
            .open(&path)
            .unwrap()
            .write_all(rest.as_bytes())
            .unwrap();
        complete.store(true, Ordering::Release);
        assert_eq!(wait(&indexer), 5000);
        assert_eq!(indexer.rows(4999, 1).unwrap(), [row(4999)]);
    }

    #[tokio::test]
    async fn file_created_later_is_waited_for() {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join("later.csv");
        let complete = Arc::new(AtomicBool::new(false));
        let indexer =
            Indexer::new(path.clone(), &IngestOptions::default(), complete.clone()).unwrap();
        std::thread::sleep(Duration::from_millis(100));
        std::fs::write(&path, numbered(10, |i| format!("{i},n{i}"))).unwrap();
        complete.store(true, Ordering::Release);
        assert_eq!(wait(&indexer), 10);
        assert!(indexer.error().is_none());
    }

    #[tokio::test]
    async fn jsonl_keys_keep_file_order_and_grow_as_rows_are_read() {
        let dir = TempDir::new().unwrap();
        let mut data = String::new();
        for i in 0..3000 {
            let extra = if i == 2500 { r#","late":[1,2]"# } else { "" };
            data.push_str(&format!(r#"{{"z":{i},"a":"n{i}","n":null{extra}}}"#));
            data.push('\n');
        }
        let path = file(&dir, "data.jsonl", &data);
        let indexer = Indexer::new(path, &IngestOptions::default(), done()).unwrap();
        assert_eq!(wait(&indexer), 3000);
        assert_eq!(indexer.header().unwrap().unwrap(), ["z", "a", "n"]);
        assert_eq!(indexer.rows(0, 1).unwrap(), [["0", "n0", ""]]);
        assert_eq!(
            indexer.rows(2499, 2).unwrap(),
            [
                vec!["2499", "n2499", "", ""],
                vec!["2500", "n2500", "", "[1,2]"]
            ]
        );
        assert_eq!(indexer.header().unwrap().unwrap(), ["z", "a", "n", "late"]);
    }

    #[tokio::test]
    async fn logfmt_rows_are_split_into_keys() {
        let dir = TempDir::new().unwrap();
        let path = file(
            &dir,
            "app.log",
            "ts=1 level=info\n\nts=2 msg=\"hi there\"\n",
        );
        let options = IngestOptions {
            format: Some(Format::Logfmt),
            ..IngestOptions::default()
        };
        let indexer = Indexer::new(path, &options, done()).unwrap();
        assert_eq!(wait(&indexer), 2);
        assert_eq!(
            indexer.rows(0, 2).unwrap(),
            [vec!["1", "info", ""], vec!["2", "", "hi there"]]
        );
        assert_eq!(indexer.header().unwrap().unwrap(), ["ts", "level", "msg"]);
    }

    #[tokio::test]
    async fn fwf_columns_come_from_the_sample_and_the_header() {
        let dir = TempDir::new().unwrap();
        let data = "id   name      city\n1    Alice     Paris\n\n2    Bob\n   3 Zoë       Köln and beyond\n";
        let path = file(&dir, "people.fwf", data);
        let indexer = Indexer::new(path.clone(), &IngestOptions::default(), done()).unwrap();
        assert_eq!(wait(&indexer), 3);
        assert_eq!(indexer.header().unwrap().unwrap(), ["id", "name", "city"]);
        assert_eq!(
            indexer.rows(0, 3).unwrap(),
            [
                vec!["1", "Alice", "Paris"],
                vec!["2", "Bob", ""],
                vec!["3", "Zoë", "Köln and beyond"]
            ]
        );
    }

    fn released(index: std::sync::Weak<Index>) -> bool {
        let start = Instant::now();
        while index.upgrade().is_some() {
            if start.elapsed() > Duration::from_secs(5) {
                return false;
            }
            std::thread::sleep(Duration::from_millis(10));
        }
        true
    }

    fn dropped_indexer_is_released(path: PathBuf) {
        let runtime = tokio::runtime::Runtime::new().unwrap();
        let released = {
            let _guard = runtime.enter();
            let complete = Arc::new(AtomicBool::new(false));
            let indexer = Indexer::new(path, &IngestOptions::default(), complete).unwrap();
            std::thread::sleep(Duration::from_millis(100));
            let index = Arc::downgrade(&indexer.index);
            drop(indexer);
            released(index)
        };
        runtime.shutdown_timeout(Duration::from_millis(100));
        assert!(
            released,
            "scan thread kept waiting after the indexer was dropped"
        );
    }

    #[test]
    fn dropping_the_indexer_stops_waiting_for_a_missing_file() {
        let dir = TempDir::new().unwrap();
        dropped_indexer_is_released(dir.path().join("never.csv"));
    }

    #[test]
    fn dropping_the_indexer_stops_following_a_stalled_file() {
        let dir = TempDir::new().unwrap();
        let path = file(&dir, "stalled.csv", &numbered(10, |i| format!("{i},n{i}")));
        dropped_indexer_is_released(path);
    }

    #[tokio::test]
    async fn failed_download_finishes_with_what_arrived() {
        let dir = TempDir::new().unwrap();
        let data = numbered(3000, |i| format!("{i},n{i}"));
        let cut = data.find("2000,n2000").unwrap() + "2000,n2".len();
        let path = file(&dir, "cut.csv", &data[..cut]);
        let complete = Arc::new(AtomicBool::new(false));
        let indexer = Indexer::new(path, &IngestOptions::default(), complete.clone()).unwrap();
        std::thread::sleep(Duration::from_millis(100));
        assert!(indexer.row_count().is_none());
        complete.store(true, Ordering::Release);
        assert_eq!(wait(&indexer), 2001);
        assert!(indexer.error().is_none());
        assert_eq!(
            indexer.rows(1999, 2).unwrap(),
            [row(1999), vec!["2000".to_string(), "n2".to_string()]]
        );
    }
}
