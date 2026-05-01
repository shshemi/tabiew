use std::{borrow::Cow, path::PathBuf};

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
