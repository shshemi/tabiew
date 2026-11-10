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

pub(super) fn read_line(reader: &mut dyn BufRead, record: &mut Vec<u8>) -> io::Result<bool> {
    loop {
        record.clear();
        if reader.read_until(b'\n', record)? == 0 {
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    fn starts(data: &str) -> Vec<u64> {
        let mut starts = Vec::new();
        scan(&mut data.as_bytes(), &mut |start| {
            starts.push(start);
            true
        })
        .unwrap();
        starts
    }

    #[test]
    fn scan_skips_blank_lines_and_keeps_an_unterminated_last_line() {
        assert_eq!(starts("a\n\n  \nb\r\nc"), [0, 6, 9]);
        assert_eq!(starts("\n\n"), [] as [u64; 0]);
    }

    #[test]
    fn read_line_matches_the_scan() {
        let data = "a\n\n  \nb\r\nc";
        let mut reader = Cursor::new(data.as_bytes());
        let mut record = Vec::new();
        let mut lines = Vec::new();
        while read_line(&mut reader, &mut record).unwrap() {
            lines.push(String::from_utf8(record.clone()).unwrap());
        }
        assert_eq!(lines, ["a", "b", "c"]);
    }
}
