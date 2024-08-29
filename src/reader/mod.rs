use std::{fs::File, path::PathBuf};

use polars::{
    frame::DataFrame,
    io::SerReader,
    prelude::{CsvParseOptions, CsvReadOptions, ParquetReader},
};

use crate::{
    args::{Args, InferSchema},
    utils::{as_ascii, safe_infer_schema},
    AppResult,
};

pub trait ReadToDataFrame {
    fn read_to_data_frame(&self, file: PathBuf) -> AppResult<DataFrame>;
}

pub trait BuildReader {
    fn build_reader(&self) -> Box<dyn ReadToDataFrame>;
}

impl BuildReader for Args {
    fn build_reader(&self) -> Box<dyn ReadToDataFrame> {
        match self.format {
            crate::args::Format::Dsv => Box::new(CsvToDataFrame::try_from_args(self)),
            crate::args::Format::Parquet => Box::new(ParquetToDataFrame),
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
    pub fn try_from_args(args: &Args) -> CsvToDataFrame {
        Self {
            infer_schema: args.infer_schema,
            quote_char: args.quote_char,
            separator_char: args.separator,
            no_header: args.no_header,
            ignore_errors: args.ignore_errors,
        }
    }
}

impl ReadToDataFrame for CsvToDataFrame {
    fn read_to_data_frame(&self, file: PathBuf) -> AppResult<DataFrame> {
        let mut df = CsvReadOptions::default()
            .with_ignore_errors(self.ignore_errors)
            .with_infer_schema_length(self.infer_schema.to_infer_schema_length())
            .with_has_header(!self.no_header)
            .with_parse_options(
                CsvParseOptions::default()
                    .with_quote_char(as_ascii(self.quote_char))
                    .with_separator(as_ascii(self.separator_char).expect("Invalid separator")),
            )
            .try_into_reader_with_file_path(file.into())?
            .finish()?;
        if matches!(self.infer_schema, InferSchema::Safe) {
            safe_infer_schema(&mut df);
        }
        Ok(df)
    }
}

pub struct ParquetToDataFrame;

impl ReadToDataFrame for ParquetToDataFrame {
    fn read_to_data_frame(&self, file: PathBuf) -> AppResult<DataFrame> {
        Ok(ParquetReader::new(File::open(&file)?)
            .set_rechunk(true)
            .finish()?)
    }
}
