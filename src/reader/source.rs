use std::{borrow::Cow, ffi::OsStr, path::PathBuf};

#[derive(Debug, Clone)]
pub enum Source {
    File(PathBuf),
    Stdin,
}

impl Source {
    pub fn table_name(&self) -> String {
        match self {
            Source::File(path_buf) => path_buf
                .file_stem()
                .map(OsStr::to_string_lossy)
                .unwrap_or("unknown".into())
                .into_owned(),
            Source::Stdin => String::from("Stdin"),
        }
    }

    pub fn display_path(&self) -> Cow<'_, str> {
        match self {
            Source::File(path_buf) => path_buf.file_name().unwrap_or_default().to_string_lossy(),
            Source::Stdin => Cow::Borrowed("Stdin"),
        }
    }
}
