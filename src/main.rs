use clap::Parser;
use polars::frame::DataFrame;
use ratatui::backend::CrosstermBackend;
use std::fs::{self};
use std::io::{self};
use tabiew::app::App;
use tabiew::args::{AppTheme, Args};
use tabiew::handler::action::execute;
use tabiew::handler::command::parse_into_action;
use tabiew::handler::event::{Event, EventHandler};
use tabiew::handler::key::KeyHandler;
use tabiew::misc::config::set_theme;
use tabiew::misc::sql::SqlBackend;
use tabiew::reader::{BuildReader, Input};
use tabiew::tui::tabs::Tab;
use tabiew::tui::theme::{Argonaut, Catppuccin, Monokai, Nord, Terminal, TokyoNight};
use tabiew::tui::{Source, TabularState};
use tabiew::{AppResult, tui};

use tabiew::misc::history::{History, enforce_line_limit};

fn main() -> AppResult<()> {
    // Parse CLI
    let args = Args::parse();

    // Create the sql backend.
    let mut sql_backend = SqlBackend::new();

    // Load files to data frames
    let tabs = if args.files.is_empty() {
        let mut vec = Vec::new();
        for (name, df) in args.build_reader("")?.named_frames(Input::Stdin)? {
            vec.push((df.clone(), sql_backend.register(&name, df, "stdin".into())))
        }
        vec
    } else {
        let mut vec = Vec::new();
        for path in args.files.iter() {
            let reader = args.build_reader(path)?;
            let frames = reader
                .named_frames(Input::File(path.to_path_buf()))
                .unwrap_or_else(|err| panic!("{}", err));
            for (name, df) in frames {
                let name = sql_backend.register(&name, df.clone(), path.clone());
                vec.push((df, name))
            }
        }
        vec
    };
    let script = args
        .script
        .map(fs::read_to_string)
        .transpose()?
        .unwrap_or_default();

    let history_path = home::home_dir().map(|path| path.join(".tabiew_history"));
    let history = history_path
        .as_ref()
        .map(|path| History::from_file(path.clone()))
        .unwrap_or(History::in_memory());

    set_theme(match args.theme {
        AppTheme::Monokai => Box::new(Monokai),
        AppTheme::Argonaut => Box::new(Argonaut),
        AppTheme::Nord => Box::new(Nord),
        AppTheme::Catppuccin => Box::new(Catppuccin),
        AppTheme::TokyoNight => Box::new(TokyoNight),
        AppTheme::Terminal => Box::new(Terminal),
    });

    let _ = start_tui(tabs, sql_backend, script, history);
    if let Some(history_path) = history_path {
        enforce_line_limit(history_path, 999);
    }
    Ok(())
}

fn start_tui(
    tabs: Vec<(DataFrame, String)>,
    mut sql_backend: SqlBackend,
    script: String,
    history: History,
) -> AppResult<()> {
    let tabs = tabs
        .into_iter()
        .map(|(df, name)| TabularState::new(df, Source::Name(name)))
        .collect();
    let keybind = KeyHandler::default();
    let mut app = App::new(tabs, history);

    // Set default data frame to the first tab
    sql_backend.set_default(
        app.tabs_mut()
            .selected()
            .and_then(Tab::tabular)
            .map(|tab| tab.table().data_frame().clone())
            .unwrap(),
    );

    // Initialize the terminal user interface.
    let mut tui = tui::Terminal::new(
        ratatui::Terminal::new(CrosstermBackend::new(io::stderr()))?,
        EventHandler::new(100),
    );
    tui.init()?;

    // Draw once before startup script
    tui.draw(&mut app)?;

    // Run startup script
    for line in script.lines().filter(|line| !line.is_empty()) {
        let action = parse_into_action(line)
            .unwrap_or_else(|err| panic!("Error in startup script: {}", err));
        execute(action, &mut app, &mut sql_backend)
            .unwrap_or_else(|err| panic!("Error in startup script: {}", err));
    }

    // Main loop
    while app.running() {
        tui.draw(&mut app)?;

        match tui.events.next()? {
            Event::Tick => app.tick()?,
            Event::Key(key_event) => {
                #[cfg(target_os = "windows")]
                {
                    use crossterm::event::KeyEventKind;
                    if matches!(key_event.kind, KeyEventKind::Press | KeyEventKind::Repeat) {
                        let mut action = keybind.action(app.context(), key_event);
                        loop {
                            match execute(action, &mut app, &mut sql_backend) {
                                Ok(Some(next_action)) => action = next_action,
                                Ok(_) => break,
                                Err(err) => {
                                    app.error(err);
                                    break;
                                }
                            }
                        }
                    }
                }
                #[cfg(not(target_os = "windows"))]
                {
                    let mut action = keybind.action(app.context(), key_event);
                    loop {
                        match execute(action, &mut app, &mut sql_backend) {
                            Ok(Some(next_action)) => action = next_action,
                            Ok(_) => break,
                            Err(err) => {
                                app.error(err);
                                break;
                            }
                        }
                    }
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
