use clap::Parser;
use serde_json::json;
use std::io::IsTerminal;
use std::str::FromStr;
use tabiew::app::App;
use tabiew::args::{Args, CtlArgs};
use tabiew::duckdb::{DataSource, DuckDB, IngestOptions};
use tabiew::handler::event::{Event, read_event};
use tabiew::handler::message::Message;
use tabiew::misc::config;
use tabiew::misc::osc52::flush_osc52_buffer;
use tabiew::misc::unwrap_or_graceful_shutdown::UnwrapOrGracefulShutdown;
use tabiew::tui::component::Component;
use tabiew::tui::tabs::Tabs;
use tabiew::tui::terminal::{draw, start_tui, stop_tui};
use tabiew::{AppResult, ctl};

#[tokio::main]
async fn main() {
    // Parse CLI
    let args = Args::parse();

    if let Some(sub_cmd) = args.sub_command {
        match sub_cmd {
            tabiew::args::SubCommand::Ctl(args) => {
                let result = run_ctl(args);
                ctl::Message::shutdown();
                if let Err(err) = result {
                    eprintln!("{}", json!({ "error": format!("{err:#}") }));
                    std::process::exit(1);
                }
            }
        }
        return;
    }

    config::init().unwrap_or_graceful_shutdown();

    let result = run(&args);
    ctl::Message::shutdown();
    if let Err(err) = result {
        eprintln!("{err:#}");
        std::process::exit(1);
    }
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

fn run(args: &Args) -> AppResult<()> {
    let mut duckdb = DuckDB::new(config::config().duckdb_config().clone())?;
    for file in &args.sql_file {
        duckdb.load_sql(file)?;
    }
    for sql in &args.commands {
        duckdb.load_query(sql);
    }
    let options = IngestOptions::from_args(args);
    let mut app = App::new(Tabs::new(Vec::new())).with_duckdb(duckdb, options.clone());
    let mut sources = args.sources.clone();
    if sources.is_empty() && args.multiparts.is_empty() && !std::io::stdin().is_terminal() {
        sources.push("-".to_string());
    }
    if !args.multiparts.is_empty() {
        let parts = args
            .multiparts
            .iter()
            .map(|part| DataSource::from_str(part))
            .collect::<AppResult<Vec<_>>>()?;
        app.load_parts(&parts, &options, args.max_rows);
    }
    for source in sources {
        app.load(
            &DataSource::from_str(&source)?,
            &options,
            args.max_rows,
            false,
        );
    }
    start_tui()?;
    let result = start_app(app);
    let _ = stop_tui();
    result
}

fn print_json(value: &impl serde::Serialize) -> AppResult<()> {
    println!("{}", serde_json::to_string_pretty(value)?);
    Ok(())
}

fn start_app(mut app: App) -> AppResult<()> {
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
            Event::FocusGained => {}
            Event::FocusLost => {}
            Event::Paste(_) => {}
        }

        apply_queued_messages(&mut app);

        flush_osc52_buffer();

        for msg in ctl::Message::recv_iter() {
            apply_queued_messages(&mut app);
            ctl::Reply::new(msg).with_app(&mut app).send();
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
