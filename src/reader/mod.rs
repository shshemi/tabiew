mod arrow;
mod csv;
mod excel;
mod fwf;
mod json;
mod json_line;
mod logfmt;
mod parquet;
mod sqlite;

use anyhow::Ok;
pub use arrow::ArrowIpcToDataFrame;
pub use csv::CsvToDataFrame;
pub use excel::ExcelToDataFarmes;
pub use fwf::FwfToDataFrame;
pub use json::JsonToDataFrame;
pub use json_line::JsonLineToDataFrame;
pub use logfmt::LogfmtToDataFrame;
pub use parquet::ParquetToDataFrame;
pub use sqlite::SqliteToDataFrames;

use std::{
    borrow::Cow,
    ffi::OsStr,
    path::{Path, PathBuf},
};

use polars::frame::DataFrame;

use crate::{
    AppResult,
    args::{Args, Format},
};

type NamedFrames = Box<[(String, DataFrame)]>;

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

pub trait ReadToDataFrames {
    fn named_frames(&self, input: Source) -> AppResult<NamedFrames>;
}

pub trait BuildReader {
    fn build_reader(&self, path: impl AsRef<Path>) -> AppResult<Box<dyn ReadToDataFrames>>;
}

impl BuildReader for Args {
    fn build_reader(&self, path: impl AsRef<Path>) -> AppResult<Box<dyn ReadToDataFrames>> {
        match self.format {
            Some(Format::Dsv) | Some(Format::Csv) => Ok(Box::new(CsvToDataFrame::from_args(self))),
            Some(Format::Tsv) => {
                let reader = CsvToDataFrame::from_args(self).with_separator('\t');
                Ok(Box::new(reader))
            }
            Some(Format::Parquet) => Ok(Box::new(ParquetToDataFrame)),
            Some(Format::Json) => Ok(Box::new(JsonToDataFrame::from_args(self))),
            Some(Format::Jsonl) => Ok(Box::new(JsonLineToDataFrame::from_args(self))),
            Some(Format::Arrow) => Ok(Box::new(ArrowIpcToDataFrame)),
            Some(Format::Fwf) => Ok(Box::new(FwfToDataFrame::from_args(self))),
            Some(Format::Sqlite) => Ok(Box::new(SqliteToDataFrames::from_args(self))),
            Some(Format::Excel) => Ok(Box::new(ExcelToDataFarmes::from_args(self))),
            Some(Format::Logfmt) => Ok(Box::new(LogfmtToDataFrame::from_args(self))),
            None => match path.as_ref().extension().and_then(|ext| ext.to_str()) {
                Some("tsv") => {
                    let reader = CsvToDataFrame::from_args(self).with_separator('\t');
                    Ok(Box::new(reader))
                }
                Some("parquet") | Some("pqt") => Ok(Box::new(ParquetToDataFrame)),
                Some("json") => Ok(Box::new(JsonToDataFrame::from_args(self))),
                Some("jsonl") => Ok(Box::new(JsonLineToDataFrame::from_args(self))),
                Some("arrow") => Ok(Box::new(ArrowIpcToDataFrame)),
                Some("fwf") => Ok(Box::new(FwfToDataFrame::from_args(self))),
                Some("db") | Some("sqlite") => Ok(Box::new(SqliteToDataFrames::from_args(self))),
                Some("xls") | Some("xlsx") | Some("xlsm") | Some("xlsb") => {
                    Ok(Box::new(ExcelToDataFarmes::from_args(self)))
                }
                _ => Ok(Box::new(CsvToDataFrame::from_args(self))),
            },
        }
    }
}
