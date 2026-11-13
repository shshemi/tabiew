use crate::AppResult;
use anyhow::anyhow;
use std::{borrow::Cow, collections::VecDeque, ops::Range, sync::Arc};
use unicode_width::UnicodeWidthStr;

pub type Columns = Arc<[(String, String)]>;

pub trait RowSource {
    fn columns(&self) -> Columns;
    fn row_count(&self) -> usize;
    fn complete(&self) -> bool {
        true
    }
    fn view(&self) -> Option<&str> {
        None
    }
    fn fetch(&self, offset: usize, len: usize) -> AppResult<Vec<Vec<Datum>>>;
}

pub struct Limited<S> {
    pub source: S,
    pub limit: Option<usize>,
}

impl<S: RowSource> RowSource for Limited<S> {
    fn columns(&self) -> Columns {
        self.source.columns()
    }
    fn row_count(&self) -> usize {
        let rows = self.source.row_count();
        self.limit.map_or(rows, |limit| rows.min(limit))
    }
    fn complete(&self) -> bool {
        self.source.complete()
            || self
                .limit
                .is_some_and(|limit| self.source.row_count() >= limit)
    }
    fn view(&self) -> Option<&str> {
        self.source.view()
    }
    fn fetch(&self, offset: usize, len: usize) -> AppResult<Vec<Vec<Datum>>> {
        let len = self
            .limit
            .map_or(len, |limit| len.min(limit.saturating_sub(offset)));
        self.source.fetch(offset, len)
    }
}

pub struct Empty;

impl RowSource for Empty {
    fn columns(&self) -> Columns {
        Arc::from([])
    }
    fn row_count(&self) -> usize {
        0
    }
    fn fetch(&self, _: usize, _: usize) -> AppResult<Vec<Vec<Datum>>> {
        Ok(Vec::new())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ReaderConfig {
    pub segment_rows: usize,
    pub segments: usize,
    pub segment_bytes: usize,
}

pub type Datum = Option<String>;

type Segment = (usize, Vec<Vec<Datum>>);

pub trait Rows: Send {
    fn columns(&self) -> Columns;
    fn row_count(&self) -> usize;
    fn complete(&self) -> bool;
    fn view(&self) -> Option<String>;
    fn widths(&self) -> &[u16];
    fn reset(&mut self);
    fn fill(&mut self, range: Range<usize>) -> AppResult<()>;
    fn cached(&self, range: Range<usize>) -> Vec<&[Datum]>;
}

pub fn single_line(value: &str) -> Cow<'_, str> {
    let line = value.lines().next().unwrap_or_default();
    if line.contains('\t') {
        Cow::Owned(line.replace('\t', " "))
    } else {
        Cow::Borrowed(line)
    }
}

pub fn is_numeric(data_type: &str) -> bool {
    data_type.starts_with("DECIMAL")
        || [
            "TINYINT",
            "SMALLINT",
            "INTEGER",
            "BIGINT",
            "HUGEINT",
            "UTINYINT",
            "USMALLINT",
            "UINTEGER",
            "UBIGINT",
            "UHUGEINT",
            "FLOAT",
            "DOUBLE",
        ]
        .contains(&data_type)
}

pub fn is_integer(data_type: &str) -> bool {
    is_numeric(data_type)
        && !matches!(data_type, "FLOAT" | "DOUBLE")
        && !data_type.starts_with("DECIMAL")
}

fn width_of(value: &str) -> u16 {
    u16::try_from(single_line(value).width()).unwrap_or(u16::MAX)
}

pub struct Reader<S> {
    source: S,
    segment_rows: usize,
    segments: usize,
    segment_bytes: usize,
    cache: VecDeque<Segment>,
    complete: bool,
    widths: Vec<u16>,
}

impl<S: RowSource> Reader<S> {
    pub fn new(source: S, config: ReaderConfig) -> Self {
        Self {
            source,
            segment_rows: config.segment_rows.max(1),
            segments: config.segments.max(1),
            segment_bytes: config.segment_bytes,
            cache: VecDeque::new(),
            complete: false,
            widths: Vec::new(),
        }
    }
    pub fn source(&self) -> &S {
        &self.source
    }
    pub fn columns(&self) -> Columns {
        self.source.columns()
    }
    pub fn widths(&self) -> &[u16] {
        &self.widths
    }
    pub fn reset(&mut self) {
        self.cache.clear();
        self.widths.clear();
    }
    pub fn row_count(&self) -> usize {
        self.source.row_count()
    }
    pub fn rows(&mut self, range: Range<usize>) -> AppResult<impl Iterator<Item = &[Datum]>> {
        self.fill(range.clone())?;
        Ok(self.cached(range))
    }
    pub fn fill(&mut self, range: Range<usize>) -> AppResult<()> {
        if !self.complete {
            self.cache
                .retain(|(_, rows)| rows.len() == self.segment_rows);
        }
        self.complete = self.source.complete();
        let row_count = self.source.row_count();
        let start = range.start.min(row_count);
        let range = start..range.end.clamp(start, row_count);
        let first = range.start / self.segment_rows;
        let last = range.end.div_ceil(self.segment_rows).max(first);
        let missing = (first..last)
            .filter(|segment| !self.cache.iter().any(|(idx, _)| idx == segment))
            .collect::<Vec<_>>();
        for run in missing.chunk_by(|a, b| a + 1 == *b) {
            let mut rows = self
                .source
                .fetch(run[0] * self.segment_rows, run.len() * self.segment_rows)?
                .into_iter();
            for segment in run {
                let rows: Vec<Vec<Datum>> = rows.by_ref().take(self.segment_rows).collect();
                let bytes = rows
                    .iter()
                    .flatten()
                    .map(|value| value.as_ref().map_or(0, String::len) + size_of::<Datum>())
                    .sum::<usize>();
                if bytes > self.segment_bytes {
                    return Err(anyhow!(
                        "{} rows take {}, more than process_ram_size / segments ({}). Raise process_ram_size in the config",
                        rows.len(),
                        bytesize::ByteSize(bytes as u64),
                        bytesize::ByteSize(self.segment_bytes as u64)
                    ));
                }
                for row in &rows {
                    if self.widths.len() < row.len() {
                        self.widths.resize(row.len(), 0);
                    }
                    for (max, value) in self.widths.iter_mut().zip(row) {
                        *max = (*max).max(width_of(value.as_deref().unwrap_or_default()));
                    }
                }
                self.cache.push_back((*segment, rows));
            }
        }
        for segment in (first..last).rev() {
            if let Some(pos) = self.cache.iter().position(|(idx, _)| *idx == segment)
                && let Some(entry) = self.cache.remove(pos)
            {
                self.cache.push_back(entry);
            }
        }
        while self.cache.len() > self.segments.max(last - first) {
            self.cache.pop_front();
        }
        let columns = self.source.columns();
        let width = columns.len();
        if self.widths.len() < width {
            self.widths.resize(width, 0);
        }
        for (max, (name, _)) in self.widths.iter_mut().zip(columns.iter()) {
            *max = (*max).max(width_of(name));
        }
        for (_, rows) in self.cache.iter_mut().rev().take(last - first) {
            for row in rows.iter_mut().filter(|row| row.len() < width) {
                row.resize(width, None);
            }
        }
        Ok(())
    }
    pub fn cached(&self, range: Range<usize>) -> impl Iterator<Item = &[Datum]> {
        let row_count = self.source.row_count();
        let start = range.start.min(row_count);
        let range = start..range.end.clamp(start, row_count);
        let first = range.start / self.segment_rows;
        let last = range.end.div_ceil(self.segment_rows).max(first);
        let segment_rows = self.segment_rows;
        let cache = &self.cache;
        (first..last)
            .flat_map(move |segment| {
                cache
                    .iter()
                    .find(|(idx, _)| *idx == segment)
                    .into_iter()
                    .flat_map(|(_, rows)| rows.iter().map(Vec::as_slice))
            })
            .skip(range.start - first * segment_rows)
            .take(range.len())
    }
}

impl<S: RowSource + Send> Rows for Reader<S> {
    fn columns(&self) -> Columns {
        Reader::columns(self)
    }
    fn row_count(&self) -> usize {
        Reader::row_count(self)
    }
    fn complete(&self) -> bool {
        self.source.complete()
    }
    fn view(&self) -> Option<String> {
        self.source.view().map(str::to_string)
    }
    fn widths(&self) -> &[u16] {
        Reader::widths(self)
    }
    fn reset(&mut self) {
        Reader::reset(self)
    }
    fn fill(&mut self, range: Range<usize>) -> AppResult<()> {
        Reader::fill(self, range)
    }
    fn cached(&self, range: Range<usize>) -> Vec<&[Datum]> {
        Reader::cached(self, range).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;

    struct Numbers {
        rows: Cell<usize>,
        width: Cell<usize>,
        complete: Cell<bool>,
        fetches: Cell<usize>,
    }

    impl RowSource for &Numbers {
        fn columns(&self) -> Columns {
            (0..self.width.get())
                .map(|i| (format!("c{i}"), "VARCHAR".to_string()))
                .collect()
        }
        fn row_count(&self) -> usize {
            self.rows.get()
        }
        fn complete(&self) -> bool {
            self.complete.get()
        }
        fn fetch(&self, offset: usize, len: usize) -> AppResult<Vec<Vec<Datum>>> {
            self.fetches.set(self.fetches.get() + 1);
            Ok((offset..(offset + len).min(self.rows.get()))
                .map(|i| vec![Some(i.to_string())])
                .collect())
        }
    }

    fn numbers(rows: usize, complete: bool) -> Numbers {
        Numbers {
            rows: Cell::new(rows),
            width: Cell::new(1),
            complete: Cell::new(complete),
            fetches: Cell::new(0),
        }
    }

    fn read<S: RowSource>(reader: &mut Reader<S>, range: Range<usize>) -> Vec<Vec<String>> {
        reader
            .rows(range)
            .unwrap()
            .map(|row| {
                row.iter()
                    .map(|cell| cell.clone().unwrap_or_default())
                    .collect()
            })
            .collect()
    }

    fn cached<S>(reader: &Reader<S>) -> Vec<usize> {
        reader.cache.iter().map(|(idx, _)| *idx).collect()
    }

    const CONFIG: ReaderConfig = ReaderConfig {
        segment_rows: 100,
        segments: 3,
        segment_bytes: usize::MAX,
    };

    #[test]
    fn segments_are_kept_least_recently_used_first_out() {
        let source = numbers(10000, true);
        let mut reader = Reader::new(&source, CONFIG);
        assert_eq!(read(&mut reader, 0..2), [["0"], ["1"]]);
        assert_eq!(cached(&reader), [0]);
        let rows = read(&mut reader, 90..110);
        assert_eq!(
            (rows.len(), &rows[0][0], &rows[19][0]),
            (20, &"90".into(), &"109".into())
        );
        assert_eq!(cached(&reader), [1, 0]);
        read(&mut reader, 5000..5010);
        assert_eq!(cached(&reader), [1, 0, 50]);
        read(&mut reader, 0..10);
        assert_eq!(cached(&reader), [1, 50, 0]);
        read(&mut reader, 9990..20000);
        assert_eq!(cached(&reader), [50, 0, 99]);
        assert_eq!(read(&mut reader, 9995..9997)[1][0], "9996");
        assert_eq!(read(&mut reader, 0..250).len(), 250);
        assert_eq!(cached(&reader), [2, 1, 0]);
        assert_eq!(source.fetches.get(), 5);
        assert!(read(&mut reader, 20000..20010).is_empty());
    }

    #[test]
    fn short_segments_are_refetched_until_the_source_is_complete() {
        let source = numbers(150, false);
        let mut reader = Reader::new(&source, CONFIG);
        assert_eq!(read(&mut reader, 100..200).len(), 50);
        source.rows.set(180);
        assert_eq!(read(&mut reader, 100..200).len(), 80);
        assert_eq!(source.fetches.get(), 2);
        source.complete.set(true);
        read(&mut reader, 100..200);
        read(&mut reader, 100..200);
        assert_eq!(source.fetches.get(), 3);
    }

    #[test]
    fn segments_bigger_than_their_share_of_process_ram_size_fail() {
        let source = numbers(1000, true);
        let config = ReaderConfig {
            segment_bytes: 100 * (size_of::<Datum>() + 2),
            ..CONFIG
        };
        let mut reader = Reader::new(&source, config);
        assert!(reader.rows(0..10).is_ok());
        let error = reader.rows(200..210).err().unwrap().to_string();
        assert!(error.contains("process_ram_size"), "{error}");
    }

    #[test]
    fn rows_are_padded_when_columns_grow() {
        let source = numbers(10, true);
        let mut reader = Reader::new(&source, CONFIG);
        assert_eq!(read(&mut reader, 0..1), [["0"]]);
        source.width.set(3);
        assert_eq!(read(&mut reader, 0..1), [["0", "", ""]]);
        assert_eq!(source.fetches.get(), 1);
    }

    #[test]
    fn widths_grow_with_headers_and_fetched_segments_only() {
        let source = numbers(1000, true);
        let mut reader = Reader::new(&source, CONFIG);
        read(&mut reader, 0..1);
        assert_eq!(reader.widths(), [2]);
        read(&mut reader, 150..151);
        assert_eq!(reader.widths(), [3]);
        read(&mut reader, 0..1);
        assert_eq!(reader.widths(), [3]);
        assert_eq!(single_line("a\tb\nsecond"), "a b");
    }
}
