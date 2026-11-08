use std::collections::BTreeMap;

type Coordinate = (usize, usize);
type Range = (usize, usize);

pub struct ValuePool {
    idx: BTreeMap<Coordinate, Range>,
    spc: usize,
    buf: String,
}

impl ValuePool {
    pub fn get(&self, row: usize, col: usize) -> Option<&str> {
        let (start, end) = self.idx.get(&(row, col)).copied()?;
        self.buf.get(start..end)
    }

    pub fn push(&mut self, row: usize, col: usize, value: &str) -> bool {
        let start = self.buf.len();
        self.buf.push_str(value);
        let end = self.buf.len();
        if let Some((start, end)) = self.idx.insert((row, col), (start, end)) {
            self.spc += end - start;
            true
        } else {
            false
        }
    }

    pub fn len(&self) -> usize {
        self.idx.len()
    }

    pub fn mem_usage(&self) -> usize {
        self.idx.len() * size_of::<usize>() * 4 + self.buf.capacity()
    }

    pub fn iter<'a>(&'a self) -> Iter<'a> {
        Iter {
            iter: self.idx.iter(),
            buf: &self.buf,
        }
    }

    pub fn merge(&mut self, other: &ValuePool) {
        for ((row, col), value) in other.iter() {
            self.push(*row, *col, value);
        }
    }
}

pub struct Iter<'a> {
    iter: std::collections::btree_map::Iter<'a, Coordinate, Range>,
    buf: &'a String,
}

impl<'a> Iterator for Iter<'a> {
    type Item = (&'a (usize, usize), &'a str);

    fn next(&mut self) -> Option<Self::Item> {
        let (coor, (start, end)) = self.iter.next()?;
        let slice = self.buf.get(*start..*end)?;
        Some((coor, slice))
    }
}
