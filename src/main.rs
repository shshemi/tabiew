use anyhow::anyhow;
use clap::{CommandFactory, Parser};
use polars::frame::DataFrame;
use polars::prelude::Schema;
use ratatui::backend::CrosstermBackend;
use std::fs::{self};
use std::io::{self, BufWriter, IsTerminal};
use std::path::PathBuf;
use std::sync::Arc;
use tabiew::app::App;
use tabiew::args::Args;
use tabiew::handler::event::{Event, EventHandler};
use tabiew::handler::message::Message;
use tabiew::misc::config::Config;
use tabiew::misc::globals::{config, sql};
use tabiew::misc::paths::{config_path, history_path, theme_path};
use tabiew::misc::type_ext::UnwrapOrGracefulShutdown;
use tabiew::misc::type_inferer::TypeInferer;
use tabiew::misc::vec_map::VecMap;
use tabiew::reader::{BuildReader, Source};
use tabiew::tui::component::Component;
use tabiew::tui::themes::custom::Custom;

use tabiew::tui::{Pane, TableType};
use tabiew::{AppResult, tui};

use tabiew::misc::history::{History, enforce_line_limit};

fn main() {
    // Parse CLI
    let args = {
        let args_os = std::env::args_os();
        // Show help message if no arguments are given and stdin is not piped
        if args_os.len() == 1 && std::io::stdin().is_terminal() {
            return Args::command().print_help().unwrap_or_graceful_shutdown();
        } else {
            Args::parse_from(args_os)
        }
    };

    // generate template file if needed
    for item in args.generate.iter() {
        let (name, path, contents) = match item {
            tabiew::args::GenerateItem::Config => (
                "config",
                config_path()
                    .ok_or(anyhow!("Home directory not found"))
                    .unwrap_or_graceful_shutdown(),
                toml::to_string(&Config::default()).unwrap_or_graceful_shutdown(),
            ),
            tabiew::args::GenerateItem::Theme => (
                "theme",
                theme_path()
                    .ok_or(anyhow!("Home directory not found"))
                    .unwrap_or_graceful_shutdown(),
                toml::to_string(&Custom::default()).unwrap_or_graceful_shutdown(),
            ),
        };
        fs::create_dir_all(
            path.parent()
                .ok_or(anyhow!("Unable to make config dir"))
                .unwrap_or_graceful_shutdown(),
        )
        .unwrap_or_graceful_shutdown();
        if path.exists() {
            println!(
                "{name} file already exists at {}, remove it first before retrying.",
                path.to_str()
                    .ok_or("Invalid path")
                    .unwrap_or_graceful_shutdown()
            )
        } else {
            fs::write(&path, contents).unwrap_or_graceful_shutdown();
            println!(
                "{name} generated at {}",
                path.to_str()
                    .ok_or("Invalid path")
                    .unwrap_or_graceful_shutdown()
            )
        }
    }

    // exit if any template generation
    if !args.generate.is_empty() {
        return;
    }

    if let Some(config_path) = config_path()
        && let Ok(text) = fs::read_to_string(config_path)
    {
        config().load(&text).unwrap_or_graceful_shutdown();
    }

    let type_infer = TypeInferer::from_args(&args);

    // Dataframe loading
    let mut name_dfs = Vec::new();

    // Load multiparts to data frames
    let mut multiparts = VecMap::<Arc<Schema>, (String, DataFrame)>::new();
    for path in args.multiparts.iter() {
        for (name, new_df) in try_read_path(&args, path).unwrap_or_graceful_shutdown() {
            let schema = new_df.schema().clone();
            if let Some((_, df)) = multiparts.get_mut(&schema) {
                df.vstack_mut_owned(new_df).unwrap_or_graceful_shutdown();
            } else {
                multiparts.insert(schema, (name, new_df));
            }
        }
    }
    for (_, (name, mut df)) in multiparts {
        df.as_single_chunk_par();
        type_infer.update(&mut df);
        let name = sql().register(&name, df.clone(), Source::File(name.clone().into()));
        name_dfs.push((name, df));
    }

    // Load files to data frames
    for path in args.files.iter() {
        for (name, mut df) in try_read_path(&args, path).unwrap_or_graceful_shutdown() {
            type_infer.update(&mut df);
            let name = sql().register(&name, df.clone(), Source::File(path.clone()));
            name_dfs.push((name, df))
        }
    }

    if name_dfs.is_empty() {
        for (name, mut df) in args
            .build_reader("")
            .unwrap_or_graceful_shutdown()
            .named_frames(Source::Stdin)
            .unwrap_or_graceful_shutdown()
        {
            type_infer.update(&mut df);
            let name = sql().register(&name, df.clone(), Source::Stdin);
            name_dfs.push((name, df))
        }
    }

    let script = args
        .script
        .map(fs::read_to_string)
        .transpose()
        .unwrap_or_graceful_shutdown()
        .unwrap_or_default();

    let history = history_path()
        .as_ref()
        .map(|path| History::from_file(path.clone()))
        .unwrap_or(History::in_memory());

    let _ = start_tui(name_dfs, script, history);
    if let Some(history_path) = history_path() {
        enforce_line_limit(history_path, 999);
    }
}

fn start_tui(tabs: Vec<(String, DataFrame)>, script: String, history: History) -> AppResult<()> {
    let tabs = tabs
        .into_iter()
        .map(|(name, df)| Pane::new(df, TableType::Name(name)))
        .collect();
    // let keybind = KeyHandler::default();
    let mut app = App::new(tabs, history);

    // Initialize the terminal user interface.
    let mut tui = tui::Terminal::new(
        ratatui::Terminal::new(CrosstermBackend::new(io::stdout()))?,
        EventHandler::new(100),
    );
    tui.init()?;

    // Draw once before startup script
    tui.draw(&mut app)?;

    // Run startup script
    // for line in script.lines().filter(|line| !line.is_empty()) {
    //     let action = parse_into_action(line).unwrap_or_graceful_shutdown();
    //     execute(action, &mut app).unwrap_or_graceful_shutdown();
    // }

    // Main loop
    while app.running() {
        tui.draw(&mut app)?;

        match tui.events.next()? {
            Event::Tick => app.tick(),
            Event::Key(key_event) => {
                #[cfg(target_os = "windows")]
                {
                    use crossterm::event::KeyEventKind;
                    if matches!(key_event.kind, KeyEventKind::Press | KeyEventKind::Repeat) {
                        app.handle(key_event);
                    }
                }
                #[cfg(not(target_os = "windows"))]
                {
                    app.handle(key_event);
                }
            }
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        }

        while let Some(action) = Message::dequeue() {
            app.update(&action);
        }
    }

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}

fn try_read_path(args: &Args, path: &PathBuf) -> AppResult<Box<[(String, DataFrame)]>> {
    let source = Source::File(path.clone());
    let reader = args.build_reader(path)?;
    reader.named_frames(source.clone())
}
