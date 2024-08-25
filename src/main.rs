use clap::Parser;
use polars::frame::DataFrame;
use polars::io::csv::read::{CsvParseOptions, CsvReadOptions};
use polars::io::parquet::read::ParquetReader;
use polars::io::SerReader;
use ratatui::backend::CrosstermBackend;
use std::fs::File;
use std::io::{self};
use std::path::PathBuf;
use tabiew::app::App;
use tabiew::args::{AppTheme, Args, Format, InferSchema};
use tabiew::handler::command::PresetCommands;
use tabiew::handler::event::{Event, EventHandler};
use tabiew::handler::keybind::Keybind;
use tabiew::sql::SqlBackend;
use tabiew::tui::{themes, Styler};
use tabiew::tui::{Tabular, TabularType, Terminal};
use tabiew::utils::{as_ascii, safe_infer_schema};
use tabiew::AppResult;

fn main() -> AppResult<()> {
    // Parse CLI
    let args = Args::parse();

    // Create the sql backend.
    let mut sql_backend = SqlBackend::new();

    // Instantiate app components
    let tabs = args
        .files
        .iter()
        .map(|path| {
            let name = path
                .file_stem()
                .expect("Invalid file name")
                .to_string_lossy()
                .into_owned();

            let df = match args.format {
                Format::Dsv => match read_csv(
                    path.clone(),
                    &args.infer_schema,
                    args.quote_char,
                    args.separator,
                    args.no_header,
                    args.ignore_errors,
                ) {
                    Ok(df) => df,
                    Err(err) => panic!("{}", err),
                },
                Format::Parquet => match read_parquet(path.clone()) {
                    Ok(df) => df,
                    Err(err) => panic!("{}", err),
                },
            };
            let name = sql_backend.register(&name, df.clone(), path.clone());
            (df, name)
        })
        .collect();

    match args.theme {
        AppTheme::Monokai => start_tui::<themes::Monokai>(tabs, sql_backend),
        AppTheme::Argonaut => start_tui::<themes::Argonaut>(tabs, sql_backend),
        AppTheme::Terminal => start_tui::<themes::Terminal>(tabs, sql_backend),
    }
}

fn start_tui<Theme: Styler>(
    tabs: Vec<(DataFrame, String)>,
    sql_backend: SqlBackend,
) -> AppResult<()> {
    let tabs = tabs
        .into_iter()
        .map(|(df, name)| Tabular::new(df, TabularType::Name(name)))
        .collect();
    let exec_tbl = PresetCommands::default().into_exec();
    let keybind = Keybind::default();
    let mut app = App::new(tabs, sql_backend, exec_tbl, keybind);

    // Initialize the terminal user interface.
    let mut tui = Terminal::new(
        ratatui::Terminal::new(CrosstermBackend::new(io::stderr()))?,
        EventHandler::new(250),
    );
    tui.init()?;

    // Run the main loop
    while app.running() {
        tui.draw::<Theme>(&mut app)?;

        match tui.events.next()? {
            Event::Tick => app.tick()?,
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
) -> AppResult<DataFrame> {
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
        safe_infer_schema(&mut df);
    }
    Ok(df)
}

fn read_parquet(path: PathBuf) -> AppResult<DataFrame> {
    Ok(ParquetReader::new(File::open(&path)?)
        .set_rechunk(true)
        .finish()?)
}
