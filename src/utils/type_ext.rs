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
        assert!(!"abacabadabacaba".has_subsequence("ababa", 4));
        assert!("xyxyxyxyxy".has_subsequence("yyy", 3));
        assert!(!"xyxyxyxyxy".has_subsequence("yyy", 2));
    }
}
