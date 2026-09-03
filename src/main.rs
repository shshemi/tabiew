use clap::{CommandFactory, Parser};
use indexmap::IndexMap;
use polars::frame::DataFrame;
use polars::prelude::Schema;
use std::io::IsTerminal;
use std::sync::Arc;
use tabiew::app::App;
use tabiew::args::Args;
use tabiew::handler::event::{Event, read_event};
use tabiew::handler::message::Message;
use tabiew::io::DataSource;
use tabiew::io::reader::ReaderSource;
use tabiew::io::reader::{BuildReader, NamedFrames};
use tabiew::misc::config;
use tabiew::misc::download::download_to_temp;
use tabiew::misc::osc52::flush_osc52_buffer;
use tabiew::misc::remote_load::Reader;
use tabiew::misc::sql::{TableSource, sql};
use tabiew::misc::type_ext::UnwrapOrGracefulShutdown;
use tabiew::misc::type_inferer::{TypeInferer, TypeInferredReader};
use tabiew::tui::component::Component;
use tabiew::tui::pane::TableDescription;
use tabiew::tui::terminal::{draw, start_tui, stop_tui};

use tabiew::AppResult;
use tabiew::tui::Pane;

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

    config::init().unwrap_or_graceful_shutdown();

    let type_infer = TypeInferer::from_args(&args);

    // Dataframe loading
    let mut name_dfs = Vec::new();

    // Load multiparts to data frames
    let mut multiparts = IndexMap::<Arc<Schema>, (String, DataFrame)>::new();
    for resource in args.multiparts.iter() {
        // Multipart tables merge several files, so no single reader can refresh them.
        let (frames, _) = try_read_path(&args, resource).unwrap_or_graceful_shutdown();
        for (name, new_df) in frames {
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
        let name = sql().register(&name, df.clone(), TableSource::File(name.clone().into()));
        name_dfs.push((name, df));
    }

    // Load files to data frames
    for resource in args.resources.iter() {
        let (frames, reader) = try_read_path(&args, resource).unwrap_or_graceful_shutdown();
        // The stored reader replays type inference too, so a refresh reproduces the frames of
        // the original load.
        let reader = reader
            .map(|reader| Arc::new(TypeInferredReader::new(reader, type_infer)) as Arc<dyn Reader>);
        for (name, mut df) in frames {
            type_infer.update(&mut df);
            let name = match reader.clone() {
                Some(reader) => {
                    sql().register_with_reader(&name, df.clone(), resource.clone(), reader)
                }
                None => sql().register(&name, df.clone(), resource.clone()),
            };
            name_dfs.push((name, df))
        }
    }

    if name_dfs.is_empty() {
        for (name, mut df) in args
            .build_reader("")
            .unwrap_or_graceful_shutdown()
            .read_to_data_frames(ReaderSource::Stdin)
            .unwrap_or_graceful_shutdown()
        {
            type_infer.update(&mut df);
            let name = sql().register(&name, df.clone(), TableSource::Stdin);
            name_dfs.push((name, df))
        }
    }

    start_tui().unwrap_or_graceful_shutdown();
    start_app(name_dfs).unwrap_or_graceful_shutdown();
    let _ = stop_tui();
}

fn start_app(tabs: Vec<(String, DataFrame)>) -> AppResult<()> {
    let tabs = tabs
        .into_iter()
        .map(|(name, df)| Pane::new(df, TableDescription::Table(name)))
        .collect();

    // Initialize the app
    let mut app = App::new(tabs);

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
                    use tabiew::tui::component::Component;

                    app.handle(key_event);
                }
            }
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
            Event::FocusGained => {}
            Event::FocusLost => {}
            Event::Paste(_) => {}
        }

        while let Some(action) = Message::dequeue() {
            app.update(&action);
        }
        flush_osc52_buffer();
    }

    // Exit the user interface.
    Ok(())
}

/// Reads a resource into data frames. For file resources the reader is returned as well, so it
/// can be stored to refresh the table later; stdin and downloaded URLs cannot be re-read.
fn try_read_path(
    args: &Args,
    resource: &DataSource,
) -> AppResult<(NamedFrames, Option<Arc<dyn Reader>>)> {
    match resource {
        DataSource::Stdin => Ok((
            args.build_reader("")?
                .read_to_data_frames(ReaderSource::Stdin)?,
            None,
        )),
        DataSource::File(path_buf) => {
            let reader: Arc<dyn Reader> = args.build_reader(path_buf)?.into();
            let frames = reader.read_to_data_frames(ReaderSource::File(path_buf.clone()))?;
            Ok((frames, Some(reader)))
        }
        DataSource::Url(url) => {
            let file = download_to_temp(url)?;
            Ok((
                args.build_reader(file.path())?
                    .read_to_data_frames(ReaderSource::File(file.path().to_owned()))?,
                None,
            ))
        }
    }
}
