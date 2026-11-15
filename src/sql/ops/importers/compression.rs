#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Compression {
    Auto,
    None,
    Gzip,
    Zstd,
}

impl Compression {
    pub fn as_sql(&self) -> &'static str {
        match self {
            Compression::Auto => "auto",
            Compression::None => "none",
            Compression::Gzip => "gzip",
            Compression::Zstd => "zstd",
        }
    }
}
