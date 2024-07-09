use clap::Parser;
use polars::frame::DataFrame;
use polars::io::csv::read::{CsvParseOptions, CsvReadOptions};
use polars::io::SerReader;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::error::Error;
use std::io::{self, Stderr};
use std::path::PathBuf;
use tabiew::app::{AppResult, StatusBar, Tabular};
use tabiew::args::{Args, InferSchema};
use tabiew::command::{CommandList, ExecutionTable};
use tabiew::event::{Event, EventHandler};
use tabiew::handler::handle_key_events;
use tabiew::sql::SqlBackend;
use tabiew::theme::Styler;
use tabiew::tui::Tui;
use tabiew::utils::{as_ascii, infer_schema_safe};

fn main() -> AppResult<()> {
    // Parse CLI
    let args = Args::parse();

    // Create the sql backend.
    let mut sql_backend = SqlBackend::new();

    // Add csv files to sql backend
    for file in args.files {
        sql_backend.register(
            file.file_stem()
                .expect("Invalid file name")
                .to_string_lossy()
                .into_owned()
                .as_str(),
            read_csv(
                file.clone(),
                &args.infer_schema,
                args.quote_char,
                args.separator,
                args.no_header,
                args.ignore_errors,
            )?,
            file,
        );
    }

    // Instantiate app components
    let tabular = Tabular::new(sql_backend.default_df().expect("Default dataframe not found"));
    let status_bar = StatusBar::default();

    // Command handling
    let exec_tbl = CommandList::default().into_exec();

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stderr());
    let terminal: Terminal<CrosstermBackend<Stderr>> = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.init()?;

    // Run the main loop
    match args.theme {
        tabiew::args::AppTheme::Monokai => main_loop::<tabiew::theme::Monokai>(
            &mut tui,
            tabular,
            status_bar,
            sql_backend,
            exec_tbl,
        )?,
        tabiew::args::AppTheme::Argonaut => main_loop::<tabiew::theme::Argonaut>(
            &mut tui,
            tabular,
            status_bar,
            sql_backend,
            exec_tbl,
        )?,
        tabiew::args::AppTheme::Terminal => main_loop::<tabiew::theme::Terminal>(
            &mut tui,
            tabular,
            status_bar,
            sql_backend,
            exec_tbl,
        )?,
    }

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}

fn main_loop<Theme: Styler>(
    tui: &mut Tui<CrosstermBackend<Stderr>>,
    mut tabular: Tabular,
    mut status_bar: StatusBar,
    mut sql_context: SqlBackend,
    exec_tbl: ExecutionTable,
) -> AppResult<()> {
    let mut running = true;

    // Start the main loop.
    while running {
        // Render the user interface.
        tui.draw::<Theme>(&mut tabular, &mut status_bar)?;
        // Handle events.
        match tui.events.next()? {
            Event::Tick => {
                tabular.tick();
                status_bar.tick();
            }
            Event::Key(key_event) => {
                #[cfg(target_os = "windows")]
                {
                    use crossterm::event::KeyEventKind;
                    if matches!(key_event.kind, KeyEventKind::Press | KeyEventKind::Repeat) {
                        handle_key_events(
                            key_event,
                            &mut tabular,
                            &mut status_bar,
                            &mut sql_context,
                            &mut running,
                            &exec_tbl,
                        )?
                    }
                }
                #[cfg(not(target_os = "windows"))]
                {
                    handle_key_events(
                        key_event,
                        &mut tabular,
                        &mut status_bar,
                        &mut sql_context,
                        &mut running,
                        &exec_tbl,
                    )?
                }
            }
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        }
    }
    Ok(())
}

fn read_csv(
    path: PathBuf,
    infer_schema: &InferSchema,
    quote_char: char,
    separator_char: char,
    no_header: bool,
    ignore_errors: bool,
) -> Result<DataFrame, Box<dyn Error>> {
    let mut df = CsvReadOptions::default()
        .with_ignore_errors(ignore_errors)
        .with_infer_schema_length(infer_schema.into())
        .with_has_header(!no_header)
        .with_parse_options(
            CsvParseOptions::default()
                .with_quote_char(as_ascii(quote_char))
                .with_separator(as_ascii(separator_char).expect("Invalid separator")),
        )
        .try_into_reader_with_file_path(path.into())?
        .finish()?;
    if matches!(infer_schema, InferSchema::Safe) {
        infer_schema_safe(&mut df);
    }
    Ok(df)
}
