use clap::{Parser, ValueEnum};
use std::{num::NonZero, path::PathBuf};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(help = "Path(s) to the file(s) to be opened.", required = false)]
    pub files: Vec<PathBuf>,

    #[arg(short, long, help = "Path to the startup script.", required = false)]
    pub script: Option<PathBuf>,

    #[arg(
        short,
        long,
        help = "Input file format",
        value_enum,
        default_value_t = Format::Dsv)]
    pub format: Format,

    #[arg(
        long,
        help = "Specify if the DSV file does not contain a header row.",
        default_value_t = false
    )]
    pub no_header: bool,

    #[arg(
        long,
        help = "If set, parsing errors while loading the DSV file will be ignored.",
        default_value_t = false
    )]
    pub ignore_errors: bool,

    #[arg(
        long,
        help = "Method to infer the DSV schema while loading the file.",
        required = false,
        value_enum,
        default_value_t = InferSchema::Safe,
    )]
    pub infer_schema: InferSchema,

    #[arg(
        long,
        help = "Character used as the field separator or delimiter while loading the DSV file.",
        required = false,
        default_value_t = ','
    )]
    pub separator: char,

    #[arg(
        long,
        help = "Character used to quote fields while loading the DSV file.",
        required = false,
        default_value_t = '"'
    )]
    pub quote_char: char,

    #[arg(
        long,
        help = "A comma-separeted list of widths, which should contain the separator spaces if any, to parse FWF files.",
        required = false,
        default_value_t = String::default(),
    )]
    pub widths: String,

    #[arg(
        long,
        help = "The length of the separator used in FWF files.",
        required = false,
        default_value_t = 1_usize
    )]
    pub separator_length: usize,

    #[arg(
        long,
        help = "If set, FWF files are going have strict column width restrictions",
        required = false,
        default_value_t = false
    )]
    pub no_flexible_width: bool,

    #[arg(
        long,
        help = "Tabiew theme",
        required = false,
        value_enum,
        default_value_t = AppTheme::Monokai
    )]
    pub theme: AppTheme,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum Format {
    Dsv,
    Parquet,
    Jsonl,
    Json,
    Arrow,
    Fwf,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum InferSchema {
    No,
    Fast,
    Full,
    Safe,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum AppTheme {
    Monokai,
    Argonaut,
    Terminal,
}

impl InferSchema {
    pub fn to_csv_infer_schema_length(&self) -> Option<usize> {
        match self {
            InferSchema::No => Some(0),
            InferSchema::Fast => Some(128),
            InferSchema::Full => None,
            InferSchema::Safe => Some(0),
        }
    }

    pub fn to_json_infer_schema_length(&self) -> Option<NonZero<usize>> {
        match self {
            InferSchema::No => None,
            InferSchema::Fast => Some(NonZero::new(128).unwrap()),
            InferSchema::Full => None,
            InferSchema::Safe => None,
        }
    }
}
