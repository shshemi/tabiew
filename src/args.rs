use clap::{Parser, ValueEnum};
use std::io::IsTerminal;
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

    #[arg(
        short = 'F',
        long,
        help = "Stream rows progressively from stdin instead of waiting for EOF. Only valid for line-oriented formats (csv, tsv, dsv, jsonl, logfmt, fwf).",
        default_value_t = false
    )]
    pub follow: bool,

    #[arg(
        long = "key",
        help = "Comma-separated 0-based column indexes that form the composite primary key for streamed upserts. Defaults to the first column. Requires --follow. Mutually exclusive with --no-key.",
        required = false,
        default_value_t = KeyColumns::default(),
    )]
    pub key: KeyColumns,

    #[arg(
        long = "no-key",
        help = "Append-only streaming: skip upsert, just insert every row. Requires --follow. Mutually exclusive with --key.",
        required = false,
        default_value_t = false,
    )]
    pub no_key: bool,

    #[arg(
        long,
        help = "Maximum rows to buffer before flushing a streaming batch to the UI.",
        required = false,
        default_value_t = 1000
    )]
    pub stream_batch_rows: usize,

    #[arg(
        long,
        help = "Maximum milliseconds to buffer before flushing a streaming batch to the UI.",
        required = false,
        default_value_t = 250
    )]
    pub stream_batch_ms: u64,

    #[arg(
        long = "flash-ms",
        help = "Duration in milliseconds for cell flash highlighting on upsert changes. Requires --follow and --key.",
        required = false,
        default_value_t = 750
    )]
    pub flash_ms: u64,

    #[arg(
        long = "no-flash",
        help = "Disable cell flash highlighting on upsert changes.",
        required = false,
        default_value_t = false,
    )]
    pub no_flash: bool,

    #[arg(
        long = "flash-color",
        help = "Color for update flash highlighting. Accepts: red, green, yellow, blue, magenta, cyan, white, or a hex color like '#FF8800'. Default: yellow.",
        required = false,
    )]
    pub flash_color: Option<String>,
}

impl Args {
    /// Validate cross-field constraints that clap cannot express directly.
    /// Returns a plain string error so this file remains self-contained
    /// (it is `include!`d by `build.rs` which has no access to crate items).
    pub fn validate(&self) -> Result<(), String> {
        if !self.key.is_default() && !self.follow {
            return Err("--key requires --follow".to_string());
        }
        if self.no_key && !self.follow {
            return Err("--no-key requires --follow".to_string());
        }
        if self.no_key && !self.key.is_default() {
            return Err("--no-key and --key are mutually exclusive".to_string());
        }
        if self.no_flash && !self.follow {
            return Err("--no-flash requires --follow".to_string());
        }
        if self.flash_ms != 750 && !self.follow {
            return Err("--flash-ms requires --follow".to_string());
        }
        if self.no_flash && self.flash_ms != 750 {
            return Err("--no-flash and --flash-ms are mutually exclusive".to_string());
        }
        if self.flash_color.is_some() && !self.follow {
            return Err("--flash-color requires --follow".to_string());
        }
        if self.flash_color.is_some() && self.no_flash {
            return Err("--flash-color and --no-flash are mutually exclusive".to_string());
        }
        if self.follow {
            if std::io::stdin().is_terminal() {
                return Err(
                    "--follow requires data to be piped on stdin (e.g. `tail -f file | tw --follow -f jsonl`)"
                        .to_string(),
                );
            }
            let resolved = self.format.clone().unwrap_or(Format::Csv);
            if !resolved.is_streamable() {
                return Err(format!(
                    "format `{}` cannot be streamed; --follow only works with csv, tsv, dsv, jsonl, logfmt, or fwf",
                    resolved.as_str()
                ));
            }
            if self.stream_batch_rows == 0 {
                return Err("--stream-batch-rows must be > 0".to_string());
            }
            if self.stream_batch_ms == 0 {
                return Err("--stream-batch-ms must be > 0".to_string());
            }
        }
        Ok(())
    }
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
    Logfmt,
}

impl Format {
    pub fn is_streamable(&self) -> bool {
        matches!(
            self,
            Format::Dsv | Format::Csv | Format::Tsv | Format::Jsonl | Format::Logfmt | Format::Fwf
        )
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Format::Dsv => "dsv",
            Format::Csv => "csv",
            Format::Tsv => "tsv",
            Format::Parquet => "parquet",
            Format::Jsonl => "jsonl",
            Format::Json => "json",
            Format::Arrow => "arrow",
            Format::Fwf => "fwf",
            Format::Sqlite => "sqlite",
            Format::Excel => "excel",
            Format::Logfmt => "logfmt",
        }
    }
}

/// Comma-separated 0-based column indexes used as a composite primary key
/// when upserting streamed rows. Mirrors the `TypeVec` newtype pattern below.
#[derive(Debug, Clone)]
pub struct KeyColumns {
    indexes: Vec<usize>,
    is_default: bool,
}

impl KeyColumns {
    pub fn indexes(&self) -> &[usize] {
        &self.indexes
    }

    pub fn is_default(&self) -> bool {
        self.is_default
    }
}

impl Default for KeyColumns {
    fn default() -> Self {
        Self {
            indexes: vec![0],
            is_default: true,
        }
    }
}

impl std::fmt::Display for KeyColumns {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let parts: Vec<String> = self.indexes.iter().map(|i| i.to_string()).collect();
        write!(f, "{}", parts.join(","))
    }
}

impl std::str::FromStr for KeyColumns {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let trimmed = s.trim();
        if trimmed.is_empty() {
            return Err("--key must contain at least one column index".to_string());
        }
        let mut indexes = Vec::new();
        for part in trimmed.split(',') {
            let p = part.trim();
            if p.is_empty() {
                return Err("--key entries must not be empty".to_string());
            }
            let idx: usize = p
                .parse()
                .map_err(|_| format!("--key index `{p}` is not a non-negative integer"))?;
            if indexes.contains(&idx) {
                return Err(format!("--key index `{idx}` is duplicated"));
            }
            indexes.push(idx);
        }
        let is_default = indexes == vec![0];
        Ok(KeyColumns {
            indexes,
            is_default,
        })
    }
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
