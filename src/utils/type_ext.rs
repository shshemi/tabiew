pub trait ToAscii {
    fn to_ascii(self) -> Option<u8>;
}

impl ToAscii for char {
    #[inline]
    fn to_ascii(self) -> Option<u8> {
        self.is_ascii().then_some(self as u8)
    }
}
