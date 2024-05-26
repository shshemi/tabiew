use clap::Parser;
use polars::io::csv::CsvReader;
use polars::io::SerReader;
use polars::lazy::frame::IntoLazy;
use polars_sql::SQLContext;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::fmt::Display;
use std::io;
use std::str::FromStr;
use tabiew::app::{AppResult, StatusBar, Table};
use tabiew::command::CommandList;
use tabiew::event::{Event, EventHandler};
use tabiew::handler::handle_key_events;
use tabiew::tui::Tui;
use tabiew::utils::column_type_brute_foce;

fn main() -> AppResult<()> {
    // Parse CLI
    let args = Args::parse();

    // Create the data frame.
    let data_frame = {
        match args.infer_schema {
            InferSchema::No => CsvReader::from_path(&args.file_name)?
                .with_ignore_errors(args.ignore_errors)
                .infer_schema(0.into())
                .has_header(!args.no_header)
                .with_quote_char((args.quote_char as u8).into())
                .with_separator(args.separator as u8)
                .finish()?,
            InferSchema::Fast => CsvReader::from_path(&args.file_name)?
                .with_ignore_errors(args.ignore_errors)
                .has_header(!args.no_header)
                .with_quote_char((args.quote_char as u8).into())
                .with_separator(args.separator as u8)
                .finish()?,
            InferSchema::Full => CsvReader::from_path(&args.file_name)?
                .with_ignore_errors(args.ignore_errors)
                .infer_schema(None)
                .has_header(!args.no_header)
                .with_quote_char((args.quote_char as u8).into())
                .with_separator(args.separator as u8)
                .finish()?,
            InferSchema::Safe => {
                let mut df = CsvReader::from_path(&args.file_name)?
                    .with_ignore_errors(args.ignore_errors)
                    .infer_schema(0.into())
                    .has_header(!args.no_header)
                    .with_quote_char((args.quote_char as u8).into())
                    .with_separator(args.separator as u8)
                    .finish()?;
                column_type_brute_foce(&mut df);
                df
            }
        }
    };

    // Setup the SQLContext
    let mut sql_context = SQLContext::new();
    sql_context.register("df", data_frame.clone().lazy());

    // Instantiate app
    let mut tabular = Table::new(data_frame);
    let mut status_bar = StatusBar::default();

    // Running variable.
    let mut running = true;

    // Command handling
    let exec_tbl = CommandList::default().into_exec();

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.init()?;

    // Start the main loop.
    while running {
        // Render the user interface.
        tui.draw(&mut tabular, &mut status_bar)?;
        // Handle events.
        match tui.events.next()? {
            Event::Tick => {
                tabular.tick();
                status_bar.tick();
            }
            Event::Key(key_event) => handle_key_events(
                key_event,
                &mut tabular,
                &mut status_bar,
                &mut sql_context,
                &mut running,
                &exec_tbl,
            )?,
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        }
    }

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(help = "File to open", required = true)]
    file_name: String,

    #[arg(
        long,
        help = "CSV file contains no header row",
        default_value_t = false
    )]
    no_header: bool,

    #[arg(
        long,
        help = "Ignore parsing errors while loading the CSV",
        default_value_t = false
    )]
    ignore_errors: bool,

    #[arg(
        long,
        help = "Schema inference method {no, fast, full, safe}.",
        required = false,
        default_value_t = InferSchema::Fast,
    )]
    infer_schema: InferSchema,

    #[arg(
        long,
        help = "The field separator or delimiter",
        required = false,
        default_value_t = ','
    )]
    separator: char,

    #[arg(
        long,
        help = "Quote character",
        required = false,
        default_value_t = '"'
    )]
    quote_char: char,
}

#[derive(Debug, Clone)]
enum InferSchema {
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
