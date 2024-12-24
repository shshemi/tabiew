use clap::Parser;
use polars::frame::DataFrame;
use ratatui::backend::CrosstermBackend;
use std::fs::{self};
use std::io::{self};
use std::path::PathBuf;
use std::str::FromStr;
use tabiew::app::App;
use tabiew::args::{AppTheme, Args};
use tabiew::handler::command::parse_into_action;
use tabiew::handler::event::{Event, EventHandler};
use tabiew::handler::keybind::default_keymap;
use tabiew::reader::{BuildReader, Input};
use tabiew::sql::SqlBackend;
use tabiew::tui::{themes, Styler};
use tabiew::tui::{TabularState, TabularType, Terminal};
use tabiew::AppResult;

fn main() -> AppResult<()> {
    // Parse CLI
    let args = Args::parse();

    // Create the sql backend.
    let mut sql_backend = SqlBackend::new();

    // Load files to data frames
    let mut tabs = Vec::new();
    for path in args.files.iter() {
        let reader = args.build_reader(path)?;
        let frames = reader
            .named_frames(Input::File(path.to_path_buf()))
            .unwrap_or_else(|err| panic!("{}", err));
        for (name, df) in frames {
            let name = name.unwrap_or(
                path.file_stem()
                    .ok_or("Invalid file name")?
                    .to_string_lossy()
                    .into_owned(),
            );
            let name = sql_backend.register(&name, df.clone(), path.clone());
            tabs.push((df, name));
        }
    }
    if tabs.is_empty() {
        let reader = args.build_reader("stdin")?;
        let frames = reader.named_frames(Input::Stdin)?;
        for (_, df) in frames {
            let name = sql_backend.register("stdin", df.clone(), PathBuf::from_str("stdin")?);
            tabs.push((df, name));
        }
    }

    let script = if let Some(path) = args.script {
        fs::read_to_string(path).unwrap_or_else(|err| panic!("{}", err))
    } else {
        Default::default()
    };

    match args.theme {
        AppTheme::Monokai => start_tui::<themes::Monokai>(tabs, sql_backend, script),
        AppTheme::Argonaut => start_tui::<themes::Argonaut>(tabs, sql_backend, script),
        AppTheme::Nord => start_tui::<themes::Nord>(tabs, sql_backend, script),
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
        .map(|(df, name)| TabularState::new(df, TabularType::Name(name)))
        .collect();
    let keybind = default_keymap();
    let mut app = App::new(tabs, sql_backend, keybind);

    // Initialize the terminal user interface.
    let mut tui = Terminal::new(
        ratatui::Terminal::new(CrosstermBackend::new(io::stderr()))?,
        EventHandler::new(100),
    );
    tui.init()?;

    // Draw once before startup script
    tui.draw::<Theme>(&mut app)?;

    // Run startup script
    for line in script.lines().filter(|line| !line.is_empty()) {
        let action = parse_into_action(line)
            .unwrap_or_else(|err| panic!("Error in startup script: {}", err));
        app.invoke(action)
            .unwrap_or_else(|err| panic!("Error in startup script: {}", err));
    }

    // Main loop
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
