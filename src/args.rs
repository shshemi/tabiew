use clap::{Parser, ValueEnum};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(help = "Path(s) to the file(s) to be opened.", required = true)]
    pub files: Vec<PathBuf>,

    #[arg(short, long, help = "Input file format",
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
}

#[derive(Debug, Clone, ValueEnum)]
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

impl From<&InferSchema> for Option<usize> {
    fn from(value: &InferSchema) -> Self {
        match value {
            InferSchema::No => Some(0),
            InferSchema::Fast => Some(128),
            InferSchema::Full => None,
            InferSchema::Safe => Some(0),
        }
    }
}
