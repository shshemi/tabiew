use crate::{
    duckdb::{FwfOptions, cut_text, sample_columns},
    indexer::{Fields, FileIndexer, lines},
};
use memchr::memchr_iter;
use std::{
    io::{self, BufRead, Cursor, Read},
    sync::OnceLock,
};

const SAMPLE: usize = 20_480;

pub struct FwfIndexer {
    options: FwfOptions,
    columns: OnceLock<Vec<(usize, Option<usize>)>>,
}

impl FwfIndexer {
    pub fn new(options: FwfOptions) -> Self {
        Self {
            options,
            columns: OnceLock::new(),
        }
    }
}

impl FileIndexer for FwfIndexer {
    fn scan(&self, file: &mut dyn Read, emit: &mut dyn FnMut(u64) -> bool) -> io::Result<()> {
        let mut head = Vec::new();
        let mut chunk = vec![0u8; 1 << 16];
        let mut newlines = 0;
        while newlines <= SAMPLE {
            let n = file.read(&mut chunk)?;
            if n == 0 {
                break;
            }
            newlines += memchr_iter(b'\n', &chunk[..n]).count();
            head.extend_from_slice(&chunk[..n]);
        }
        let text = String::from_utf8_lossy(&head);
        let mut lines = text
            .lines()
            .map(|line| line.trim_end_matches('\r'))
            .filter(|line| !line.trim().is_empty());
        let header = if self.options.header {
            lines.next()
        } else {
            None
        };
        let sample = lines.take(SAMPLE).map(String::from).collect::<Vec<_>>();
        let _ = self
            .columns
            .set(sample_columns(header, &sample, &self.options));
        lines::scan(&mut Cursor::new(head.as_slice()).chain(file), emit)
    }
    fn read_record(&self, reader: &mut dyn BufRead, record: &mut Vec<u8>) -> io::Result<bool> {
        lines::read_line(reader, record)
    }
    fn split(&self, record: &[u8]) -> Fields {
        let line = String::from_utf8_lossy(record);
        Fields::Values(match self.columns.get() {
            Some(columns) => columns
                .iter()
                .map(|&column| cut_text(&line, column))
                .collect(),
            None => vec![line.into_owned()],
        })
    }
    fn header(&self) -> bool {
        self.options.header
    }
}
