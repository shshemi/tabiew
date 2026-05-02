use std::{borrow::Cow, convert::Infallible, path::PathBuf, str::FromStr};

#[derive(Debug, Clone, Hash)]
pub enum DataSource {
    Stdin,
    File(PathBuf),
    Url(String),
}

impl DataSource {
    pub fn display_path(&self) -> Cow<'_, str> {
        match self {
            DataSource::Stdin => Cow::Borrowed("Stdin"),
            DataSource::File(path_buf) => {
                path_buf.file_name().unwrap_or_default().to_string_lossy()
            }
            DataSource::Url(url) => url.into(),
        }
    }
}

impl FromStr for DataSource {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "-" {
            Ok(DataSource::Stdin)
        } else if s.starts_with("http://") || s.starts_with("https://") {
            Ok(DataSource::Url(s.to_owned()))
        } else {
            Ok(DataSource::File(PathBuf::from(s)))
        }
    }
}
