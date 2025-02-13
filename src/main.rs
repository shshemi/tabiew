use anyhow::anyhow;
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
use tabiew::reader::{BuildReader, Input};
use tabiew::sql::SqlBackend;
use tabiew::tui::{themes, Styler};
use tabiew::tui::{Source, TabContentState, Terminal};
use tabiew::utils::history::{enforce_line_limit, History};
use tabiew::AppResult;

fn main() -> AppResult<()> {
    // Parse CLI
    let args = Args::parse();

    // Create the sql backend.
    let mut sql_backend = SqlBackend::new();

    // Load files to data frames
    let tabs = if args.files.is_empty() {
        let mut vec = Vec::new();
        for (name, df) in args.build_reader("")?.named_frames(Input::Stdin)? {
            vec.push((
                df.clone(),
                sql_backend.register(&name.unwrap_or("stdin".to_owned()), df, "stdin".into()),
            ))
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
                let name = sql_backend.register(
                    &name.unwrap_or(
                        path.file_stem()
                            .ok_or(anyhow!("Invalid file name"))?
                            .to_string_lossy()
                            .into_owned(),
                    ),
                    df.clone(),
                    path.clone(),
                );
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

    match args.theme {
        AppTheme::Monokai => start_tui::<themes::Monokai>(tabs, sql_backend, script, history),
        AppTheme::Argonaut => start_tui::<themes::Argonaut>(tabs, sql_backend, script, history),
        AppTheme::Nord => start_tui::<themes::Nord>(tabs, sql_backend, script, history),
        AppTheme::Catppuccin => start_tui::<themes::Catppuccin>(tabs, sql_backend, script, history),
        AppTheme::TokyoNight => start_tui::<themes::TokyoNight>(tabs, sql_backend, script, history),
        AppTheme::Terminal => start_tui::<themes::Terminal>(tabs, sql_backend, script, history),
    }?;
    if let Some(history_path) = history_path {
        enforce_line_limit(history_path, 999);
    }
    Ok(())
}

fn start_tui<Theme: Styler>(
    tabs: Vec<(DataFrame, String)>,
    mut sql_backend: SqlBackend,
    script: String,
    history: History,
) -> AppResult<()> {
    let tabs = tabs
        .into_iter()
        .map(|(df, name)| TabContentState::new(df, Source::Name(name)))
        .collect();
    let keybind = KeyHandler::default();
    let mut app = App::new(tabs, history);

    // Set default data frame to the first tab
    sql_backend.set_default(
        app.tabs()
            .selected()
            .map(|tab| tab.data_frame().clone())
            .unwrap(),
    );

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
        execute(action, &mut app, &mut sql_backend)
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
