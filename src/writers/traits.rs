use std::path::PathBuf;

use polars::frame::DataFrame;

use crate::AppResult;

#[derive(Debug, Clone)]
pub enum Destination {
    File(PathBuf),
    Clipboard,
}

pub trait WriteToFile {
    fn write_to_file(&self, dest: Destination, data_frame: &mut DataFrame) -> AppResult<()>;
}
