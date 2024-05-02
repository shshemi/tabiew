use clap::Parser;
use polars::io::csv::CsvReader;
use polars::io::SerReader;
use polars::lazy::frame::IntoLazy;
use polars_sql::SQLContext;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::io;
use tabiew::app::{App, AppResult};
use tabiew::event::{Event, EventHandler};
use tabiew::handler::handle_key_events;
use tabiew::tui::Tui;

fn main() -> AppResult<()> {

    // Parse CLI
    let args = Args::parse();

    // Create an application.
    let data_frame = CsvReader::from_path(&args.file_name)?
        .infer_schema(None)
        .has_header(true)
        .finish()?;

    // Setup the SQLContext
    let mut sql_context = SQLContext::new();
    sql_context.register("df", data_frame.clone().lazy());

    // Instantiate app
    let mut app = App::new(data_frame);

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.init()?;

    // Start the main loop.
    while app.running {
        // Render the user interface.
        tui.draw(&mut app)?;
        // Handle events.
        match tui.events.next()? {
            Event::Tick => app.tick(),
            Event::Key(key_event) => handle_key_events(key_event, &mut app, &mut sql_context)?,
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        }
    }

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(help = "File to open", required = true)]
    file_name: String,
}
