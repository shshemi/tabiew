use ratatui::style::Style;

#[derive(Debug, Default, Clone)]
pub struct Scroll {
    val: usize,
    max: usize,
}

impl Scroll {
    pub fn up(&mut self) {
        self.val = self.val.saturating_sub(1);
    }

    pub fn down(&mut self) {
        self.val = self.val.saturating_add(1).min(self.max);
    }

    pub fn adjust(&mut self, lines: usize, height: u16) {
        self.max = lines.saturating_sub(height.into());
        self.val = self.val.min(self.max);
    }

    pub fn val_u16(&self) -> u16 {
        self.val as u16
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

pub fn invert_style(mut style: Style) -> Style {
    std::mem::swap(&mut style.bg, &mut style.fg);
    style
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
