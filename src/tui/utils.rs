use polars::{prelude::AnyValue, series::Series};

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
    use super::*;

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
