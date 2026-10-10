use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub sub_command: Option<SubCommand>,

    #[arg(help = "Path(s) to the file(s) to be opened.", required = false)]
    pub sources: Vec<String>,

    #[arg(long, help = "Paths to be opened and concatenated vertically.",
        num_args = 1..,
        required = false)]
    pub multiparts: Vec<String>,

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
        long,
        help = "Limits the number of rows read from the input. Applies to CSV/DSV/TSV, Parquet, JSON, JSON Lines, Arrow, and Avro files.",
        required = false
    )]
    pub max_rows: Option<usize>,
}

#[derive(Debug, Subcommand)]
pub enum SubCommand {
    #[clap(subcommand)]
    Ctl(CtlArgs),
}

#[derive(Debug, Subcommand)]
pub enum CtlArgs {
    #[command(about = "List running tabiew instances")]
    Ps,

    #[command(about = "Open a new tab in another tabiew instance with a SQL query")]
    Sql {
        #[arg(long, help = "Process ID", required = true)]
        pid: u32,
        #[arg(long, help = "Query", required = true)]
        query: String,
    },

    #[command(
        about = "Print the tables loaded in another tabiew instance as JSON",
        long_about = "Print the tables loaded in another tabiew instance as JSON.\n\n\
            For each table, the output includes its origin, row and column counts, \
            null count and estimated size, and per column the data type, null count, \
            and minimum and maximum values. Use `tw ctl ps` to find the process ID."
    )]
    Schema {
        #[arg(
            long,
            help = "Process ID of the target tabiew instance (see `tw ctl ps`)",
            required = true
        )]
        pid: u32,
    },

    #[command(
        about = "Load a file into another tabiew instance as a new table",
        long_about = "Load a file into another tabiew instance as a new table.\n\n\
            Choose the file format as a subcommand, then pass the target instance's \
            process ID and the file's path or URL, e.g.\n\n  \
            tw ctl import csv --pid 1234 data.csv --separator ';'\n\n\
            Run `tw ctl import <FORMAT> --help` to see the options for each format, \
            and `tw ctl ps` to find the process ID."
    )]
    Import {
        #[clap(subcommand)]
        args: CtlImportArgs,
    },
}

#[derive(Debug, Subcommand)]
pub enum CtlImportArgs {
    #[command(about = "Import a CSV/DSV file into another tabiew instance")]
    Csv {
        #[command(flatten)]
        common: CtlImportCommon,
        #[arg(
            long,
            help = "Character used as the field separator or delimiter.",
            default_value_t = ','
        )]
        separator: char,
        #[arg(long, help = "Character used to quote fields.", default_value_t = '"')]
        quote_char: char,
        #[arg(long, help = "Specifies if the input does not contain a header row.")]
        no_header: bool,
        #[arg(long, help = "Ignores parsing errors while loading.")]
        ignore_errors: bool,
        #[arg(long, help = "Truncate ragged lines while reading the file.")]
        truncate_ragged_lines: bool,
        #[arg(long, help = "Limits the number of rows read from the input.")]
        max_rows: Option<usize>,
    },
    #[command(about = "Import a Parquet file into another tabiew instance")]
    Parquet {
        #[command(flatten)]
        common: CtlImportCommon,
        #[arg(long, help = "Limits the number of rows read from the input.")]
        max_rows: Option<usize>,
    },
    #[command(about = "Import a JSON file into another tabiew instance")]
    Json {
        #[command(flatten)]
        common: CtlImportCommon,
        #[arg(long, help = "Ignores parsing errors while loading.")]
        ignore_errors: bool,
        #[arg(long, help = "Limits the number of rows read from the input.")]
        max_rows: Option<usize>,
    },
    #[command(about = "Import a JSON Lines file into another tabiew instance")]
    Jsonl {
        #[command(flatten)]
        common: CtlImportCommon,
        #[arg(long, help = "Ignores parsing errors while loading.")]
        ignore_errors: bool,
        #[arg(long, help = "Limits the number of rows read from the input.")]
        max_rows: Option<usize>,
    },
    #[command(about = "Import an Arrow IPC file into another tabiew instance")]
    Arrow {
        #[command(flatten)]
        common: CtlImportCommon,
        #[arg(long, help = "Limits the number of rows read from the input.")]
        max_rows: Option<usize>,
    },
    #[command(about = "Import an Avro file into another tabiew instance")]
    Avro {
        #[command(flatten)]
        common: CtlImportCommon,
        #[arg(long, help = "Limits the number of rows read from the input.")]
        max_rows: Option<usize>,
    },
    #[command(about = "Import a fixed-width (FWF) file into another tabiew instance")]
    Fwf {
        #[command(flatten)]
        common: CtlImportCommon,
        #[arg(
            long,
            help = "A comma-separated list of widths, which specifies the column widths.",
            default_value_t = String::default(),
        )]
        widths: String,
        #[arg(
            long,
            help = "Specifies the separator length.",
            default_value_t = 1_usize
        )]
        separator_length: usize,
        #[arg(long, help = "Sets strict column width restrictions.")]
        no_flexible_width: bool,
        #[arg(long, help = "Specifies if the input does not contain a header row.")]
        no_header: bool,
    },
    #[command(about = "Import a SQLite database into another tabiew instance")]
    Sqlite {
        #[command(flatten)]
        common: CtlImportCommon,
        #[arg(long, help = "Sets the key for sqlite (if required)")]
        sqlite_key: Option<String>,
    },
    #[command(about = "Import an Excel workbook into another tabiew instance")]
    Excel {
        #[command(flatten)]
        common: CtlImportCommon,
    },
    #[command(about = "Import a logfmt file into another tabiew instance")]
    Logfmt {
        #[command(flatten)]
        common: CtlImportCommon,
    },
    #[command(about = "Import tables from an HTML file into another tabiew instance")]
    Html {
        #[command(flatten)]
        common: CtlImportCommon,
    },
    #[command(about = "Import tables from a Markdown file into another tabiew instance")]
    Markdown {
        #[command(flatten)]
        common: CtlImportCommon,
    },
}

impl CtlImportArgs {
    pub fn pid(&self) -> u32 {
        match self {
            CtlImportArgs::Csv { common, .. }
            | CtlImportArgs::Parquet { common, .. }
            | CtlImportArgs::Json { common, .. }
            | CtlImportArgs::Jsonl { common, .. }
            | CtlImportArgs::Arrow { common, .. }
            | CtlImportArgs::Avro { common, .. }
            | CtlImportArgs::Fwf { common, .. }
            | CtlImportArgs::Sqlite { common, .. }
            | CtlImportArgs::Excel { common }
            | CtlImportArgs::Logfmt { common }
            | CtlImportArgs::Html { common }
            | CtlImportArgs::Markdown { common } => common.pid,
        }
    }
}

#[derive(Debug, clap::Args)]
pub struct CtlImportCommon {
    #[arg(long, help = "Process ID", required = true)]
    pub pid: u32,

    #[arg(help = "Path or URL of the file to be imported.", required = true)]
    pub source: String,
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
    Avro,
    Html,
    Markdown,
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
