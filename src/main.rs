use clap::{CommandFactory, Parser};
use indexmap::IndexMap;
use polars::frame::DataFrame;
use polars::prelude::Schema;
use std::io::IsTerminal;
use std::path::Path;
use std::str::FromStr;
use std::sync::Arc;
use tabiew::app::App;
use tabiew::args::{Args, CtlArgs, Format};
use tabiew::handler::event::{Event, read_event};
use tabiew::handler::message::Message;
use tabiew::misc::ipc::ops::{fetch_other_process, fetch_sql_backend};
use tabiew::misc::osc52::flush_osc52_buffer;
use tabiew::misc::sql::{TableSource, sql};
use tabiew::misc::unwrap_or_graceful_shutdown::UnwrapOrGracefulShutdown;
use tabiew::misc::{config, ipc};
use tabiew::net::downloader::download_to_temp;
use tabiew::parsers::data_frame_parser::DataFrameParser;
use tabiew::readers::DataSource;
use tabiew::readers::ReaderSource;
use tabiew::readers::{
    ArrowIpcToDataFrame, AvroToDataFrame, CsvToDataFrame, DataFrameReader, ExcelToDataFrames,
    FwfToDataFrame, HtmlToDataFrame, JsonLineToDataFrame, JsonToDataFrame, LogfmtToDataFrame,
    MarkdownToDataFrame, NamedFrames, ParquetToDataFrame, SqliteToDataFrames,
};
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

    if let Some(sub_cmd) = args.sub_command {
        match sub_cmd {
            tabiew::args::SubCommand::Ctl(args) => start_ctl(args),
        }
        return;
    }

    config::init().unwrap_or_graceful_shutdown();

    let parser = DataFrameParser::from_env_args();

    // Dataframe loading
    let mut name_dfs = Vec::new();

    // Load multiparts to data frames
    let mut multiparts = IndexMap::<Arc<Schema>, (String, DataFrame)>::new();
    for resource in args.multiparts.iter() {
        let resource = DataSource::from_str(resource).unwrap_or_graceful_shutdown();
        for (name, new_df) in try_read_path(&args, &resource).unwrap_or_graceful_shutdown() {
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
        parser.parse_and_update(&mut df);
        let name = sql().register(&name, df.clone(), TableSource::File(name.clone().into()));
        name_dfs.push((name, df));
    }

    // Load files to data frames
    for resource in args.sources.iter() {
        let resource = DataSource::from_str(resource).unwrap_or_graceful_shutdown();
        for (name, mut df) in try_read_path(&args, &resource).unwrap_or_graceful_shutdown() {
            parser.parse_and_update(&mut df);
            let name = sql().register(&name, df.clone(), resource.clone());
            name_dfs.push((name, df))
        }
    }

    if name_dfs.is_empty() {
        for (name, mut df) in build_reader(&args, "")
            .unwrap_or_graceful_shutdown()
            .read_to_data_frames(ReaderSource::Stdin)
            .unwrap_or_graceful_shutdown()
        {
            parser.parse_and_update(&mut df);
            let name = sql().register(&name, df.clone(), TableSource::Stdin);
            name_dfs.push((name, df))
        }
    }

    start_tui().unwrap_or_graceful_shutdown();
    start_app(name_dfs).unwrap_or_graceful_shutdown();
    let _ = stop_tui();
}

fn start_ctl(args: CtlArgs) {
    match args {
        CtlArgs::Ps => {
            let pids = fetch_other_process();
            println!(
                "{}",
                serde_json::to_string_pretty(&pids).unwrap_or_default()
            )
        }
        CtlArgs::Sql { pid: _, query: _ } => todo!(),
        CtlArgs::Schema { pid } => {
            let schema = fetch_sql_backend(pid);
            println!(
                "{}",
                serde_json::to_string_pretty(&schema).unwrap_or_default()
            )
        }
    }
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

        ipc::recv_iter().for_each(handle_ipc_message);
    }

    // Exit the user interface.
    Ok(())
}

fn try_read_path(args: &Args, source: &DataSource) -> AppResult<NamedFrames> {
    match source {
        DataSource::Stdin => build_reader(args, "")?.read_to_data_frames(ReaderSource::Stdin),
        DataSource::File(path_buf) => {
            build_reader(args, path_buf)?.read_to_data_frames(ReaderSource::File(path_buf.clone()))
        }
        DataSource::Url(url) => {
            let file = download_to_temp(url)?;
            build_reader(args, file.path())?
                .read_to_data_frames(ReaderSource::File(file.path().to_owned()))
        }
    }
}

fn build_reader(args: &Args, path: impl AsRef<Path>) -> AppResult<Box<dyn DataFrameReader>> {
    match args.format {
        Some(Format::Dsv) | Some(Format::Csv) => Ok(Box::new(CsvToDataFrame::from_args(args))),
        Some(Format::Tsv) => Ok(Box::new(
            CsvToDataFrame::from_args(args).with_separator('\t'),
        )),
        Some(Format::Parquet) => Ok(Box::new(ParquetToDataFrame::from_args(args))),
        Some(Format::Json) => Ok(Box::new(JsonToDataFrame::from_args(args))),
        Some(Format::Jsonl) => Ok(Box::new(JsonLineToDataFrame::from_args(args))),
        Some(Format::Arrow) => Ok(Box::new(ArrowIpcToDataFrame::from_args(args))),
        Some(Format::Fwf) => Ok(Box::new(FwfToDataFrame::from_args(args))),
        Some(Format::Sqlite) => Ok(Box::new(SqliteToDataFrames::from_args(args))),
        Some(Format::Excel) => Ok(Box::new(ExcelToDataFrames::from_args(args))),
        Some(Format::Logfmt) => Ok(Box::new(LogfmtToDataFrame::from_args(args))),
        Some(Format::Avro) => Ok(Box::new(AvroToDataFrame::from_args(args))),
        Some(Format::Html) => Ok(Box::new(HtmlToDataFrame::from_args(args))),
        Some(Format::Markdown) => Ok(Box::new(MarkdownToDataFrame::from_args(args))),
        None => match path.as_ref().extension().and_then(|ext| ext.to_str()) {
            Some("tsv") => {
                let reader = CsvToDataFrame::from_args(args).with_separator('\t');
                Ok(Box::new(reader))
            }
            Some("parquet") | Some("pqt") => Ok(Box::new(ParquetToDataFrame::from_args(args))),
            Some("json") => Ok(Box::new(JsonToDataFrame::from_args(args))),
            Some("jsonl") => Ok(Box::new(JsonLineToDataFrame::from_args(args))),
            Some("arrow") => Ok(Box::new(ArrowIpcToDataFrame::from_args(args))),
            Some("avro") => Ok(Box::new(AvroToDataFrame::from_args(args))),
            Some("fwf") => Ok(Box::new(FwfToDataFrame::from_args(args))),
            Some("db") | Some("sqlite") => Ok(Box::new(SqliteToDataFrames::from_args(args))),
            Some("xls") | Some("xlsx") | Some("xlsm") | Some("xlsb") => {
                Ok(Box::new(ExcelToDataFrames::from_args(args)))
            }
            Some("html") | Some("htm") => Ok(Box::new(HtmlToDataFrame::from_args(args))),
            Some("md") | Some("markdown") => Ok(Box::new(MarkdownToDataFrame::from_args(args))),
            _ => Ok(Box::new(CsvToDataFrame::from_args(args))),
        },
    }
}

fn handle_ipc_message(msg: ipc::Message) {
    match msg {
        ipc::Message::Ps => ipc::send(ipc::Message::PsReply {
            pid: std::process::id(),
        }),
        ipc::Message::Schema { pid } if pid == std::process::id() => {
            ipc::send(ipc::Message::SchemaReplay {
                pid,
                schema: sql().schema().clone(),
            })
        }
        ipc::Message::Schema { pid: _ } => (),
        ipc::Message::PsReply { pid: _ } => (),
        ipc::Message::SchemaReplay { pid: _, schema: _ } => (),
    }
}
