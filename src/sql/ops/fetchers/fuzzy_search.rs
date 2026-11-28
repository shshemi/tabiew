use std::num::NonZeroUsize;

use crate::sql::{
    db::Query,
    ops::{
        fetchers::table_slice::{self, TableSlice},
        reply::{Receiver, channel},
    },
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Similarity {
    Jaccard,
    Jaro,
    JaroWinkler,
    #[default]
    Levenshtein,
    DamerauLevenshtein,
}

#[derive(Debug, Clone)]
pub struct Fetcher {
    query: Query,
    pattern: String,
    similarity: Similarity,
    stream_size: Option<NonZeroUsize>,
}

impl Fetcher {
    pub fn new(query: Query, pattern: impl Into<String>) -> Self {
        Self {
            query,
            pattern: pattern.into(),
            similarity: Similarity::default(),
            stream_size: None,
        }
    }

    pub fn with_similarity(mut self, similarity: Similarity) -> Self {
        self.similarity = similarity;
        self
    }

    pub fn with_stream_size(mut self, values: NonZeroUsize) -> Self {
        self.stream_size = Some(values);
        self
    }

    pub fn run(&self) -> Receiver<TableSlice> {
        let sql = self.query.as_str();
        let pattern = self.pattern.to_lowercase().replace('\'', "''");
        let sql = match self.similarity {
            Similarity::Jaccard => format!(
                "SELECT * FROM ({sql}) \
                 ORDER BY list_max(list_transform(string_split(concat_ws(chr(0), *COLUMNS(*)), chr(0)), \
                 lambda x: CASE WHEN lower(x) = '' OR '{pattern}' = '' THEN 0 ELSE jaccard(lower(x), '{pattern}') END)) DESC"
            ),
            Similarity::Jaro => format!(
                "SELECT * FROM ({sql}) \
                 ORDER BY list_max(list_transform(string_split(concat_ws(chr(0), *COLUMNS(*)), chr(0)), \
                 lambda x: jaro_similarity(lower(x), '{pattern}'))) DESC"
            ),
            Similarity::JaroWinkler => format!(
                "SELECT * FROM ({sql}) \
                 ORDER BY list_max(list_transform(string_split(concat_ws(chr(0), *COLUMNS(*)), chr(0)), \
                 lambda x: jaro_winkler_similarity(lower(x), '{pattern}'))) DESC"
            ),
            Similarity::Levenshtein => format!(
                "SELECT * FROM ({sql}) \
                 ORDER BY list_min(list_transform(string_split(concat_ws(chr(0), *COLUMNS(*)), chr(0)), \
                 lambda x: levenshtein(lower(x), '{pattern}'))) ASC"
            ),
            Similarity::DamerauLevenshtein => format!(
                "SELECT * FROM ({sql}) \
                 ORDER BY list_min(list_transform(string_split(concat_ws(chr(0), *COLUMNS(*)), chr(0)), \
                 lambda x: damerau_levenshtein(lower(x), '{pattern}'))) ASC"
            ),
        };
        let query = match Query::new(sql) {
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

    fn ranked(fetcher: Fetcher, col: usize) -> Vec<String> {
        let slices = collect(fetcher.run()).unwrap();
        assert_eq!(slices.len(), 1);
        (0..)
            .map_while(|row| slices[0].get(row, col).map(str::to_owned))
            .collect()
    }

    const FRUITS: &str = "SELECT * FROM (VALUES \
        (1, 'Pineapple'), \
        (2, 'apple'), \
        (3, 'ape'), \
        (4, 'a_p_e'), \
        (5, 'pear')) v(n, s)";

    const SIMILARITIES: [Similarity; 5] = [
        Similarity::Jaccard,
        Similarity::Jaro,
        Similarity::JaroWinkler,
        Similarity::Levenshtein,
        Similarity::DamerauLevenshtein,
    ];

    #[test]
    fn default_similarity_is_levenshtein() {
        assert_eq!(Similarity::default(), Similarity::Levenshtein);
    }

    #[test]
    fn every_row_is_returned() {
        assert_eq!(ranked(Fetcher::new(query(FRUITS), "zzz"), 1).len(), 5);
    }

    #[test]
    fn closest_match_comes_first() {
        let rows = ranked(
            Fetcher::new(
                query("SELECT * FROM (VALUES ('Pineapple'), ('apple'), ('ape'), ('pear')) v(s)"),
                "ape",
            ),
            0,
        );
        assert_eq!(rows, ["ape", "apple", "pear", "Pineapple"]);
    }

    #[test]
    fn every_similarity_puts_the_exact_match_first() {
        for similarity in SIMILARITIES {
            let rows = ranked(
                Fetcher::new(query(FRUITS), "ape").with_similarity(similarity),
                1,
            );
            assert_eq!(rows.len(), 5, "{similarity:?}");
            assert_eq!(rows[0], "ape", "{similarity:?}");
        }
    }

    #[test]
    fn lowercase_patterns_ignore_case() {
        let rows = ranked(
            Fetcher::new(
                query("SELECT * FROM (VALUES ('apex'), ('Ape')) v(s)"),
                "ape",
            ),
            0,
        );
        assert_eq!(rows, ["Ape", "apex"]);
    }

    #[test]
    fn uppercase_patterns_ignore_case() {
        let rows = ranked(
            Fetcher::new(
                query("SELECT * FROM (VALUES ('apex'), ('ape')) v(s)"),
                "APE",
            ),
            0,
        );
        assert_eq!(rows, ["ape", "apex"]);
    }

    #[test]
    fn rows_are_ranked_by_their_best_column() {
        let slices = collect(
            Fetcher::new(
                query("SELECT * FROM (VALUES ('zzzz', 'apex'), ('ape', 'zzzz')) v(a, b)"),
                "ape",
            )
            .run(),
        )
        .unwrap();
        assert_eq!(slices[0].get(0, 0), Some("ape"));
        assert_eq!(slices[0].get(1, 1), Some("apex"));
    }

    #[test]
    fn quotes_in_the_pattern_are_literal() {
        let rows = ranked(
            Fetcher::new(
                query("SELECT * FROM (VALUES ('its'), ('it''s')) v(s)"),
                "it's",
            ),
            0,
        );
        assert_eq!(rows, ["it's", "its"]);
    }

    #[test]
    fn empty_cells_and_patterns_are_ranked() {
        for similarity in SIMILARITIES {
            for pattern in ["", "a"] {
                let result = collect(
                    Fetcher::new(
                        query("SELECT * FROM (VALUES (''), ('a'), (NULL)) v(s)"),
                        pattern,
                    )
                    .with_similarity(similarity)
                    .run(),
                );
                assert!(result.is_ok(), "{similarity:?} {pattern:?}: {result:?}");
            }
        }
    }

    #[test]
    fn missing_tables_fail() {
        assert!(collect(Fetcher::new(query("SELECT * FROM missing"), "x").run()).is_err());
    }

    #[test]
    fn results_are_streamed_in_slices() {
        let slices = collect(
            Fetcher::new(query("SELECT 'abc' AS s FROM range(10)"), "ac")
                .with_stream_size(NonZeroUsize::new(4).unwrap())
                .run(),
        )
        .unwrap();
        assert_eq!(
            slices.iter().map(TableSlice::len).collect::<Vec<_>>(),
            [4, 4, 2]
        );
    }
}
