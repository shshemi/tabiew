use polars::{frame::DataFrame, prelude::AnyValue, series::Series};

use crate::utils::RoundRobinExt;

#[derive(Debug)]
pub struct TableValues {
    pool: ValuePool,
    height: usize,
    width: usize,
}

impl TableValues {
    pub fn from_dataframe(data_frame: &DataFrame) -> Self {
        let height = data_frame.height();
        let width = data_frame.width();
        let pool = data_frame.iter().map(Series::iter).round_robin().collect();
        Self {
            pool,
            width,
            height,
        }
    }

    pub fn get(&self, row: usize, col: usize) -> Option<&str> {
        self.pool.get(row * self.width + col)
    }

    pub fn get_row(&self, row: usize) -> Vec<&str> {
        (0..self.width)
            .map(|col| self.get(row, col).unwrap_or(""))
            .collect()
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn replace_dataframe(&mut self, data_frame: &DataFrame) {
        self.height = data_frame.height();
        self.width = data_frame.width();
        self.pool.clear();
        data_frame
            .iter()
            .map(Series::iter)
            .round_robin()
            .for_each(|value| self.pool.push(value));
    }
}

#[derive(Debug)]
struct ValuePool {
    pool: String,
    indices: Vec<usize>,
}

impl ValuePool {
    pub fn new(pool_capacity: usize, index_capacity: usize) -> Self {
        let pool = String::with_capacity(pool_capacity);
        let mut indices = Vec::with_capacity(index_capacity);
        indices.push(0);
        Self { pool, indices }
    }

    pub fn push(&mut self, value: AnyValue) {
        self.pool.push_str(&any_value_into_string(value));
        self.indices.push(self.pool.len());
    }

    pub fn get(&self, index: usize) -> Option<&str> {
        let start = *self.indices.get(index)?;
        let end = *self.indices.get(index + 1)?;
        self.pool.get(start..end)
    }

    pub fn clear(&mut self) {
        self.pool.clear();
        self.indices.clear();
        self.indices.push(0);
    }
}

impl<'a> FromIterator<AnyValue<'a>> for ValuePool {
    fn from_iter<T: IntoIterator<Item = AnyValue<'a>>>(iter: T) -> Self {
        let mut pool = ValuePool::new(16, 16);
        for value in iter {
            pool.push(value);
        }
        pool
    }
}

#[derive(Debug, Default, Clone)]
pub struct Scroll(usize);

impl From<Scroll> for usize {
    fn from(val: Scroll) -> Self {
        val.0
    }
}

impl From<Scroll> for u16 {
    fn from(val: Scroll) -> Self {
        val.0 as u16
    }
}

impl Scroll {
    pub fn up(&mut self) {
        self.0 = self.0.saturating_sub(1);
    }

    pub fn down(&mut self) {
        self.0 = self.0.saturating_add(1);
    }

    pub fn adjust(&mut self, lines: usize, space: usize) {
        self.0 = self.0.min(lines.saturating_sub(space))
    }

    pub fn to_u16(&self) -> u16 {
        self.0 as u16
    }
}

pub fn line_count(text: &str, width: usize) -> usize {
    let mut line_count = 1;
    let mut used_space = 0;
    for word_len in text.split(' ').map(str::len) {
        if word_len <= width {
            if used_space + word_len <= width {
                used_space += word_len + 1;
            } else {
                used_space = word_len + 1;
                line_count += 1;
            }
        } else {
            line_count += (word_len - width + used_space).div_ceil(width)
        }
    }
    line_count
}

pub fn data_frame_widths(df: &polars::frame::DataFrame) -> Vec<usize> {
    df.get_column_names()
        .into_iter()
        .zip(df.get_columns())
        .map(|(col, series)| col.len().max(series_width(series)))
        .collect::<Vec<_>>()
}

pub fn series_width(series: &Series) -> usize {
    series
        .iter()
        .map(|any_value| {
            any_value_into_string(any_value)
                .lines()
                .next()
                .map(str::len)
                .unwrap_or(0)
        })
        .max()
        .unwrap_or_default()
}

pub fn any_value_into_string(value: polars::datatypes::AnyValue) -> String {
    match value {
        AnyValue::Null => "".to_owned(),
        AnyValue::StringOwned(v) => v.to_string(),
        AnyValue::String(v) => v.to_string(),
        _ => value.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use polars::df;

    use super::*;

    #[test]
    fn test_value_pool() {
        let df = df! {
            "ints" => [1,2,3],
            "floats" => [1.0,2.0,3.0],
        }
        .unwrap();

        let vp = TableValues::from_dataframe(&df);
        println!("{:?}", vp.get(0, 0))
    }

    #[test]
    fn test_line_count_single_line() {
        let text = "Hello world";
        assert_eq!(line_count(text, 15), 1);
        assert_eq!(line_count(text, 11), 1);
        assert_eq!(line_count(text, 10), 2);
    }

    #[test]
    fn test_line_count_multiple_lines() {
        let text = "Hello world this is a test";
        assert_eq!(line_count(text, 15), 2);
        assert_eq!(line_count(text, 10), 3);
        assert_eq!(line_count(text, 5), 5);
    }

    #[test]
    fn test_line_count_exact_width() {
        let text = "Hello world";
        assert_eq!(line_count(text, 5), 2);
        assert_eq!(line_count(text, 6), 2);
        assert_eq!(line_count(text, 11), 1);
    }

    #[test]
    fn test_line_count_with_long_word() {
        let text = "supercalifragilisticexpialidocious";
        assert_eq!(line_count(text, 10), 4);
        assert_eq!(line_count(text, 20), 2);
        assert_eq!(line_count(text, 30), 2);
    }

    #[test]
    fn test_line_count_with_mixed_length_words() {
        let text = "a bb ccc dddd eeeee ffffff ggggggg";
        assert_eq!(line_count(text, 10), 4);
        assert_eq!(line_count(text, 5), 8);
        assert_eq!(line_count(text, 20), 2);
    }

    #[test]
    fn test_line_count_empty_string() {
        let text = "";
        assert_eq!(line_count(text, 10), 1);
    }
}
