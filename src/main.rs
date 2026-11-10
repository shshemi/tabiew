use clap::Parser;
use indexmap::IndexMap;
use itertools::Itertools;
use polars::frame::DataFrame;
use polars::prelude::Schema;
use serde_json::json;
use std::io::IsTerminal;
use std::path::Path;
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;
use tabiew::app::App;
use tabiew::args::{Args, CtlArgs, Format};
use tabiew::duckdb::{DataSource, DuckDB, IngestOptions, Load};
use tabiew::handler::event::{Event, read_event};
use tabiew::handler::message::Message;
use tabiew::misc::config;
use tabiew::misc::osc52::flush_osc52_buffer;
use tabiew::misc::sql::{TableSource, sql};
use tabiew::misc::unwrap_or_graceful_shutdown::UnwrapOrGracefulShutdown;
use tabiew::net::downloader::download_to_temp;
use tabiew::parsers::data_frame_parser::DataFrameParser;
use tabiew::readers::ReaderSource;
use tabiew::readers::{
    ArrowIpcToDataFrame, AvroToDataFrame, CsvToDataFrame, DataFrameReader, ExcelToDataFrames,
    FwfToDataFrame, HtmlToDataFrame, JsonLineToDataFrame, JsonToDataFrame, LogfmtToDataFrame,
    MarkdownToDataFrame, NamedFrames, ParquetToDataFrame, SqliteToDataFrames,
};
use tabiew::tui::component::Component;
use tabiew::tui::pane::TableDescription;
use tabiew::tui::tabs::Tabs;
use tabiew::tui::terminal::{draw, start_tui, stop_tui};

use tabiew::tui::Pane;
use tabiew::{AppResult, ctl};

#[tokio::main]
async fn main() {
    // Parse CLI
    let args = Args::parse();

    if let Some(sub_cmd) = args.sub_command {
        match sub_cmd {
            tabiew::args::SubCommand::Ctl(args) => {
                if let Err(err) = run_ctl(args) {
                    eprintln!("{}", json!({ "error": format!("{err:#}") }));
                    std::process::exit(1);
                }
            }
        }
        return;
    }

    config::init().unwrap_or_graceful_shutdown();

    #[cfg(debug_assertions)]
    if let Some(path) = &args.duckdb_inspect {
        if let Err(err) = tabiew::duckdb::inspect(path, args.inspect_rows) {
            eprintln!("{err:#}");
            std::process::exit(1);
        }
        return;
    }

    if args.duckdb {
        if let Err(err) = run_duckdb(&args) {
            eprintln!("{err:#}");
            std::process::exit(1);
        }
        return;
    }

    let parser = DataFrameParser::from_env_args();

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
        sql().register(&name, df.clone(), TableSource::File(name.clone().into()));
    }

    // Load files to data frames
    for resource in args.sources.iter() {
        let resource = DataSource::from_str(resource).unwrap_or_graceful_shutdown();
        for (name, mut df) in try_read_path(&args, &resource).unwrap_or_graceful_shutdown() {
            parser.parse_and_update(&mut df);
            sql().register(&name, df.clone(), resource.clone());
        }
    }

    // Read stdin only when it is piped, otherwise start with no tabs
    if sql().schema().is_empty() && !std::io::stdin().is_terminal() {
        for (name, mut df) in build_reader(&args, "")
            .unwrap_or_graceful_shutdown()
            .read_to_data_frames(ReaderSource::Stdin)
            .unwrap_or_graceful_shutdown()
        {
            parser.parse_and_update(&mut df);
            sql().register(&name, df.clone(), TableSource::Stdin);
        }
    }

    start_tui().unwrap_or_graceful_shutdown();
    start_app().unwrap_or_graceful_shutdown();
    let _ = stop_tui();
}

fn run_ctl(args: CtlArgs) -> AppResult<()> {
    match args {
        CtlArgs::Ps => print_json(&ctl::ops::fetch_other_process()?)?,
        CtlArgs::Sql { pid, query } => print_json(&ctl::ops::send_sql_query(pid, query)?)?,
        CtlArgs::Schema { pid } => print_json(&ctl::ops::fetch_sql_backend(pid)?)?,
        CtlArgs::Tabs { pid } => print_json(&ctl::ops::fetch_tabs(pid)?)?,
        CtlArgs::TabSwitch { pid, tab_idx } => {
            print_json(&json!({ "tab_idx": ctl::ops::switch_tab(pid, tab_idx)? }))?
        }
        CtlArgs::Filter { pid, filter } => print_json(&ctl::ops::send_filter(pid, filter)?)?,
        CtlArgs::Select { pid, columns } => print_json(&ctl::ops::send_select(pid, columns)?)?,
        CtlArgs::Order { pid, order_by } => print_json(&ctl::ops::send_order(pid, order_by)?)?,
        CtlArgs::Export { args } => {
            print_json(&json!({ "path": ctl::ops::send_export(args.pid(), &args)? }))?
        }
        CtlArgs::RowSelect { pid, row_idx } => {
            print_json(&json!({ "row_idx": ctl::ops::select_row(pid, row_idx)? }))?
        }
        CtlArgs::Rows { pid, offset, count } => {
            print_json(&ctl::ops::fetch_rows(pid, offset, count)?.to_json()?)?
        }
        CtlArgs::RmTab { pid, tab_idx } => {
            print_json(&json!({ "removed_tab_idx": ctl::ops::remove_tab(pid, tab_idx)? }))?
        }
        CtlArgs::Import { args } => {
            print_json(&json!({ "tables": ctl::ops::send_import(args.pid(), &args)? }))?
        }
    }
    Ok(())
}

fn run_duckdb(args: &Args) -> AppResult<()> {
    let mut duckdb = DuckDB::new(config::config().duckdb_config().clone())?;
    let options = IngestOptions::from_args(args);
    let mut ingesting = 0;
    #[cfg(debug_assertions)]
    let mut bars = Vec::new();
    let mut sources = args.sources.clone();
    if sources.is_empty() && args.multiparts.is_empty() && !std::io::stdin().is_terminal() {
        sources.push("-".to_string());
    }
    let mut loads = Vec::new();
    if !args.multiparts.is_empty() {
        let parts = args
            .multiparts
            .iter()
            .map(|part| DataSource::from_str(part))
            .collect::<AppResult<Vec<_>>>()?;
        let loaded = duckdb.load_parts(&parts, &options, args.max_rows);
        loads.push((args.multiparts.join(" + "), loaded));
    }
    for source in sources {
        let loaded = match DataSource::from_str(&source)? {
            DataSource::Stdin => duckdb.load_stdin(None, &options, args.max_rows),
            DataSource::File(path) => duckdb.load(&path, None, &options, args.max_rows),
            DataSource::Url(url) => duckdb.load_url(&url, None, &options, args.max_rows),
        };
        loads.push((source, loaded));
    }
    for (source, loaded) in loads {
        match loaded {
            Ok(Load::Ingesting(_progress)) => {
                ingesting += 1;
                println!("{source}: ingesting");
                #[cfg(debug_assertions)]
                bars.push((source, _progress, None));
            }
            Ok(Load::Ready(names)) => {
                println!("{source}: loaded from the cache as {}", names.join(", "))
            }
            Err(err) => eprintln!("{source}: {err:#}"),
        }
    }
    while ingesting > 0 {
        std::thread::sleep(Duration::from_millis(100));
        #[cfg(debug_assertions)]
        for (source, progress, last) in &mut bars {
            let percent = progress
                .fraction()
                .map(|fraction| (fraction * 100.0) as u64);
            if percent != *last {
                *last = percent;
                if let Some(percent) = percent {
                    println!("{source}: {percent}%");
                }
            }
        }
        for (source, result) in duckdb.flush()?.tables {
            ingesting -= 1;
            match result {
                Ok(names) => println!("{}: ready as {}", source.display(), names.join(", ")),
                Err(err) => eprintln!("{}: {err:#}", source.display()),
            }
        }
    }
    Ok(())
}

fn print_json(value: &impl serde::Serialize) -> AppResult<()> {
    println!("{}", serde_json::to_string_pretty(value)?);
    Ok(())
}

fn start_app() -> AppResult<()> {
    let names = sql()
        .schema()
        .iter()
        .map(|(name, _)| name.to_owned())
        .collect_vec();

    let panes = names
        .into_iter()
        .map(|name| {
            let df = sql().execute_with_default(
                &format!("SELECT * FROM \"{}\"", name.replace('"', "\"\"")),
                None,
            )?;
            Ok(Pane::new(df, TableDescription::Table(name.to_owned())))
        })
        .collect::<AppResult<Vec<_>>>()?;

    let tabs = Tabs::new(panes);

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

        apply_queued_messages(&mut app);

        flush_osc52_buffer();

        for msg in ctl::Message::recv_iter() {
            apply_queued_messages(&mut app);
            ctl::Reply::new(msg).with_app(&app).send();
        }
    }

    // Exit the user interface.
    Ok(())
}

fn apply_queued_messages(app: &mut App) {
    while let Some(action) = Message::dequeue() {
        app.update(&action);
    }
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
