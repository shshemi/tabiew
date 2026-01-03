use clap::{CommandFactory, Parser};
use indexmap::IndexMap;
use polars::frame::DataFrame;
use polars::prelude::Schema;
use ratatui::backend::CrosstermBackend;
use std::io::{self, IsTerminal};
use std::path::PathBuf;
use std::sync::Arc;
use tabiew::app::App;
use tabiew::args::Args;
use tabiew::handler::event::{Event, EventHandler};
use tabiew::handler::message::Message;
use tabiew::misc::config::config;
use tabiew::misc::globals::sql;
use tabiew::misc::osc52::flush_osc52_buffer;
use tabiew::misc::type_ext::UnwrapOrGracefulShutdown;
use tabiew::misc::type_inferer::TypeInferer;
use tabiew::reader::{BuildReader, Source};
use tabiew::tui::component::{Component, FocusState};
use tabiew::tui::pane::TableDescription;

use tabiew::tui::Pane;
use tabiew::{AppResult, tui};

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

    let _ = config().reload();

    let type_infer = TypeInferer::from_args(&args);

    // Dataframe loading
    let mut name_dfs = Vec::new();

    // Load multiparts to data frames
    let mut multiparts = IndexMap::<Arc<Schema>, (String, DataFrame)>::new();
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

    let _ = start_tui(name_dfs);
}

fn start_tui(tabs: Vec<(String, DataFrame)>) -> AppResult<()> {
    let tabs = tabs
        .into_iter()
        .map(|(name, df)| Pane::new(df, TableDescription::Table(name)))
        .collect();

    // Initialize the terminal user interface.
    let mut tui = tui::Terminal::new(
        ratatui::Terminal::new(CrosstermBackend::new(io::stdout()))?,
        EventHandler::new(100),
    );
    tui.init()?;

    // Initialize the app
    let mut app = App::new(tabs);

    // Main loop
    while app.running() {
        tui.draw(&mut app)?;
        flush_osc52_buffer();

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
            app.update(&action, FocusState::Focused);
        }
        flush_osc52_buffer();
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
