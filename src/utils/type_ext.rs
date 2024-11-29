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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_subsequence() {
        assert!("abcde".has_subsequence("bcd"));
        assert!("hello".has_subsequence("hell"));
        assert!("hello".has_subsequence("lo"));
        assert!("".has_subsequence(""));
        assert!("abc".has_subsequence(""));
        assert!(!"".has_subsequence("abc"));
        assert!("a".has_subsequence("a"));
        assert!(!"a".has_subsequence("b"));
        assert!("abc".has_subsequence("a"));
        assert!("abc".has_subsequence("abc"));
        assert!("aaaa".has_subsequence("aaa"));
        assert!("abcdefghij".has_subsequence("def"));
        assert!(!"abcdefghij".has_subsequence("xyz"));
        assert!("a quick brown fox".has_subsequence("quick"));
        assert!(!"a quick brown fox".has_subsequence("brownie"));
    }
}
