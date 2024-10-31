mod fwf;

use std::path::Path;

use fwf::ReadFwfToDataFrame;
use polars::{
    frame::DataFrame,
    io::{mmap::MmapBytesReader, SerReader},
    prelude::{
        CsvParseOptions, CsvReadOptions, IpcReader, JsonLineReader, JsonReader, ParquetReader,
    },
};

use crate::{
    args::{Args, Format, InferSchema},
    utils::{as_ascii, safe_infer_schema},
    AppResult,
};

pub trait ReadToDataFrame<R> {
    fn read_to_data_frame(&self, reader: R) -> AppResult<DataFrame>;
}

pub trait BuildReader<R> {
    fn build_reader<P: AsRef<Path>>(&self, path: P) -> Box<dyn ReadToDataFrame<R>>;
}

impl<R: MmapBytesReader> BuildReader<R> for Args {
    fn build_reader<P: AsRef<Path>>(&self, path: P) -> Box<dyn ReadToDataFrame<R>> {
        match self.format {
            Some(Format::Dsv) => Box::new(CsvToDataFrame::from_args(self)),
            Some(Format::Parquet) => Box::new(ParquetToDataFrame),
            Some(Format::Json) => Box::new(JsonToDataFrame::from_args(self)),
            Some(Format::Jsonl) => Box::new(JsonLineToDataFrame::from_args(self)),
            Some(Format::Arrow) => Box::new(ArrowIpcToDataFrame),
            Some(Format::Fwf) => Box::new(ReadFwfToDataFrame::from_args(self)),
            None => match path.as_ref().extension().and_then(|ext| ext.to_str()) {
                Some("tsv") => {
                    let mut reader = CsvToDataFrame::from_args(self);
                    reader.separator_char = '\t';
                    Box::new(reader)
                }
                Some("parquet") => Box::new(ParquetToDataFrame),
                Some("json") => Box::new(JsonToDataFrame::from_args(self)),
                Some("jsonl") => Box::new(JsonLineToDataFrame::from_args(self)),
                Some("arrow") => Box::new(ArrowIpcToDataFrame),
                Some("fwf") => Box::new(ReadFwfToDataFrame::from_args(self)),
                _ => Box::new(CsvToDataFrame::from_args(self)),
            },
        }
    }
}

pub struct CsvToDataFrame {
    infer_schema: InferSchema,
    quote_char: char,
    separator_char: char,
    no_header: bool,
    ignore_errors: bool,
}

impl CsvToDataFrame {
    pub fn from_args(args: &Args) -> Self {
        Self {
            infer_schema: args.infer_schema,
            quote_char: args.quote_char,
            separator_char: args.separator,
            no_header: args.no_header,
            ignore_errors: args.ignore_errors,
        }
    }
}

impl<R: MmapBytesReader> ReadToDataFrame<R> for CsvToDataFrame {
    fn read_to_data_frame(&self, reader: R) -> AppResult<DataFrame> {
        let mut df = CsvReadOptions::default()
            .with_ignore_errors(self.ignore_errors)
            .with_infer_schema_length(self.infer_schema.to_csv_infer_schema_length())
            .with_has_header(!self.no_header)
            .with_parse_options(
                CsvParseOptions::default()
                    .with_quote_char(as_ascii(self.quote_char))
                    .with_separator(as_ascii(self.separator_char).expect("Invalid separator")),
            )
            .into_reader_with_file_handle(reader)
            .finish()?;
        if matches!(self.infer_schema, InferSchema::Safe) {
            safe_infer_schema(&mut df);
        }
        Ok(df)
    }
}

pub struct ParquetToDataFrame;

impl<R: MmapBytesReader> ReadToDataFrame<R> for ParquetToDataFrame {
    fn read_to_data_frame(&self, reader: R) -> AppResult<DataFrame> {
        Ok(ParquetReader::new(reader).set_rechunk(true).finish()?)
    }
}

pub struct JsonLineToDataFrame {
    infer_schema: InferSchema,
    ignore_errors: bool,
}

impl JsonLineToDataFrame {
    pub fn from_args(args: &Args) -> Self {
        Self {
            infer_schema: args.infer_schema,
            ignore_errors: args.ignore_errors,
        }
    }
}

impl<R: MmapBytesReader> ReadToDataFrame<R> for JsonLineToDataFrame {
    fn read_to_data_frame(&self, reader: R) -> AppResult<DataFrame> {
        let mut df = JsonLineReader::new(reader)
            .with_rechunk(true)
            .infer_schema_len(None)
            .with_ignore_errors(self.ignore_errors)
            .finish()?;
        if matches!(
            self.infer_schema,
            InferSchema::Safe | InferSchema::Full | InferSchema::Fast
        ) {
            safe_infer_schema(&mut df);
        }
        Ok(df)
    }
}

pub struct JsonToDataFrame {
    infer_schema: InferSchema,
    ignore_errors: bool,
}

impl JsonToDataFrame {
    pub fn from_args(args: &Args) -> Self {
        Self {
            infer_schema: args.infer_schema,
            ignore_errors: args.ignore_errors,
        }
    }
}

impl<R: MmapBytesReader> ReadToDataFrame<R> for JsonToDataFrame {
    fn read_to_data_frame(&self, reader: R) -> AppResult<DataFrame> {
        let mut df = JsonReader::new(reader)
            .set_rechunk(true)
            .infer_schema_len(None)
            .with_ignore_errors(self.ignore_errors)
            .finish()?;
        if matches!(
            self.infer_schema,
            InferSchema::Safe | InferSchema::Full | InferSchema::Fast
        ) {
            safe_infer_schema(&mut df);
        }
        Ok(df)
    }
}

pub struct ArrowIpcToDataFrame;

impl<R: MmapBytesReader> ReadToDataFrame<R> for ArrowIpcToDataFrame {
    fn read_to_data_frame(&self, reader: R) -> AppResult<DataFrame> {
        Ok(IpcReader::new(reader).set_rechunk(true).finish()?)
    }
}
