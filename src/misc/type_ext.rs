use std::fmt::Display;

use ratatui::layout::Constraint;
use unicode_width::UnicodeWidthChar;

use crate::{AppResult, handler::message::Message};

static OSC52_BUFFER: LazyLock<Mutex<Vec<String>>> = LazyLock::new(|| Mutex::new(Vec::new()));

pub fn flush_osc52_buffer() {
    let mut buffer = OSC52_BUFFER.lock().unwrap();
    for seq in buffer.drain(..) {
        print!("{}", seq);
    }
    std::io::stdout().flush().unwrap();
}

pub trait ToAscii {
    fn to_ascii(self) -> Option<u8>;
}

impl ToAscii for char {
    #[inline]
    fn to_ascii(self) -> Option<u8> {
        self.is_ascii().then_some(self as u8)
    }
}

pub trait HasSubsequence {
    fn has_subsequence(&self, other: &Self) -> bool;
}

pub trait UnwrapOrGracefulShutdown<T> {
    fn unwrap_or_graceful_shutdown(self) -> T;
}

pub trait VecExt<T> {
    fn take(&mut self, idx: usize) -> Option<T>;
}

pub trait UnwrapOrEnqueueError {
    fn unwrap_or_enqueue_error(&self);
}

pub trait ConstraintExt {
    fn value(&self) -> u16;
}

impl HasSubsequence for str {
    fn has_subsequence(&self, other: &Self) -> bool {
        let mut oitr = other.chars();
        let mut current = oitr.next();
        for chr in self.chars() {
            if let Some(cur) = current {
                if chr == cur {
                    current = oitr.next();
                }
            } else {
                break;
            }
        }
        current.is_none()
    }
}

pub struct SnakeCaseNameGen<'a> {
    base: &'a str,
    stage: u32,
}

impl<'a> SnakeCaseNameGen<'a> {
    pub fn with(base: &'a str) -> Self {
        Self { base, stage: 0 }
    }
}

impl Iterator for SnakeCaseNameGen<'_> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        self.stage += 1;
        match self.stage {
            1 => self.base.to_owned().into(),
            2.. => format!("{}_{}", self.base, self.stage).into(),
            _ => unimplemented!(),
        }
    }
}

pub trait SnakeCaseNameGenExt {
    fn snake_case_names(&self) -> SnakeCaseNameGen<'_>;
}

impl SnakeCaseNameGenExt for str {
    fn snake_case_names(&self) -> SnakeCaseNameGen<'_> {
        SnakeCaseNameGen::with(self)
    }
}

impl<T, E> UnwrapOrGracefulShutdown<T> for Result<T, E>
where
    E: Display,
{
    fn unwrap_or_graceful_shutdown(self) -> T {
        match self {
            Ok(val) => val,
            Err(err) => {
                eprintln!("Error: {err}");
                std::process::exit(1);
            }
        }
    }
}

pub fn human_readable_size(volume: u64) -> String {
    if volume < 1024 {
        format!("{volume} B")
    } else if volume < 1024 * 1024 {
        format!("{:.2} KB", volume as f64 / 1024.0)
    } else if volume < 1024 * 1024 * 1024 {
        format!("{:.2} MB", volume as f64 / (1024.0 * 1024.0))
    } else if volume < 1024 * 1024 * 1024 * 1024 {
        format!("{:.2} GB", volume as f64 / (1024.0 * 1024.0 * 1024.0))
    } else if volume < 1024 * 1024 * 1024 * 1024 * 1024 {
        format!(
            "{:.2} TB",
            volume as f64 / (1024.0 * 1024.0 * 1024.0 * 1024.0)
        )
    } else {
        format!(
            "{:.2} PB",
            volume as f64 / (1024.0 * 1024.0 * 1024.0 * 1024.0 * 1024.0)
        )
    }
}

pub trait FitToWidth {
    fn fit_to_width(&self, width: usize) -> &Self;
}

impl FitToWidth for str {
    fn fit_to_width(&self, width: usize) -> &Self {
        let end = self
            .char_indices()
            .map(|(i, c)| (i + c.len_utf8(), c.width().unwrap_or_default()))
            .scan(0, |s, (i, w)| {
                *s += w;
                (*s <= width).then_some(i)
            })
            .last()
            .unwrap_or_default();
        &self[..end]
    }
}

impl<T> VecExt<T> for Vec<T> {
    fn take(&mut self, idx: usize) -> Option<T> {
        (idx < self.len()).then(|| self.remove(idx))
    }
}

impl ConstraintExt for Constraint {
    fn value(&self) -> u16 {
        match self {
            Constraint::Min(val)
            | Constraint::Max(val)
            | Constraint::Length(val)
            | Constraint::Fill(val) => *val,
            Constraint::Percentage(_) | Constraint::Ratio(_, _) => 0,
        }
    }
}

impl UnwrapOrEnqueueError for AppResult<()> {
    fn unwrap_or_enqueue_error(&self) {
        match self {
            Ok(_) => (),
            Err(err) => Message::AppShowError(err.to_string()).enqueue(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_subsequence() {
        assert!("Chakra".has_subsequence("Ca"));
        assert!("abcdef".has_subsequence("abc"));
        assert!("abcdef".has_subsequence("ace"));
        assert!(!"abcdef".has_subsequence("aec"));
        assert!("".has_subsequence(""));
        assert!("abcdef".has_subsequence(""));
        assert!(!"".has_subsequence("abc"));
        assert!("a".has_subsequence("a"));
        assert!(!"a".has_subsequence("b"));
        assert!("abcdef".has_subsequence("a"));
        assert!("abcdef".has_subsequence("abcdef"));
        assert!(!"abcdef".has_subsequence("abcdefg"));
        assert!("abcdef".has_subsequence("ace"));
        assert!("abcdef".has_subsequence("ace"));
        assert!("abcdef".has_subsequence("acf"));
        assert!("abcdef".has_subsequence("abc"));
        assert!(!"abcdef".has_subsequence("ace"));
        assert!("abcdef".has_subsequence("ace"));
        assert!("abcdef".has_subsequence("f"));
        assert!("aaaaa".has_subsequence("aaa"));
        assert!("aaaaa".has_subsequence("aaaa"));
        assert!("a".has_subsequence("a"));
        assert!(!"abcdef".has_subsequence("g"));
        assert!("abcdef".has_subsequence("a"));
        assert!("abcdef".has_subsequence("f"));
        assert!("abcdef".has_subsequence("d"));
        assert!(!"".has_subsequence("a"));
        assert!("abc".has_subsequence("b"));
        assert!("abc".has_subsequence("b"));
        assert!("abc".has_subsequence("b"));
        assert!("abc".has_subsequence("c"));
        assert!(!"abc".has_subsequence("d"));
        assert!(!"abacabad".has_subsequence("aad"));
        assert!(!"abacabad".has_subsequence("aad"));
        assert!("x".has_subsequence("x"));
        assert!(!"x".has_subsequence("y"));
        assert!("aaaaa".has_subsequence("aaaaa"));
        assert!(!"aaaaa".has_subsequence("aaaaaa"));
        assert!("abcdef".has_subsequence("abcdef"));
        assert!(!"abcdef".has_subsequence("abcdefg"));
        assert!("a b c d e f".has_subsequence("abcdef"));
        assert!(!"abcdef".has_subsequence("a b c"));
        assert!("abacabadabacaba".has_subsequence("ababa"));
        assert!("abacabadabacaba".has_subsequence("ababa"));
        assert!("xyxyxyxyxy".has_subsequence("yyy"));
        assert!("xyxyxyxyxy".has_subsequence("yyy"));
        assert!(!"xyxyxyxyxy".has_subsequence("yyy"));
    }

    #[test]
    fn test_table_name_gen() {
        let mut name_gen = SnakeCaseNameGen::with("student");
        assert_eq!(name_gen.next().unwrap(), "student");
        assert_eq!(name_gen.next().unwrap(), "student_2");
        assert_eq!(name_gen.next().unwrap(), "student_3");
        assert_eq!(name_gen.next().unwrap(), "student_4");
    }

    #[test]
    fn test_fit_to_width_ascii() {
        // ASCII characters, each width 1
        assert_eq!("hello".fit_to_width(0), "");
        assert_eq!("hello".fit_to_width(1), "h");
        assert_eq!("hello".fit_to_width(2), "he");
        assert_eq!("hello".fit_to_width(5), "hello");
        assert_eq!("hello".fit_to_width(10), "hello");
    }

    #[test]
    fn test_fit_to_width_unicode_wide() {
        // Unicode wide characters (e.g., CJK, emoji)
        let s = "你好吗"; // Each Chinese char is width 2
        assert_eq!(s.fit_to_width(0), "");
        assert_eq!(s.fit_to_width(1), "");
        assert_eq!(s.fit_to_width(2), "你");
        assert_eq!(s.fit_to_width(3), "你");
        assert_eq!(s.fit_to_width(4), "你好");
        assert_eq!(s.fit_to_width(5), "你好");
        assert_eq!(s.fit_to_width(6), "你好吗");
        assert_eq!(s.fit_to_width(10), "你好吗");

        let emoji = "🙂🙃"; // Each emoji is width 2
        assert_eq!(emoji.fit_to_width(0), "");
        assert_eq!(emoji.fit_to_width(1), "");
        assert_eq!(emoji.fit_to_width(2), "🙂");
        assert_eq!(emoji.fit_to_width(3), "🙂");
        assert_eq!(emoji.fit_to_width(4), "🙂🙃");
    }

    #[test]
    fn test_fit_to_width_mixed() {
        let s = "a你b好c"; // a(1), 你(2), b(1), 好(2), c(1)
        assert_eq!(s.fit_to_width(0), "");
        assert_eq!(s.fit_to_width(1), "a");
        assert_eq!(s.fit_to_width(2), "a");
        assert_eq!(s.fit_to_width(3), "a你");
        assert_eq!(s.fit_to_width(4), "a你b");
        assert_eq!(s.fit_to_width(5), "a你b");
        assert_eq!(s.fit_to_width(6), "a你b好");
        assert_eq!(s.fit_to_width(7), "a你b好c");
        assert_eq!(s.fit_to_width(10), "a你b好c");
    }

    #[test]
    fn test_fit_to_width_empty() {
        assert_eq!("".fit_to_width(0), "");
        assert_eq!("".fit_to_width(10), "");
    }

    #[test]
    fn test_fit_to_width_combining() {
        let s = "a\u{0301}b";
        assert_eq!(s.fit_to_width(0), "");
        assert_eq!(s.fit_to_width(1), "a\u{0301}");
        assert_eq!(s.fit_to_width(2), "a\u{0301}b");
        assert_eq!(s.fit_to_width(3), "a\u{0301}b");
    }

    #[test]
    fn test_fit_to_width_edge_cases() {
        let s = "ab";
        assert_eq!(s.fit_to_width(1), "a");
        assert_eq!(s.fit_to_width(2), "ab");

        let s = "你";
        assert_eq!(s.fit_to_width(1), "");
        assert_eq!(s.fit_to_width(2), "你");
    }
}
