use clap::{CommandFactory, Parser};
use indexmap::IndexMap;
use polars::frame::DataFrame;
use polars::prelude::Schema;
use std::io::IsTerminal;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::mpsc::sync_channel;
use tabiew::app::{App, StreamSink};
use tabiew::args::Args;
use tabiew::handler::event::{Event, read_event};
use tabiew::handler::message::Message;
use tabiew::misc::config::config;
use tabiew::misc::osc52::flush_osc52_buffer;
use tabiew::misc::sql::sql;
use tabiew::misc::type_ext::UnwrapOrGracefulShutdown;
use tabiew::misc::type_inferer::TypeInferer;
use tabiew::misc::upsert_index::UpsertIndex;
use tabiew::reader::{BuildReader, BuildStreamReader, Source};
use tabiew::tui::component::{Component, FocusState};
use tabiew::tui::pane::TableDescription;
use tabiew::tui::terminal::{draw, start_tui, stop_tui};

use ratatui::style::Color;
use tabiew::AppResult;
use tabiew::tui::Pane;

fn parse_color(s: Option<&str>) -> Color {
    match s {
        None => Color::Yellow,
        Some(name) => match name.to_lowercase().as_str() {
            "black" => Color::Black,
            "red" => Color::Red,
            "green" => Color::Green,
            "yellow" => Color::Yellow,
            "blue" => Color::Blue,
            "magenta" => Color::Magenta,
            "cyan" => Color::Cyan,
            "white" => Color::White,
            hex if hex.starts_with('#') && hex.len() == 7 => {
                let r = u8::from_str_radix(&hex[1..3], 16).unwrap_or(255);
                let g = u8::from_str_radix(&hex[3..5], 16).unwrap_or(255);
                let b = u8::from_str_radix(&hex[5..7], 16).unwrap_or(0);
                Color::Rgb(r, g, b)
            }
            _ => {
                eprintln!("Warning: unknown --flash-color '{}', using yellow", name);
                Color::Yellow
            }
        },
    }
}

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
    args.validate().unwrap_or_graceful_shutdown();

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
        df.rechunk_mut_par();
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

    let mut stream_sink: Option<StreamSink> = None;

    if name_dfs.is_empty() {
        if args.follow {
            // Streaming stdin path: build a streaming reader, spawn the
            // producer thread, and hand the receiver to the App so batches
            // are drained on each tick.
            let reader = args.build_stream_reader().unwrap_or_else(|| {
                eprintln!(
                    "Error: format is not streamable; this should have been caught by validate()"
                );
                std::process::exit(1);
            });
            let (tx, rx) = sync_channel(64);
            // Register the placeholder in the SQL context under a stable
            // name so the tab and future batch refreshes agree.
            let empty_df = DataFrame::empty();
            let table_name =
                sql().register(&Source::Stdin.table_name(), empty_df.clone(), Source::Stdin);
            reader.stream_to_data_frames(Source::Stdin, tx);
            name_dfs.push((table_name.clone(), empty_df));
            let upsert = if args.no_key {
                None
            } else {
                Some(UpsertIndex::new(args.key.indexes().to_vec()))
            };
            let flash_duration = if args.no_flash || args.no_key {
                std::time::Duration::ZERO
            } else {
                std::time::Duration::from_millis(args.flash_ms)
            };
            let flash_update_color = parse_color(args.flash_color.as_deref());
            stream_sink = Some(StreamSink::new(
                rx, 0, table_name, upsert, flash_duration, flash_update_color,
            ));
        } else {
            for (name, mut df) in args
                .build_reader("")
                .unwrap_or_graceful_shutdown()
                .read_to_data_frames(Source::Stdin)
                .unwrap_or_graceful_shutdown()
            {
                type_infer.update(&mut df);
                let name = sql().register(&name, df.clone(), Source::Stdin);
                name_dfs.push((name, df))
            }
        }
    }

    let _ = start_app(name_dfs, stream_sink);
}

fn start_app(tabs: Vec<(String, DataFrame)>, stream_sink: Option<StreamSink>) -> AppResult<()> {
    let tabs = tabs
        .into_iter()
        .map(|(name, df)| Pane::new(df, TableDescription::Table(name)))
        .collect();

    start_tui()?;

    // Initialize the app
    let mut app = App::new(tabs);
    if let Some(sink) = stream_sink {
        app = app.with_stream(sink);
    }

    // Main loop
    while app.running() {
        draw(&mut app)?;
        flush_osc52_buffer();

        match read_event()? {
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
    stop_tui()?;
    Ok(())
}

fn try_read_path(args: &Args, path: &PathBuf) -> AppResult<Box<[(String, DataFrame)]>> {
    let source = Source::File(path.clone());
    let reader = args.build_reader(path)?;
    reader.read_to_data_frames(source.clone())
}
