use clap::Parser;
use itertools::Itertools;
use polars::frame::DataFrame;
use ratatui::backend::CrosstermBackend;
use std::fs::{self, File};
use std::io::{self, Cursor, Read};
use std::path::PathBuf;
use std::str::FromStr;
use tabiew::app::App;
use tabiew::args::{AppTheme, Args};
use tabiew::handler::command::parse_into_action;
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
    let mut tabs = args
        .files
        .iter()
        .map(|path| {
            let reader = args.build_reader(path);
            let name = path
                .file_stem()
                .expect("Invalid file name")
                .to_string_lossy()
                .into_owned();

            let df = reader
                .read_to_data_frame(File::open(path).unwrap_or_else(|err| panic!("{}", err)))
                .unwrap_or_else(|err| panic!("{}", err));
            let name = sql_backend.register(&name, df.clone(), path.clone());
            (df, name)
        })
        .collect_vec();
    if tabs.is_empty() {
        let reader = args.build_reader("stdin");
        let mut buf = Vec::new();
        io::stdin().read_to_end(&mut buf)?;
        let df = reader.read_to_data_frame(Cursor::new(buf))?;
        let name = sql_backend.register("stdin", df.clone(), PathBuf::from_str("stdin")?);
        tabs.push((df, name));
    }

    let script = if let Some(path) = args.script {
        fs::read_to_string(path).unwrap_or_else(|err| panic!("{}", err))
    } else {
        Default::default()
    };

    match args.theme {
        AppTheme::Monokai => start_tui::<themes::Monokai>(tabs, sql_backend, script),
        AppTheme::Argonaut => start_tui::<themes::Argonaut>(tabs, sql_backend, script),
        AppTheme::Terminal => start_tui::<themes::Terminal>(tabs, sql_backend, script),
    }
}

fn start_tui<Theme: Styler>(
    tabs: Vec<(DataFrame, String)>,
    sql_backend: SqlBackend,
    script: String,
) -> AppResult<()> {
    let tabs = tabs
        .into_iter()
        .map(|(df, name)| Tabular::new(df, TabularType::Name(name)))
        .collect();
    let keybind = Keybind::default();
    let mut app = App::new(tabs, sql_backend, keybind);

    // Initialize the terminal user interface.
    let mut tui = Terminal::new(
        ratatui::Terminal::new(CrosstermBackend::new(io::stderr()))?,
        EventHandler::new(250),
    );
    tui.init()?;

    tui.draw::<Theme>(&mut app)?;
    for line in script.lines().filter(|line| !line.is_empty()) {
        let action = parse_into_action(line)
            .unwrap_or_else(|err| panic!("Error in startup script: {}", err));
        app.invoke(action)
            .unwrap_or_else(|err| panic!("Error in startup script: {}", err));
    }

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
