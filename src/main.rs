use polars::io::csv::CsvReader;
use polars::io::SerReader;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::io;
use tabiew::app::{App, AppResult};
use tabiew::event::{Event, EventHandler};
use tabiew::handler::handle_key_events;
use tabiew::tui::Tui;

fn main() -> AppResult<()> {
    // Create an application.
    let data_frame = CsvReader::from_path("sample_large.csv")
        .unwrap()
        .infer_schema(None)
        .has_header(true)
        .finish()
        .unwrap();
    let mut app = App::new(&data_frame);

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
            Event::Key(key_event) => handle_key_events(key_event, &mut app)?,
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        }
    }

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}
