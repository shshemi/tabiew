use std::path::PathBuf;
use clap::{Parser, ValueEnum};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(help = "File to open", required = true)]
    pub file_name: PathBuf,

    #[arg(
        long,
        help = "CSV file contains no header row",
        default_value_t = false
    )]
    pub no_header: bool,

    #[arg(
        long,
        help = "Ignore parsing errors while loading the CSV",
        default_value_t = false
    )]
    pub ignore_errors: bool,

    #[arg(
        long,
        help = "Schema inference method",
        required = false,
        value_enum,
        default_value_t = InferSchema::Fast,
    )]
    pub infer_schema: InferSchema,

    #[arg(
        long,
        help = "The field separator or delimiter",
        required = false,
        default_value_t = ','
    )]
    pub separator: char,

    #[arg(
        long,
        help = "Quote character",
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
pub enum InferSchema {
    No,
    Fast,
    Full,
    Safe,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum AppTheme {
    Monokai,
    Terminal
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
