use std::path::Path;
use std::sync::Arc;
use std::sync::mpsc::SyncSender;

use polars::frame::DataFrame;
use polars::prelude::Schema;

use crate::{
    AppResult,
    args::{Args, Format},
    reader::{
        ArrowIpcToDataFrame, CsvToDataFrame, ExcelToDataFarmes, FwfToDataFrame,
        JsonLineToDataFrame, JsonToDataFrame, LogfmtToDataFrame, ParquetToDataFrame, Source,
        SqliteToDataFrames,
    },
};

pub type NamedFrame = (String, DataFrame);
pub type NamedFrames = Box<[NamedFrame]>;

/// Shared batching knobs for streaming readers.
#[derive(Debug, Clone, Copy)]
pub struct StreamingConfig {
    pub batch_rows: usize,
    pub batch_ms: u64,
}

impl StreamingConfig {
    pub fn from_args(args: &Args) -> Self {
        Self {
            batch_rows: args.stream_batch_rows.max(1),
            batch_ms: args.stream_batch_ms.max(1),
        }
    }
}

pub trait ReadToDataFrames {
    fn read_to_data_frames(&self, input: Source) -> AppResult<NamedFrames>;
}

pub trait BuildReader {
    fn build_reader(&self, path: impl AsRef<Path>) -> AppResult<Box<dyn ReadToDataFrames>>;
}

/// Events emitted by a streaming reader running on a background thread.
#[derive(Debug)]
pub enum StreamEvent {
    /// Initial schema or schema update (e.g. logfmt growing a new column).
    Schema {
        name: String,
        schema: Arc<Schema>,
    },
    /// A batch of new rows. Schema must match the most recent `Schema` event.
    Batch { name: String, rows: DataFrame },
    /// Producer reached end-of-stream cleanly.
    Eof { name: String },
    /// Fatal error; the producer thread will exit after sending this.
    Error {
        name: String,
        error: anyhow::Error,
    },
}

/// A reader capable of incrementally producing dataframe batches from a
/// streaming source. Implementations are consumed by `stream_to_data_frames`,
/// which spawns a background thread that owns the producer until EOF or error.
pub trait StreamToDataFrames: Send {
    fn stream_to_data_frames(self: Box<Self>, input: Source, sender: SyncSender<StreamEvent>);
}

/// Builder selecting the appropriate streaming reader for the configured
/// format. Returns `None` for non-streamable formats.
pub trait BuildStreamReader {
    fn build_stream_reader(&self) -> Option<Box<dyn StreamToDataFrames>>;
}

impl BuildStreamReader for Args {
    fn build_stream_reader(&self) -> Option<Box<dyn StreamToDataFrames>> {
        let resolved = self.format.clone().unwrap_or(Format::Csv);
        match resolved {
            Format::Jsonl => Some(Box::new(
                crate::reader::json_line::JsonLineStreamReader::from_args(self),
            )),
            // Other streamable formats are wired in Phase 5.
            _ => None,
        }
    }
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
