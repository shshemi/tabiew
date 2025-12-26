use clap::{Parser, ValueEnum};
use std::{num::NonZero, path::PathBuf};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(help = "Path(s) to the file(s) to be opened.", required = false)]
    pub files: Vec<PathBuf>,

    #[arg(long, help = "Paths to be opened and concatenated vertically.",
        num_args = 1..,
        required = false)]
    pub multiparts: Vec<PathBuf>,

    #[arg(
        short,
        long,
        help = "Specifies the input format. By default, the format is selected based on the file extension",
        value_enum
    )]
    pub format: Option<Format>,

    #[arg(long, help = "Sets the key for sqlite (if required)", value_enum)]
    pub sqlite_key: Option<String>,

    #[arg(
        long,
        help = "Specifies if the input does not contain a header row.",
        default_value_t = false
    )]
    pub no_header: bool,

    #[arg(
        long,
        help = "Ignores parsing errors while loading.",
        default_value_t = false
    )]
    pub ignore_errors: bool,

    #[arg(
        long,
        help = "Specifies the method to infer the schema.",
        required = false,
        value_enum,
        default_value_t = InferSchema::Safe,
    )]
    pub infer_schema: InferSchema,

    #[arg(
        long,
        help = "Performs additional processing to parse date and datetime columns",
        default_value_t = false
    )]
    pub infer_datetimes: bool,

    #[arg(
        long,
        help = "Character used as the field separator or delimiter while loading DSV files.",
        required = false,
        default_value_t = ','
    )]
    pub separator: char,

    #[arg(
        long,
        help = "Character used to quote fields while loading DSV files.",
        required = false,
        default_value_t = '"'
    )]
    pub quote_char: char,

    #[arg(
        long,
        help = "A comma-separated list of widths, which specifies the column widths for FWF files.",
        required = false,
        default_value_t = String::default(),
    )]
    pub widths: String,

    #[arg(
        long,
        help = "Specifies the separator length for FWF files.",
        required = false,
        default_value_t = 1_usize
    )]
    pub separator_length: usize,

    #[arg(
        long,
        help = "Sets strict column width restrictions for FWF files.",
        required = false,
        default_value_t = false
    )]
    pub no_flexible_width: bool,

    #[arg(
        long,
        help = "Truncate ragged lines while reading the file.",
        required = false,
        default_value_t = false
    )]
    pub truncate_ragged_lines: bool,

    #[arg(
        long,
        help = "Specifies the types to infer for text-based files.",
        required = false,
        default_value_t = TypeVec(vec![Type::Int, Type::Float]),
    )]
    pub infer_types: TypeVec,

    #[arg(
        long,
        help = "Disables type inference",
        required = false,
        default_value_t = false
    )]
    pub no_type_inference: bool,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum Format {
    Dsv,
    Csv,
    Tsv,
    Parquet,
    Jsonl,
    Json,
    Arrow,
    Fwf,
    Sqlite,
    Excel,
}

#[derive(Debug, Clone)]
pub struct TypeVec(Vec<Type>);

impl TypeVec {
    pub fn inner(&self) -> &[Type] {
        &self.0
    }
}

impl std::fmt::Display for TypeVec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let type_strings: Vec<String> = self.0.iter().map(|t| t.to_string()).collect();
        write!(f, "{}", type_strings.join(" "))
    }
}

impl std::str::FromStr for TypeVec {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.split(' ')
            .map(|t| t.trim().parse::<Type>())
            .collect::<Result<Vec<_>, _>>()
            .map(TypeVec)
    }
}

#[derive(Debug, Clone, ValueEnum)]
pub enum Type {
    All,
    Int,
    Float,
    Boolean,
    Date,
    Datetime,
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::All => write!(f, "all"),
            Type::Int => write!(f, "int"),
            Type::Float => write!(f, "float"),
            Type::Boolean => write!(f, "boolean"),
            Type::Date => write!(f, "date"),
            Type::Datetime => write!(f, "datetime"),
        }
    }
}

impl std::str::FromStr for Type {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "all" => Ok(Type::All),
            "int" => Ok(Type::Int),
            "float" => Ok(Type::Float),
            "boolean" => Ok(Type::Boolean),
            "date" => Ok(Type::Date),
            "datetime" => Ok(Type::Datetime),
            _ => Err(format!("Unknown type: {s}")),
        }
    }
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum InferSchema {
    No,
    Fast,
    Safe,
}

impl InferSchema {
    pub fn to_csv_infer_schema_length(&self) -> Option<usize> {
        match self {
            InferSchema::No => Some(0),
            InferSchema::Fast => Some(128),
            InferSchema::Safe => Some(0),
        }
    }

    pub fn to_json_infer_schema_length(&self) -> Option<NonZero<usize>> {
        match self {
            InferSchema::No => None,
            InferSchema::Fast => Some(NonZero::new(128).unwrap()),
            InferSchema::Safe => None,
        }
    }
}
