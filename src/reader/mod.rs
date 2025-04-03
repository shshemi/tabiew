mod fwf;
mod sqlite;

use anyhow::{Ok, anyhow};
pub use fwf::FwfToDataFrame;
pub use sqlite::SqliteToDataFrames;

use std::{
    ffi::OsStr,
    fs::File,
    io::{self},
    path::{Path, PathBuf},
};

use polars::{
    frame::DataFrame,
    io::{SerReader, mmap::MmapBytesReader},
    prelude::{
        CsvParseOptions, CsvReadOptions, IpcReader, JsonLineReader, JsonReader, ParquetReader,
    },
};

use crate::{
    AppResult,
    args::{Args, Format, InferSchema},
    misc::{
        polars_ext::SafeInferSchema,
        type_ext::{ReadToCursor, ToAscii},
    },
};

type NamedFrames = Box<[(String, DataFrame)]>;

#[derive(Debug)]
pub enum Input {
    File(PathBuf),
    Stdin,
}

impl Input {
    pub fn table_name(&self) -> String {
        match self {
            Input::File(path_buf) => path_buf
                .file_stem()
                .map(OsStr::to_string_lossy)
                .unwrap_or("unknown".into())
                .into_owned(),
            Input::Stdin => String::from("Stdin"),
        }
    }
}

pub trait ReadToDataFrames {
    fn named_frames(&self, input: Input) -> AppResult<NamedFrames>;
}

pub trait BuildReader {
    fn build_reader(&self, path: impl AsRef<Path>) -> AppResult<Box<dyn ReadToDataFrames>>;
}

impl BuildReader for Args {
    fn build_reader(&self, path: impl AsRef<Path>) -> AppResult<Box<dyn ReadToDataFrames>> {
        match self.format {
            Some(Format::Dsv) => Ok(Box::new(CsvToDataFrame::from_args(self))),
            Some(Format::Parquet) => Ok(Box::new(ParquetToDataFrame)),
            Some(Format::Json) => Ok(Box::new(JsonToDataFrame::from_args(self))),
            Some(Format::Jsonl) => Ok(Box::new(JsonLineToDataFrame::from_args(self))),
            Some(Format::Arrow) => Ok(Box::new(ArrowIpcToDataFrame)),
            Some(Format::Fwf) => Ok(Box::new(FwfToDataFrame::from_args(self)?)),
            Some(Format::Sqlite) => Ok(Box::new(SqliteToDataFrames)),
            None => match path.as_ref().extension().and_then(|ext| ext.to_str()) {
                Some("tsv") => {
                    let mut reader = CsvToDataFrame::from_args(self);
                    reader.separator_char = '\t';
                    Ok(Box::new(reader))
                }
                Some("parquet") => Ok(Box::new(ParquetToDataFrame)),
                Some("json") => Ok(Box::new(JsonToDataFrame::from_args(self))),
                Some("jsonl") => Ok(Box::new(JsonLineToDataFrame::from_args(self))),
                Some("arrow") => Ok(Box::new(ArrowIpcToDataFrame)),
                Some("fwf") => Ok(Box::new(FwfToDataFrame::from_args(self)?)),
                Some("db") | Some("sqlite") => Ok(Box::new(SqliteToDataFrames)),
                _ => Ok(Box::new(CsvToDataFrame::from_args(self))),
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
    truncate_ragged_lines: bool,
}

impl CsvToDataFrame {
    pub fn from_args(args: &Args) -> Self {
        Self {
            infer_schema: args.infer_schema,
            quote_char: args.quote_char,
            separator_char: args.separator,
            no_header: args.no_header,
            ignore_errors: args.ignore_errors,
            truncate_ragged_lines: args.truncate_ragged_lines,
        }
    }

    pub fn with_separator(mut self, c: char) -> Self {
        self.separator_char = c;
        self
    }

    pub fn with_no_header(mut self, no_header: bool) -> Self {
        self.no_header = no_header;
        self
    }

    pub fn with_quote_char(mut self, c: char) -> Self {
        self.quote_char = c;
        self
    }

    fn try_into_frame(&self, reader: impl MmapBytesReader) -> AppResult<DataFrame> {
        let mut df = CsvReadOptions::default()
            .with_ignore_errors(self.ignore_errors)
            .with_infer_schema_length(self.infer_schema.to_csv_infer_schema_length())
            .with_has_header(!self.no_header)
            .with_parse_options(
                CsvParseOptions::default()
                    .with_truncate_ragged_lines(self.truncate_ragged_lines)
                    .with_quote_char(self.quote_char.to_ascii())
                    .with_separator(
                        self.separator_char
                            .to_ascii()
                            .ok_or(anyhow!("non-ASCII separator character"))?,
                    ),
            )
            .with_rechunk(true)
            .into_reader_with_file_handle(reader)
            .finish()?;
        if matches!(self.infer_schema, InferSchema::Safe) {
            df.safe_infer_schema();
        }
        Ok(df)
    }
}

impl Default for CsvToDataFrame {
    fn default() -> Self {
        Self {
            infer_schema: InferSchema::Safe,
            quote_char: '"',
            separator_char: ',',
            no_header: false,
            ignore_errors: true,
            truncate_ragged_lines: false,
        }
    }
}

impl ReadToDataFrames for CsvToDataFrame {
    fn named_frames(&self, input: Input) -> AppResult<NamedFrames> {
        let df = match &input {
            Input::File(path) => self.try_into_frame(File::open(path)?),
            Input::Stdin => self.try_into_frame(io::stdin().read_to_cursor()?),
        }?;
        Ok([(input.table_name(), df)].into())
    }
}

pub struct ParquetToDataFrame;

impl ReadToDataFrames for ParquetToDataFrame {
    fn named_frames(&self, input: Input) -> AppResult<NamedFrames> {
        let df = match &input {
            Input::File(path) => ParquetReader::new(File::open(path)?)
                .set_rechunk(true)
                .finish()?,

            Input::Stdin => ParquetReader::new(io::stdin().read_to_cursor()?)
                .set_rechunk(true)
                .finish()?,
        };
        Ok([(input.table_name(), df)].into())
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

impl Default for JsonLineToDataFrame {
    fn default() -> Self {
        Self {
            infer_schema: InferSchema::Safe,
            ignore_errors: true,
        }
    }
}

impl ReadToDataFrames for JsonLineToDataFrame {
    fn named_frames(&self, input: Input) -> AppResult<NamedFrames> {
        let mut df = match &input {
            Input::File(path) => JsonLineReader::new(File::open(path)?)
                .with_rechunk(true)
                .infer_schema_len(None)
                .with_ignore_errors(self.ignore_errors)
                .finish()?,
            Input::Stdin => JsonLineReader::new(io::stdin().read_to_cursor()?)
                .with_rechunk(true)
                .infer_schema_len(None)
                .with_ignore_errors(self.ignore_errors)
                .finish()?,
        };
        if matches!(
            self.infer_schema,
            InferSchema::Safe | InferSchema::Full | InferSchema::Fast
        ) {
            df.safe_infer_schema();
        }
        Ok([(input.table_name(), df)].into())
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

impl Default for JsonToDataFrame {
    fn default() -> Self {
        Self {
            infer_schema: InferSchema::Full,
            ignore_errors: true,
        }
    }
}

impl ReadToDataFrames for JsonToDataFrame {
    fn named_frames(&self, input: Input) -> AppResult<NamedFrames> {
        let mut df = match &input {
            Input::File(path) => JsonReader::new(File::open(path)?)
                .set_rechunk(true)
                .infer_schema_len(None)
                .with_ignore_errors(self.ignore_errors)
                .finish()?,
            Input::Stdin => JsonReader::new(io::stdin().read_to_cursor()?)
                .set_rechunk(true)
                .infer_schema_len(None)
                .with_ignore_errors(self.ignore_errors)
                .finish()?,
        };
        if matches!(
            self.infer_schema,
            InferSchema::Safe | InferSchema::Full | InferSchema::Fast
        ) {
            df.safe_infer_schema();
        }
        Ok([(input.table_name(), df)].into())
    }
}

pub struct ArrowIpcToDataFrame;

impl ReadToDataFrames for ArrowIpcToDataFrame {
    fn named_frames(&self, input: Input) -> AppResult<NamedFrames> {
        let df = match &input {
            Input::File(path) => IpcReader::new(File::open(path)?)
                .set_rechunk(true)
                .finish()?,
            Input::Stdin => IpcReader::new(io::stdin().read_to_cursor()?)
                .set_rechunk(true)
                .finish()?,
        };
        Ok([(input.table_name(), df)].into())
    }
}
