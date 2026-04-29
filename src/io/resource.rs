use std::{
    borrow::Cow,
    convert::Infallible,
    ffi::OsStr,
    path::{Path, PathBuf},
    str::FromStr,
};

use strum_macros::{EnumIter, IntoStaticStr};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Resource {
    File(PathBuf),
    Stdin,
    Url(String),
}

impl Resource {
    pub fn table_name(&self) -> String {
        match self {
            Resource::File(path_buf) => path_buf
                .file_stem()
                .map(OsStr::to_string_lossy)
                .unwrap_or("unknown".into())
                .into_owned(),
            Resource::Stdin => String::from("Stdin"),
            Resource::Url(url) => Path::new(
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
            Resource::File(path_buf) => path_buf.file_name().unwrap_or_default().to_string_lossy(),
            Resource::Stdin => Cow::Borrowed("Stdin"),
            Resource::Url(url) => Cow::Borrowed(url),
        }
    }

    pub fn resource_type(&self) -> ResourceType {
        match self {
            Resource::File(_) => ResourceType::File,
            Resource::Stdin => ResourceType::Stdin,
            Resource::Url(_) => ResourceType::Url,
        }
    }
}

impl FromStr for Resource {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("http://") || s.starts_with("https://") {
            Ok(Resource::Url(s.to_owned()))
        } else {
            Ok(Resource::File(PathBuf::from(s)))
        }
    }
}

#[derive(Debug, Clone, Copy, IntoStaticStr, EnumIter)]
pub enum ResourceType {
    File,
    Stdin,
    Url,
}
