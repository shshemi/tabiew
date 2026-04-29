use std::{
    borrow::Cow,
    convert::Infallible,
    ffi::OsStr,
    path::{Path, PathBuf},
    str::FromStr,
};

use strum_macros::{EnumIter, IntoStaticStr};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum DataSource {
    File(PathBuf),
    Stdin,
    Url(String),
}

impl DataSource {
    pub fn table_name(&self) -> String {
        match self {
            DataSource::File(path_buf) => path_buf
                .file_stem()
                .map(OsStr::to_string_lossy)
                .unwrap_or("unknown".into())
                .into_owned(),
            DataSource::Stdin => String::from("Stdin"),
            DataSource::Url(url) => Path::new(
                url.split(['?', '#'])
                    .next()
                    .unwrap_or(url)
                    .rsplit('/')
                    .next()
                    .unwrap_or(""),
            )
            .file_stem()
            .map(|s| s.to_string_lossy().into_owned())
            .unwrap_or_else(|| "url".into()),
        }
    }

    pub fn display_path(&self) -> Cow<'_, str> {
        match self {
            DataSource::File(path_buf) => {
                path_buf.file_name().unwrap_or_default().to_string_lossy()
            }
            DataSource::Stdin => Cow::Borrowed("Stdin"),
            DataSource::Url(url) => Cow::Borrowed(url),
        }
    }

    pub fn resource_type(&self) -> DataSourceType {
        match self {
            DataSource::File(_) => DataSourceType::File,
            DataSource::Stdin => DataSourceType::Stdin,
            DataSource::Url(_) => DataSourceType::Url,
        }
    }
}

impl FromStr for DataSource {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("http://") || s.starts_with("https://") {
            Ok(DataSource::Url(s.to_owned()))
        } else {
            Ok(DataSource::File(PathBuf::from(s)))
        }
    }
}

#[derive(Debug, Clone, Copy, IntoStaticStr, EnumIter)]
pub enum DataSourceType {
    File,
    Stdin,
    Url,
}
