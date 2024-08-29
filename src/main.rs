use clap::Parser;
use polars::frame::DataFrame;
use ratatui::backend::CrosstermBackend;
use std::io::{self};
use tabiew::app::App;
use tabiew::args::{AppTheme, Args};
use tabiew::handler::command::PresetCommands;
use tabiew::handler::event::{Event, EventHandler};
use tabiew::handler::keybind::Keybind;
use tabiew::reader::BuildReader;
use tabiew::sql::SqlBackend;
use tabiew::tui::{themes, Styler};
use tabiew::tui::{Tabular, TabularType, Terminal};
use tabiew::AppResult;

fn main() -> AppResult<()> {
    // Parse CLI
    let args = Args::parse();

    // Create the sql backend.
    let mut sql_backend = SqlBackend::new();

    // Loading files to data frames
    let reader = args.build_reader();
    let tabs = args
        .files
        .iter()
        .map(|path| {
            let name = path
                .file_stem()
                .expect("Invalid file name")
                .to_string_lossy()
                .into_owned();

            let df = reader
                .read_to_data_frame(path.clone())
                .unwrap_or_else(|err| panic!("{}", err));
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