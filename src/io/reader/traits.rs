use std::path::Path;

use polars::frame::DataFrame;

use crate::{
    AppResult,
    args::{Args, Format},
    io::{
        Resource,
        reader::{
            ArrowIpcToDataFrame, CsvToDataFrame, ExcelToDataFrames, FwfToDataFrame,
            JsonLineToDataFrame, JsonToDataFrame, LogfmtToDataFrame, ParquetToDataFrame,
            SqliteToDataFrames,
        },
    },
};

pub type NamedFrame = (String, DataFrame);
pub type NamedFrames = Box<[NamedFrame]>;

pub trait ReadToDataFrames {
    fn read_to_data_frames(&self, input: Resource) -> AppResult<NamedFrames>;
}

pub trait BuildReader {
    fn build_reader(&self, path: impl AsRef<Path>) -> AppResult<Box<dyn ReadToDataFrames>>;
}

impl BuildReader for Args {
    fn build_reader(&self, path: impl AsRef<Path>) -> AppResult<Box<dyn ReadToDataFrames>> {
        match self.format {
            Some(Format::Dsv) | Some(Format::Csv) => Ok(Box::new(CsvToDataFrame::from_args(self))),
            Some(Format::Tsv) => Ok(Box::new(
                CsvToDataFrame::from_args(self).with_separator('\t'),
            )),
            Some(Format::Parquet) => Ok(Box::new(ParquetToDataFrame)),
            Some(Format::Json) => Ok(Box::new(JsonToDataFrame::from_args(self))),
            Some(Format::Jsonl) => Ok(Box::new(JsonLineToDataFrame::from_args(self))),
            Some(Format::Arrow) => Ok(Box::new(ArrowIpcToDataFrame)),
            Some(Format::Fwf) => Ok(Box::new(FwfToDataFrame::from_args(self))),
            Some(Format::Sqlite) => Ok(Box::new(SqliteToDataFrames::from_args(self))),
            Some(Format::Excel) => Ok(Box::new(ExcelToDataFrames::from_args(self))),
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
                    Ok(Box::new(ExcelToDataFrames::from_args(self)))
                }
                _ => Ok(Box::new(CsvToDataFrame::from_args(self))),
            },
        }
    }
}
