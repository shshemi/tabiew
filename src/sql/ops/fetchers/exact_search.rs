use std::num::NonZeroUsize;

use crate::sql::{
    db::Query,
    ops::{
        fetchers::table_slice::{self, TableSlice},
        reply::{Receiver, channel},
    },
};

#[derive(Debug, Clone)]
pub struct Fetcher {
    query: Query,
    pattern: String,
    stream_size: Option<NonZeroUsize>,
}

impl Fetcher {
    pub fn new(query: Query, pattern: impl Into<String>) -> Self {
        Self {
            query,
            pattern: pattern.into(),
            stream_size: None,
        }
    }

    pub fn with_stream_size(mut self, values: NonZeroUsize) -> Self {
        self.stream_size = Some(values);
        self
    }

    pub fn run(&self) -> Receiver<TableSlice> {
        let query = match Query::new(format!(
            "SELECT * FROM ({}) WHERE contains(concat_ws(chr(0), *COLUMNS(*)), '{}')",
            self.query.as_str(),
            self.pattern.replace('\'', "''")
        )) {
            Ok(query) => query,
            Err(err) => {
                let (send, recv) = channel();
                send.send(Err(err));
                return recv;
            }
        };
        let fetcher = table_slice::Fetcher::new(query);
        match self.stream_size {
            Some(stream_size) => fetcher.with_stream_size(stream_size).run(),
            None => fetcher.run(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::{Duration, Instant};

    use super::*;
    use crate::{AppResult, sql::ops::Next};

    fn query(sql: &str) -> Query {
        Query::new(sql).unwrap()
    }

    fn collect(recv: Receiver<TableSlice>) -> AppResult<Vec<TableSlice>> {
        let deadline = Instant::now() + Duration::from_secs(30);
        let mut slices = Vec::new();
        loop {
            match recv.recv() {
                Next::Value(slice) => slices.push(slice?),
                Next::Done => return Ok(slices),
                Next::Empty if Instant::now() > deadline => panic!("timed out waiting for slices"),
                Next::Empty => std::thread::sleep(Duration::from_millis(5)),
            }
        }
    }

    fn search(sql: &str, pattern: &str) -> TableSlice {
        let mut slices = collect(Fetcher::new(query(sql), pattern).run()).unwrap();
        assert_eq!(slices.len(), 1);
        slices.remove(0)
    }

    const FRUITS: &str = "SELECT * FROM (VALUES \
        (1, 'apple', NULL), \
        (2, 'pear', 'xapx'), \
        (3, 'kiwi', NULL), \
        (42, NULL, 'Apple')) v(n, s, z)";

    #[test]
    fn matches_any_column() {
        let slice = search(FRUITS, "ap");
        assert_eq!(slice.get(0, 1), Some("apple"));
        assert_eq!(slice.get(1, 2), Some("xapx"));
        assert_eq!(slice.get(2, 0), None);
    }

    #[test]
    fn numbers_match_by_text() {
        let slice = search(FRUITS, "4");
        assert_eq!(slice.get(0, 0), Some("42"));
        assert_eq!(slice.get(1, 0), None);
    }

    #[test]
    fn matching_is_case_sensitive() {
        let slice = search(FRUITS, "Apple");
        assert_eq!(slice.get(0, 0), Some("42"));
        assert_eq!(slice.get(1, 0), None);
    }

    #[test]
    fn matches_do_not_span_columns() {
        assert!(search(FRUITS, "1apple").is_empty());
        assert!(search(FRUITS, "applexapx").is_empty());
    }

    #[test]
    fn empty_pattern_matches_every_row() {
        let slice = search(FRUITS, "");
        assert_eq!(slice.get(3, 0), Some("42"));
        assert_eq!(slice.get(4, 0), None);
    }

    #[test]
    fn no_matches_send_one_empty_slice() {
        assert!(search(FRUITS, "banana").is_empty());
    }

    #[test]
    fn quotes_in_the_pattern_are_literal() {
        let slice = search("SELECT * FROM (VALUES ('it''s'), ('its')) v(s)", "it's");
        assert_eq!(slice.get(0, 0), Some("it's"));
        assert_eq!(slice.get(1, 0), None);
    }

    #[test]
    fn missing_tables_fail() {
        assert!(collect(Fetcher::new(query("SELECT * FROM missing"), "x").run()).is_err());
    }

    #[test]
    fn results_are_streamed_in_slices() {
        let slices = collect(
            Fetcher::new(
                query("SELECT range AS n, range * 10 AS m FROM range(100)"),
                "5",
            )
            .with_stream_size(NonZeroUsize::new(4).unwrap())
            .run(),
        )
        .unwrap();
        assert_eq!(
            slices.iter().map(TableSlice::len).collect::<Vec<_>>(),
            [4, 4, 4, 4, 4, 4, 4, 4, 4, 2]
        );
        assert_eq!(slices[0].get(0, 0), Some("5"));
        assert_eq!(slices[0].get(1, 1), Some("150"));
    }
}
