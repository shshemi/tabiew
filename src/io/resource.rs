use std::{borrow::Cow, ffi::OsStr, path::PathBuf};

#[derive(Debug, Clone)]
pub enum Resource {
    LocalFile(PathBuf),
    Stdin,
}

impl Resource {
    pub fn table_name(&self) -> String {
        match self {
            Resource::LocalFile(path_buf) => path_buf
                .file_stem()
                .map(OsStr::to_string_lossy)
                .unwrap_or("unknown".into())
                .into_owned(),
            Resource::Stdin => String::from("Stdin"),
        }
    }

    pub fn display_path(&self) -> Cow<'_, str> {
        match self {
            Resource::LocalFile(path_buf) => {
                path_buf.file_name().unwrap_or_default().to_string_lossy()
            }
            Resource::Stdin => Cow::Borrowed("Stdin"),
        }
    }

    pub fn resource_type(&self) -> ResourceType {
        match self {
            Resource::LocalFile(_) => ResourceType::LocalFile,
            Resource::Stdin => ResourceType::Stdin,
        }
    }
}

pub enum ResourceType {
    LocalFile,
    Stdin,
}
