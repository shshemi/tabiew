use crate::misc::config::config;
use memchr::memchr;
use std::io::{self, BufRead, Read};

pub(super) fn scan(file: &mut dyn Read, emit: &mut dyn FnMut(u64) -> bool) -> io::Result<()> {
    let mut buf = vec![0u8; 1 << 20];
    let mut base = 0u64;
    let mut start = 0u64;
    let mut blank = true;
    loop {
        let n = match file.read(&mut buf) {
            Ok(0) => break,
            Ok(n) => n,
            Err(e) if e.kind() == io::ErrorKind::Interrupted => continue,
            Err(e) => return Err(e),
        };
        let chunk = &buf[..n];
        let mut i = 0;
        while let Some(j) = memchr(b'\n', &chunk[i..]) {
            let k = i + j;
            if (!blank || has_text(&chunk[i..k])) && !emit(start) {
                return Ok(());
            }
            start = base + k as u64 + 1;
            blank = true;
            i = k + 1;
        }
        blank &= !has_text(&chunk[i..]);
        base += n as u64;
    }
    if !blank {
        emit(start);
    }
    Ok(())
}

pub(super) fn read_limited(reader: &mut dyn BufRead, record: &mut Vec<u8>) -> io::Result<usize> {
    let limit = config().line_limit();
    let read = reader.take(limit as u64 + 1).read_until(b'\n', record)?;
    match record.len() > limit {
        true => Err(io::Error::other(format!(
            "A line is longer than process_ram_size / 4 ({}). Raise process_ram_size in the config",
            bytesize::ByteSize(limit as u64)
        ))),
        false => Ok(read),
    }
}

pub(super) fn read_line(reader: &mut dyn BufRead, record: &mut Vec<u8>) -> io::Result<bool> {
    loop {
        record.clear();
        if read_limited(reader, record)? == 0 {
            return Ok(false);
        }
        while matches!(record.last(), Some(b'\n' | b'\r')) {
            record.pop();
        }
        if has_text(record) {
            return Ok(true);
        }
    }
}

fn has_text(bytes: &[u8]) -> bool {
    bytes.iter().any(|b| !b.is_ascii_whitespace())
}
