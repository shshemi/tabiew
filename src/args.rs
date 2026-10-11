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
        #[arg(help = "Query", required = true)]
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
        about = "Print the open tabs of another tabiew instance as JSON",
        long_about = "Print the open tabs of another tabiew instance as JSON.\n\n\
            For each tab, the output lists its stack of tables, from the original table \
            to the current view. Each table includes its columns and data types, the \
            first visible row, the number of rows on screen, and the step that produced \
            it (e.g. a query or filter). Use `tw ctl ps` to find the process ID."
    )]
    Tabs {
        #[arg(
            long,
            help = "Process ID of the target tabiew instance (see `tw ctl ps`)",
            required = true
        )]
        pid: u32,
    },

    #[command(
        about = "Switch the selected tab of another tabiew instance",
        long_about = "Switch the selected tab of another tabiew instance.\n\n\
            Tabs are numbered from 0, in the order `tw ctl tabs` lists them. \
            Use `tw ctl ps` to find the process ID."
    )]
    TabSwitch {
        #[arg(
            long,
            help = "Process ID of the target tabiew instance (see `tw ctl ps`)",
            required = true
        )]
        pid: u32,
        #[arg(
            long,
            help = "Index of the tab to select, starting from 0",
            required = true
        )]
        tab_idx: usize,
    },

    #[command(
        about = "Close a tab of another tabiew instance",
        long_about = "Close a tab of another tabiew instance.\n\n\
            Tabs are numbered from 0, in the order `tw ctl tabs` lists them. Unlike \
            closing a tab from the instance itself, the last remaining tab can also be \
            closed, which leaves the instance open with no tabs. \
            Use `tw ctl ps` to find the process ID."
    )]
    RmTab {
        #[arg(
            long,
            help = "Process ID of the target tabiew instance (see `tw ctl ps`)",
            required = true
        )]
        pid: u32,
        #[arg(
            long,
            help = "Index of the tab to close, starting from 0",
            required = true
        )]
        tab_idx: usize,
    },

    #[command(
        about = "Select a row in the current table of another tabiew instance",
        long_about = "Select a row in the current table of another tabiew instance.\n\n\
            Moves the cursor of the selected tab's current table to the given row, \
            scrolling it into view. Rows are numbered from 0, so the first row is 0 \
            (unlike the go-to-line popup, which starts at 1). \
            Use `tw ctl ps` to find the process ID."
    )]
    RowSelect {
        #[arg(
            long,
            help = "Process ID of the target tabiew instance (see `tw ctl ps`)",
            required = true
        )]
        pid: u32,
        #[arg(
            long,
            help = "Index of the row to select, starting from 0",
            required = true
        )]
        row_idx: usize,
    },

    #[command(
        about = "Filter the current table of another tabiew instance",
        long_about = "Filter the current table of another tabiew instance.\n\n\
            Works like the inline filter: the condition is used as the WHERE clause \
            of `SELECT * FROM _ WHERE <FILTER>` on the selected tab's current table, \
            and the result is added as a new view on that tab, e.g.\n\n  \
            tw ctl filter --pid 1234 \"price > 5000000 AND bedrooms >= 3\"\n\n\
            Use `tw ctl ps` to find the process ID."
    )]
    Filter {
        #[arg(
            long,
            help = "Process ID of the target tabiew instance (see `tw ctl ps`)",
            required = true
        )]
        pid: u32,
        #[arg(
            help = "SQL condition to filter rows by (the WHERE clause)",
            required = true
        )]
        filter: String,
    },

    #[command(
        about = "Select columns from the current table of another tabiew instance",
        long_about = "Select columns from the current table of another tabiew instance.\n\n\
            Works like the inline column selection: the columns are used as the select \
            list of `SELECT <COLUMNS> FROM _` on the selected tab's current table, and \
            the result is added as a new view on that tab. Expressions and aliases are \
            allowed, e.g.\n\n  \
            tw ctl select --pid 1234 \"price, area, price / area AS price_per_area\"\n\n\
            Use `tw ctl ps` to find the process ID."
    )]
    Select {
        #[arg(
            long,
            help = "Process ID of the target tabiew instance (see `tw ctl ps`)",
            required = true
        )]
        pid: u32,
        #[arg(
            help = "Comma-separated columns or expressions to select",
            required = true
        )]
        columns: String,
    },

    #[command(
        about = "Sort the current table of another tabiew instance",
        long_about = "Sort the current table of another tabiew instance.\n\n\
            Works like the inline ordering: the columns are used as the ORDER BY clause \
            of `SELECT * FROM _ ORDER BY <ORDER_BY>` on the selected tab's current table, \
            and the result is added as a new view on that tab. Add DESC to sort a column \
            in descending order, e.g.\n\n  \
            tw ctl order --pid 1234 \"price DESC, area\"\n\n\
            Use `tw ctl ps` to find the process ID."
    )]
    Order {
        #[arg(
            long,
            help = "Process ID of the target tabiew instance (see `tw ctl ps`)",
            required = true
        )]
        pid: u32,
        #[arg(
            help = "Comma-separated columns to sort by (the ORDER BY clause)",
            required = true
        )]
        order_by: String,
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

    #[command(
        about = "Save the current table of another tabiew instance to a file",
        long_about = "Save the current table of another tabiew instance to a file.\n\n\
            Writes the selected tab's current table, including any filter, ordering or \
            column selection applied to it. Choose the file format as a subcommand, then \
            pass the target instance's process ID and the output path, e.g.\n\n  \
            tw ctl export csv --pid 1234 out.csv --separator ';'\n\n\
            Relative paths are resolved against the current directory of this command, \
            not the target instance. Run `tw ctl export <FORMAT> --help` to see the \
            options for each format, and `tw ctl ps` to find the process ID."
    )]
    Export {
        #[clap(subcommand)]
        args: CtlExportArgs,
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

#[derive(Debug, Subcommand)]
pub enum CtlExportArgs {
    #[command(about = "Save as a CSV/DSV file")]
    Csv {
        #[command(flatten)]
        common: CtlExportCommon,
        #[arg(
            long,
            help = "Character used as the field separator or delimiter.",
            default_value_t = ','
        )]
        separator: char,
        #[arg(long, help = "Character used to quote fields.", default_value_t = '"')]
        quote_char: char,
        #[arg(long, help = "Do not write a header row.")]
        no_header: bool,
    },
    #[command(about = "Save as a TSV file")]
    Tsv {
        #[command(flatten)]
        common: CtlExportCommon,
        #[arg(long, help = "Character used to quote fields.", default_value_t = '"')]
        quote_char: char,
        #[arg(long, help = "Do not write a header row.")]
        no_header: bool,
    },
    #[command(about = "Save as a JSON file")]
    Json {
        #[command(flatten)]
        common: CtlExportCommon,
    },
    #[command(about = "Save as a JSON Lines file")]
    Jsonl {
        #[command(flatten)]
        common: CtlExportCommon,
    },
    #[command(about = "Save as a Parquet file")]
    Parquet {
        #[command(flatten)]
        common: CtlExportCommon,
    },
    #[command(about = "Save as an Arrow IPC file")]
    Arrow {
        #[command(flatten)]
        common: CtlExportCommon,
    },
    #[command(about = "Save as an Avro file")]
    Avro {
        #[command(flatten)]
        common: CtlExportCommon,
    },
    #[command(about = "Save as a Markdown table")]
    Markdown {
        #[command(flatten)]
        common: CtlExportCommon,
    },
}

impl CtlExportArgs {
    pub fn pid(&self) -> u32 {
        match self {
            CtlExportArgs::Csv { common, .. }
            | CtlExportArgs::Tsv { common, .. }
            | CtlExportArgs::Json { common }
            | CtlExportArgs::Jsonl { common }
            | CtlExportArgs::Parquet { common }
            | CtlExportArgs::Arrow { common }
            | CtlExportArgs::Avro { common }
            | CtlExportArgs::Markdown { common } => common.pid,
        }
    }
}

#[derive(Debug, clap::Args)]
pub struct CtlExportCommon {
    #[arg(
        long,
        help = "Process ID of the target tabiew instance (see `tw ctl ps`)",
        required = true
    )]
    pub pid: u32,

    #[arg(help = "Path of the file to write.", required = true)]
    pub path: std::path::PathBuf,
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
