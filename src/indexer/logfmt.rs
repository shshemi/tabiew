use crate::indexer::{Fields, FileIndexer, lines};
use logfmt_zerocopy::Logfmt;
use std::io::{self, BufRead, Read};

pub struct LogfmtIndexer;

impl FileIndexer for LogfmtIndexer {
    fn scan(&self, file: &mut dyn Read, emit: &mut dyn FnMut(u64) -> bool) -> io::Result<()> {
        lines::scan(file, emit)
    }
    fn read_record(&self, reader: &mut dyn BufRead, record: &mut Vec<u8>) -> io::Result<bool> {
        lines::read_line(reader, record)
    }
    fn split(&self, record: &[u8]) -> Fields {
        Fields::Pairs(
            String::from_utf8_lossy(record)
                .logfmt()
                .map(|(key, value)| (key.to_string(), value.to_string()))
                .collect(),
        )
    }
    fn header(&self) -> bool {
        false
    }
}
