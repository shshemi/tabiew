use std::io::{Write, stdout};

use base64::Engine;

use crate::AppResult;

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
    fn has_subsequence(&self, other: &Self, max_space: usize) -> bool;
}

impl HasSubsequence for str {
    fn has_subsequence(&self, other: &Self, max_space: usize) -> bool {
        let mut oitr = other.chars();
        let mut current = oitr.next();
        let mut space = 0;
        let mut first_match = false;
        for chr in self.chars() {
            if let Some(cur) = current {
                if chr == cur {
                    current = oitr.next();
                    space = 0;
                    first_match = true;
                } else if first_match {
                    space += 1;
                }
                if space > max_space {
                    break;
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
    fn snake_case_names(&self) -> SnakeCaseNameGen;
}

impl SnakeCaseNameGenExt for str {
    fn snake_case_names(&self) -> SnakeCaseNameGen {
        SnakeCaseNameGen::with(self)
    }
}

pub trait CopyToClipboardOsc52 {
    fn copy_to_clipboard_via_osc52(&self) -> AppResult<()>;
}

impl<T> CopyToClipboardOsc52 for T
where
    T: AsRef<[u8]>,
{
    fn copy_to_clipboard_via_osc52(&self) -> AppResult<()> {
        let encoded = base64::engine::general_purpose::STANDARD.encode(self);
        let sequence = format!("\x1b]52;c;{encoded}\x07");
        let mut out = stdout();
        out.write_all(sequence.as_bytes())?;
        out.flush()?;
        Ok(())
    }
}

pub fn human_readable_size(volume: usize) -> String {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_subsequence() {
        assert!("abcdef".has_subsequence("abc", 10));
        assert!("abcdef".has_subsequence("ace", 2));
        assert!(!"abcdef".has_subsequence("aec", 2));
        assert!("".has_subsequence("", 10));
        assert!("abcdef".has_subsequence("", 10));
        assert!(!"".has_subsequence("abc", 10));
        assert!("a".has_subsequence("a", 0));
        assert!(!"a".has_subsequence("b", 0));
        assert!("abcdef".has_subsequence("a", 10));
        assert!("abcdef".has_subsequence("abcdef", 10));
        assert!(!"abcdef".has_subsequence("abcdefg", 10));
        assert!("abcdef".has_subsequence("ace", 2));
        assert!("abcdef".has_subsequence("ace", 1));
        assert!("abcdef".has_subsequence("acf", 3));
        assert!("abcdef".has_subsequence("abc", 0));
        assert!(!"abcdef".has_subsequence("ace", 0));
        assert!("abcdef".has_subsequence("ace", 10));
        assert!("abcdef".has_subsequence("f", 100));
        assert!("aaaaa".has_subsequence("aaa", 1));
        assert!("aaaaa".has_subsequence("aaaa", 0));
        assert!("a".has_subsequence("a", 0));
        assert!(!"abcdef".has_subsequence("g", 10));
        assert!("abcdef".has_subsequence("a", 0));
        assert!("abcdef".has_subsequence("f", 0));
        assert!("abcdef".has_subsequence("d", 2));
        assert!(!"".has_subsequence("a", 0));
        assert!("abc".has_subsequence("b", 0));
        assert!("abc".has_subsequence("b", 1));
        assert!("abc".has_subsequence("b", 2));
        assert!("abc".has_subsequence("c", 2));
        assert!(!"abc".has_subsequence("d", 0));
        assert!(!"abacabad".has_subsequence("aad", 3));
        assert!(!"abacabad".has_subsequence("aad", 2));
        assert!("x".has_subsequence("x", 0));
        assert!(!"x".has_subsequence("y", 0));
        assert!("aaaaa".has_subsequence("aaaaa", 0));
        assert!(!"aaaaa".has_subsequence("aaaaaa", 0));
        assert!("abcdef".has_subsequence("abcdef", 5));
        assert!(!"abcdef".has_subsequence("abcdefg", 5));
        assert!("a b c d e f".has_subsequence("abcdef", 5));
        assert!(!"abcdef".has_subsequence("a b c", 0));
        assert!("abacabadabacaba".has_subsequence("ababa", 5));
        assert!("abacabadabacaba".has_subsequence("ababa", 4));
        assert!("xyxyxyxyxy".has_subsequence("yyy", 3));
        assert!("xyxyxyxyxy".has_subsequence("yyy", 1));
        assert!(!"xyxyxyxyxy".has_subsequence("yyy", 0));
    }

    #[test]
    fn test_table_name_gen() {
        let mut name_gen = SnakeCaseNameGen::with("student");
        assert_eq!(name_gen.next().unwrap(), "student");
        assert_eq!(name_gen.next().unwrap(), "student_2");
        assert_eq!(name_gen.next().unwrap(), "student_3");
        assert_eq!(name_gen.next().unwrap(), "student_4");
    }
}
