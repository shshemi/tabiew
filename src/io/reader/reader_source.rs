use std::{borrow::Cow, convert::Infallible, ffi::OsStr, path::PathBuf, str::FromStr};

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

    pub fn display_path(&self) -> Cow<'_, str> {
        match self {
            ReaderSource::File(path_buf) => {
                path_buf.file_name().unwrap_or_default().to_string_lossy()
            }
            ReaderSource::Stdin => Cow::Borrowed("Stdin"),
        }
    }

    // pub fn resource_type(&self) -> ReaderSourceType {
    //     match self {
    //         ReaderSource::File(_) => ReaderSourceType::File,
    //         ReaderSource::Stdin => ReaderSourceType::Stdin,
    //     }
    // }
}

impl FromStr for ReaderSource {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(ReaderSource::File(PathBuf::from(s)))
    }
}

// #[derive(Debug, Clone, Copy, IntoStaticStr, EnumIter)]
// pub enum DataSourceType {
//     File,
//     Stdin,
//     Url,
// }
