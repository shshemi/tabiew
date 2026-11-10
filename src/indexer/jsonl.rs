use crate::indexer::{Fields, FileIndexer, lines};
use serde_json::Value;
use std::io::{self, BufRead, Read};

pub struct JsonlIndexer;

impl FileIndexer for JsonlIndexer {
    fn scan(&self, file: &mut dyn Read, emit: &mut dyn FnMut(u64) -> bool) -> io::Result<()> {
        lines::scan(file, emit)
    }
    fn read_record(&self, reader: &mut dyn BufRead, record: &mut Vec<u8>) -> io::Result<bool> {
        lines::read_line(reader, record)
    }
    fn split(&self, record: &[u8]) -> Fields {
        let Ok(Value::Object(object)) = serde_json::from_slice(record) else {
            return Fields::Pairs(Vec::new());
        };
        Fields::Pairs(
            object
                .into_iter()
                .map(|(key, value)| {
                    let value = match value {
                        Value::String(text) => text,
                        Value::Null => String::new(),
                        value => value.to_string(),
                    };
                    (key, value)
                })
                .collect(),
        )
    }
    fn header(&self) -> bool {
        false
    }
}
