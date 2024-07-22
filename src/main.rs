use clap::Parser;
use polars::frame::DataFrame;
use polars::io::csv::read::{CsvParseOptions, CsvReadOptions};
use polars::io::SerReader;
use ratatui::backend::CrosstermBackend;
use tabiew::keybind::Keybind;
use std::error::Error;
use std::io::{self};
use std::path::PathBuf;
use tabiew::app::status_bar::StatusBar;
use tabiew::app::tabular::{DataFrameReset, Tabular};
use tabiew::app::{App, AppResult};
use tabiew::args::{AppTheme, Args, InferSchema};
use tabiew::command::Commands;
use tabiew::event::{Event, EventHandler};
use tabiew::sql::SqlBackend;
use tabiew::theme::{Argonaut, Monokai, Terminal};
use tabiew::tui::Tui;
use tabiew::utils::{as_ascii, infer_schema_safe};

fn main() -> AppResult<()> {
    // Parse CLI
    let args = Args::parse();

    // Create the sql backend.
    let mut sql_backend = SqlBackend::new();

    // Instantiate app components
    let tabs = args.files.iter().map(|path|{
        let name = path.file_stem()
                .expect("Invalid file name")
                .to_string_lossy()
                .into_owned();
        let df = match read_csv(
                path.clone(),
                &args.infer_schema,
                args.quote_char,
                args.separator,
                args.no_header,
                args.ignore_errors,
            ){
                Ok(df) => df,
                Err(err) => panic!("{}", err),
            };
        let name = sql_backend.register( &name, df.clone(), path.clone());
        Tabular::new(df, DataFrameReset::Query(format!("SELECT * FROM {}", name)))
    }).collect();
    let status_bar = StatusBar::default();
    let exec_tbl = Commands::default().into_exec();
    let keybind = Keybind::default();
    let mut app = App::new(tabs, status_bar, sql_backend, exec_tbl, keybind);

    // Command handling

    // Initialize the terminal user interface.
    let mut tui = Tui::new(
        ratatui::Terminal::new(CrosstermBackend::new(io::stderr()))?,
        EventHandler::new(250),
    );
    tui.init()?;

    // Run the main loop
    while app.running() {
        match args.theme {
            AppTheme::Monokai => {
                tui.draw::<Monokai>(&mut app)?
            }
            AppTheme::Argonaut => {
                tui.draw::<Argonaut>(&mut app)?
            }
            AppTheme::Terminal => {
                tui.draw::<Terminal>(&mut app)?
            }
        }

        match tui.events.next()? {
            Event::Tick => {
                app.tick()?
            }
            Event::Key(key_event) => {
                #[cfg(target_os = "windows")]
                {
                    use crossterm::event::KeyEventKind;
                    if matches!(key_event.kind, KeyEventKind::Press | KeyEventKind::Repeat) {
                        app.handle_key_event(key_event)?
                    }
                }
                #[cfg(not(target_os = "windows"))]
                {
                    app.handle_key_event(key_event)?
                }
            }
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        }
    }

    // Exit the user interface.
    tui.exit()?;
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
