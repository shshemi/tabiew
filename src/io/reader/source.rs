use std::{convert::Infallible, ffi::OsStr, path::PathBuf, str::FromStr};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum ReaderSource {
    File(PathBuf),
    Stdin,
}

impl ReaderSource {
    pub fn table_name(&self) -> String {
        match self {
            ReaderSource::File(path_buf) => path_buf
                .file_stem()
                .map(OsStr::to_string_lossy)
                .unwrap_or("unknown".into())
                .into_owned(),
            ReaderSource::Stdin => String::from("Stdin"),
        }
    }
}

impl From<PathBuf> for ReaderSource {
    fn from(value: PathBuf) -> Self {
        ReaderSource::File(value)
    }
}

impl FromStr for ReaderSource {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(ReaderSource::File(PathBuf::from(s)))
    }
}
