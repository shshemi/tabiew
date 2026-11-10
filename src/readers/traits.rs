use polars::frame::DataFrame;

use crate::{
    AppResult,
    args::Format,
    duckdb::IngestOptions,
    readers::{
        ArrowIpcToDataFrame, AvroToDataFrame, ExcelToDataFrames, HtmlToDataFrame,
        JsonLineToDataFrame, JsonToDataFrame, LogfmtToDataFrame, MarkdownToDataFrame,
        ParquetToDataFrame, ReaderSource,
    },
};

pub type NamedFrame = (String, DataFrame);
pub type NamedFrames = Box<[NamedFrame]>;

pub trait DataFrameReader {
    fn read_to_data_frames(&self, source: ReaderSource) -> AppResult<NamedFrames>;
}

pub trait IngestFormat {
    fn ingest_options(&self, base: IngestOptions) -> IngestOptions;
}

macro_rules! ingest_format {
    ($($reader:ty => $format:ident),* $(,)?) => {
        $(impl IngestFormat for $reader {
            fn ingest_options(&self, base: IngestOptions) -> IngestOptions {
                IngestOptions {
                    format: Some(Format::$format),
                    ..base
                }
            }
        })*
    };
}

ingest_format!(
    ArrowIpcToDataFrame => Arrow,
    AvroToDataFrame => Avro,
    ExcelToDataFrames => Excel,
    HtmlToDataFrame => Html,
    JsonLineToDataFrame => Jsonl,
    JsonToDataFrame => Json,
    LogfmtToDataFrame => Logfmt,
    MarkdownToDataFrame => Markdown,
    ParquetToDataFrame => Parquet,
);
