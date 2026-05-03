use anyhow::anyhow;
use std::{borrow::Cow, path::PathBuf, str::FromStr};
use url::Url;

#[derive(Debug, Clone, Hash)]
pub enum DataSource {
    Stdin,
    File(PathBuf),
    Url(Url),
}

impl DataSource {
    pub fn display_path(&self) -> Cow<'_, str> {
        match self {
            DataSource::Stdin => Cow::Borrowed("Stdin"),
            DataSource::File(path_buf) => {
                path_buf.file_name().unwrap_or_default().to_string_lossy()
            }
            DataSource::Url(url) => url.as_str().into(),
        }
    }
}

impl FromStr for DataSource {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "-" {
            return Ok(DataSource::Stdin);
        }
        match Url::parse(s) {
            Ok(url) => match url.scheme() {
                "http" | "https" => Ok(DataSource::Url(url)),
                _ => Err(anyhow!("Unsupported url scheme: {}", url.scheme())),
            },
            Err(_) => Ok(DataSource::File(PathBuf::from(s))),
        }
    }
}
