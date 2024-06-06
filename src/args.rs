use std::path::PathBuf;
use std::str::FromStr;
use std::fmt::Display;
use clap::Parser;

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
        help = "Schema inference method {no, fast, full, safe}.",
        required = false,
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
        default_value_t = AppTheme::Monokai
    )]
    pub theme: AppTheme,
}

#[derive(Debug, Clone)]
pub enum InferSchema {
    No,
    Fast,
    Full,
    Safe,
}

impl FromStr for InferSchema {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "no" => Ok(InferSchema::No),
            "fast" => Ok(InferSchema::Fast),
            "full" => Ok(InferSchema::Full),
            "safe" => Ok(InferSchema::Safe),
            _ => Err("Invalid schema inference option"),
        }
    }
}

impl Display for InferSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InferSchema::No => write!(f, "no"),
            InferSchema::Fast => write!(f, "fast"),
            InferSchema::Full => write!(f, "full"),
            InferSchema::Safe => write!(f, "safe"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum AppTheme {
    Monokai,
    Terminal
}

impl FromStr for AppTheme {
    type Err= &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "monokai" => Ok(AppTheme::Monokai),
            "terminal" => Ok(AppTheme::Terminal),
            _ => Err("Invalid theme")
        }
    }
}

impl Display for AppTheme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self{
            AppTheme::Monokai => write!(f, "monokai"),
            AppTheme::Terminal => write!(f, "terminal"),
        }
    }
}
