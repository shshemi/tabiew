use std::collections::BTreeMap;

use crate::sql::db::ValueRef;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Coordinate {
    row: usize,
    col: usize,
}

#[derive(Debug)]
struct Range {
    single_start: usize,
    single_end: usize,
    multi_start: usize,
    multi_end: usize,
}

impl Range {
    fn offset(self, offset: usize) -> Self {
        Range {
            single_start: offset + self.single_start,
            single_end: offset + self.single_end,
            multi_start: offset + self.multi_start,
            multi_end: offset + self.multi_end,
        }
    }

    fn len(&self) -> usize {
        self.buffer_end().saturating_sub(self.buffer_start())
    }

    fn buffer_start(&self) -> usize {
        self.single_start.min(self.multi_start)
    }

    fn buffer_end(&self) -> usize {
        self.single_end.max(self.multi_end)
    }
}

#[derive(Debug, Default)]
pub struct ValuePool {
    idx: BTreeMap<Coordinate, Range>,
    buf: String,
}

impl ValuePool {
    pub fn get_singleline(&self, row: usize, col: usize) -> Option<&str> {
        self.idx
            .get(&Coordinate { row, col })
            .and_then(|r| self.buf.get(r.single_start..r.single_end))
    }

    pub fn get_multiline(&self, row: usize, col: usize) -> Option<&str> {
        self.idx
            .get(&Coordinate { row, col })
            .and_then(|r| self.buf.get(r.multi_start..r.multi_end))
    }

    pub fn push_value(&mut self, row: usize, col: usize, value: ValueRef) -> bool {
        let offset = self.buf.len();
        let rng = value_to_insertable(value, &mut self.buf);
        if let Some(_) = self.idx.insert(Coordinate { row, col }, rng.offset(offset)) {
            true
        } else {
            false
        }
    }

    pub fn value_count(&self) -> usize {
        self.idx.len()
    }

    pub fn mem_usage(&self) -> usize {
        self.idx.len() * size_of::<usize>() * 4 + self.buf.capacity()
    }

    pub fn merge(&mut self, other: ValuePool) {
        let offset = self.buf.len();
        for (crd, rng) in other.idx {
            self.idx.insert(crd, rng.offset(offset));
        }
        self.buf.push_str(&other.buf);
    }
}

fn value_to_insertable(value: ValueRef, buf: &mut String) -> Range {
    todo!()
}
