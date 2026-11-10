use crate::{
    AppResult,
    duckdb::CsvOptions,
    indexer::{Fields, FileIndexer},
};
use anyhow::anyhow;
use memchr::memchr2;
use std::io::{self, BufRead, Read};

pub struct CsvIndexer {
    delimiter: u8,
    quote: u8,
    header: bool,
}

impl CsvIndexer {
    pub fn new(options: &CsvOptions) -> AppResult<Self> {
        Ok(Self {
            delimiter: ascii(options.delimiter)?,
            quote: ascii(options.quote)?,
            header: options.header,
        })
    }
    fn fields(&self, record: &[u8]) -> Vec<String> {
        let mut fields = Vec::new();
        let mut field = Vec::new();
        let mut in_quotes = false;
        let mut i = 0;
        while i < record.len() {
            let b = record[i];
            if b == self.quote {
                if in_quotes && record.get(i + 1) == Some(&self.quote) {
                    field.push(self.quote);
                    i += 1;
                } else {
                    in_quotes = !in_quotes;
                }
            } else if b == self.delimiter && !in_quotes {
                fields.push(String::from_utf8_lossy(&field).into_owned());
                field.clear();
            } else {
                field.push(b);
            }
            i += 1;
        }
        fields.push(String::from_utf8_lossy(&field).into_owned());
        fields
    }
}

impl FileIndexer for CsvIndexer {
    fn scan(&self, file: &mut dyn Read, emit: &mut dyn FnMut(u64) -> bool) -> io::Result<()> {
        let mut buf = vec![0u8; 1 << 20];
        let mut base = 0u64;
        let mut start = 0u64;
        let mut in_quotes = false;
        let mut prev = b'\n';
        loop {
            let n = match file.read(&mut buf) {
                Ok(0) => break,
                Ok(n) => n,
                Err(e) if e.kind() == io::ErrorKind::Interrupted => continue,
                Err(e) => return Err(e),
            };
            let chunk = &buf[..n];
            let mut i = 0;
            while let Some(j) = memchr2(self.quote, b'\n', &chunk[i..]) {
                let k = i + j;
                if chunk[k] == self.quote {
                    in_quotes = !in_quotes;
                } else if !in_quotes {
                    let end = base + k as u64;
                    let before = if k > 0 { chunk[k - 1] } else { prev };
                    if end - start > u64::from(before == b'\r') && !emit(start) {
                        return Ok(());
                    }
                    start = end + 1;
                }
                i = k + 1;
            }
            prev = chunk[n - 1];
            base += n as u64;
        }
        if base > start && base - start > u64::from(prev == b'\r') {
            emit(start);
        }
        Ok(())
    }
    fn read_record(&self, reader: &mut dyn BufRead, record: &mut Vec<u8>) -> io::Result<bool> {
        loop {
            record.clear();
            let mut in_quotes = false;
            loop {
                let start = record.len();
                if reader.read_until(b'\n', record)? == 0 {
                    break;
                }
                let quotes = record[start..].iter().filter(|&&b| b == self.quote).count();
                in_quotes ^= quotes % 2 == 1;
                if !in_quotes {
                    break;
                }
            }
            if record.is_empty() {
                return Ok(false);
            }
            if record.last() == Some(&b'\n') {
                record.pop();
            }
            if record.last() == Some(&b'\r') {
                record.pop();
            }
            if !record.is_empty() {
                return Ok(true);
            }
        }
    }
    fn split(&self, record: &[u8]) -> Fields {
        Fields::Values(self.fields(record))
    }
    fn header(&self) -> bool {
        self.header
    }
}

fn ascii(c: char) -> AppResult<u8> {
    u8::try_from(c)
        .ok()
        .filter(u8::is_ascii)
        .ok_or_else(|| anyhow!("{c:?} is not an ASCII character"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{BufReader, Cursor};

    struct OneByte<'a>(&'a [u8]);

    impl Read for OneByte<'_> {
        fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
            let Some((&first, rest)) = self.0.split_first() else {
                return Ok(0);
            };
            buf[0] = first;
            self.0 = rest;
            Ok(1)
        }
    }

    fn indexer() -> CsvIndexer {
        CsvIndexer::new(&CsvOptions::default()).unwrap()
    }

    fn starts(data: &str) -> Vec<u64> {
        let mut whole = Vec::new();
        indexer()
            .scan(&mut Cursor::new(data), &mut |start| {
                whole.push(start);
                true
            })
            .unwrap();
        let mut bytes = Vec::new();
        indexer()
            .scan(&mut OneByte(data.as_bytes()), &mut |start| {
                bytes.push(start);
                true
            })
            .unwrap();
        assert_eq!(whole, bytes);
        whole
    }

    fn records(indexer: &CsvIndexer, data: &str) -> Vec<Vec<String>> {
        let mut reader = BufReader::new(Cursor::new(data));
        let mut record = Vec::new();
        let mut records = Vec::new();
        while indexer.read_record(&mut reader, &mut record).unwrap() {
            records.push(indexer.fields(&record));
        }
        records
    }

    #[test]
    fn scan_finds_each_record_start() {
        assert_eq!(starts("a,b\n1,2\n3,4\n"), [0, 4, 8]);
        assert_eq!(starts("a\nb"), [0, 2]);
        assert_eq!(starts(""), Vec::<u64>::new());
    }

    #[test]
    fn scan_ignores_newlines_inside_quotes() {
        assert_eq!(starts("a\n\"x\ny\"\nz\n"), [0, 2, 8]);
        assert_eq!(starts("\"a\"\"b\"\nc\n"), [0, 7]);
    }

    #[test]
    fn scan_skips_blank_lines_and_handles_crlf() {
        assert_eq!(starts("a\r\n\r\n\nb\r\n"), [0, 6]);
        assert_eq!(starts("a\r\nb\r"), [0, 3]);
    }

    #[test]
    fn scan_stops_when_emit_says_so() {
        let mut seen = Vec::new();
        indexer()
            .scan(&mut Cursor::new("a\nb\nc\n"), &mut |start| {
                seen.push(start);
                seen.len() < 2
            })
            .unwrap();
        assert_eq!(seen, [0, 2]);
    }

    #[test]
    fn records_match_the_scan() {
        let data = "id,note\r\n1,\"x\ny\"\n\n2,\"say \"\"hi\"\"\"\r\n3,last";
        assert_eq!(starts(data).len(), 4);
        assert_eq!(
            records(&indexer(), data),
            [
                vec!["id", "note"],
                vec!["1", "x\ny"],
                vec!["2", "say \"hi\""],
                vec!["3", "last"],
            ]
        );
    }

    #[test]
    fn split_keeps_empty_fields_and_quoted_delimiters() {
        let indexer = indexer();
        assert_eq!(indexer.fields(b"a,,b,"), ["a", "", "b", ""]);
        assert_eq!(
            indexer.fields(b"a,\"b,c\",\"d\"\"e\""),
            ["a", "b,c", "d\"e"]
        );
        assert_eq!(indexer.fields(b""), [""]);
    }

    #[test]
    fn custom_delimiter_and_quote() {
        let indexer = CsvIndexer::new(&CsvOptions {
            delimiter: ';',
            quote: '\'',
            ..CsvOptions::default()
        })
        .unwrap();
        assert_eq!(
            records(&indexer, "a;'b;\nc'\nd;\"e\"\n"),
            [vec!["a", "b;\nc"], vec!["d", "\"e\""]]
        );
    }

    #[test]
    fn non_ascii_options_are_rejected() {
        let options = CsvOptions {
            delimiter: '§',
            ..CsvOptions::default()
        };
        assert!(CsvIndexer::new(&options).is_err());
    }
}
