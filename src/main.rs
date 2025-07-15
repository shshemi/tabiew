use anyhow::anyhow;
use clap::Parser;
use polars::frame::DataFrame;
use ratatui::backend::CrosstermBackend;
use std::fs::{self};
use std::io::{self};
use std::path::PathBuf;
use tabiew::app::App;
use tabiew::args::{AppTheme, Args};
use tabiew::handler::action::execute;
use tabiew::handler::command::parse_into_action;
use tabiew::handler::event::{Event, EventHandler};
use tabiew::handler::key::KeyHandler;
use tabiew::misc::globals::{set_theme, sql};
use tabiew::misc::type_inferer::TypeInferer;
use tabiew::reader::{BuildReader, Source};

use tabiew::tui::theme::{
    Argonaut, Catppuccin, Chakra, Monokai, Nord, Terminal, Theme, TokyoNight,
};
use tabiew::tui::{TabContentState, TableType};
use tabiew::{AppResult, tui};

use tabiew::misc::history::{History, enforce_line_limit};

fn main() -> AppResult<()> {
    // Parse CLI
    let args = Args::parse();

    if args.generate_theme {
        let path = theme_path().ok_or(anyhow!("Home directory not found"))?;
        let _ = fs::create_dir_all(path.parent().ok_or(anyhow!("Unable to make parent dir"))?);
        if path.exists() {
            println!(
                "Theme file already exists at ~/.config/tabiew/theme.toml, remove it first before retrying.",
            )
        } else {
            let contents = toml::to_string(&Theme::sample())?;
            fs::write(&path, contents)?;
            println!("Theme generated at ~/.config/tabiew/theme.toml")
        }
        return Ok(());
    }

    let type_infer = TypeInferer::from_args(&args);

    // Load files to data frames
    let tabs = if args.files.is_empty() {
        let mut vec = Vec::new();
        for (name, mut df) in args.build_reader("")?.named_frames(Source::Stdin)? {
            type_infer.update(&mut df);
            vec.push((df.clone(), sql().register(&name, df, Source::Stdin)))
        }
        vec
    } else {
        let mut vec = Vec::new();
        for path in args.files.iter() {
            let source = Source::File(path.clone());
            let reader = args.build_reader(path)?;
            let frames = reader.named_frames(source.clone()).unwrap_or_else(|err| {
                eprintln!("tw: {err}");
                std::process::exit(1)
            });
            for (name, mut df) in frames {
                type_infer.update(&mut df);
                let name = sql().register(&name, df.clone(), source.clone());
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

    let history = history_path()
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
        AppTheme::Chakra => Box::new(Chakra),
        AppTheme::Config => {
            let theme: Theme = toml::from_str(
                &fs::read_to_string(theme_path().ok_or(anyhow!("Home directory not found"))?)
                    .map_err(|_| anyhow!("Create the theme at ~/.config/tabiew/theme.toml first or use --generate-theme to generate one."))?,
            )?;
            Box::new(theme)
        }
    });

    let _ = start_tui(tabs, script, history);
    if let Some(history_path) = history_path() {
        enforce_line_limit(history_path, 999);
    }
    Ok(())
}

fn start_tui(tabs: Vec<(DataFrame, String)>, script: String, history: History) -> AppResult<()> {
    let tabs = tabs
        .into_iter()
        .map(|(df, name)| TabContentState::new(df, TableType::Name(name)))
        .collect();
    let keybind = KeyHandler::default();
    let mut app = App::new(tabs, history);

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
        let action = parse_into_action(line).unwrap_or_else(|err| {
            eprintln!("tw: Error in startup script: {err}");
            std::process::exit(1);
        });
        execute(action, &mut app).unwrap_or_else(|err| {
            eprintln!("tw: Error in startup script: {err}");
            std::process::exit(1);
        });
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
                            match execute(action, &mut app) {
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
                        match execute(action, &mut app) {
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

fn history_path() -> Option<PathBuf> {
    home::home_dir().map(|path| path.join(".tabiew_history"))
}

fn theme_path() -> Option<PathBuf> {
    home::home_dir().map(|path| path.join(".config").join("tabiew").join("theme.toml"))
}
