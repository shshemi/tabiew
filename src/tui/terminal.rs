use crate::AppResult;
use crate::app::App;
use crate::misc::config::theme;
use crate::misc::type_ext::UnwrapOrGracefulShutdown;
use crate::tui::component::Component;
use crate::tui::component::FocusState;
use crossterm::terminal::{self, EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::prelude::CrosstermBackend;
use std::io;
use std::io::Stdout;
use std::ops::DerefMut;
use std::panic;
use std::sync::Mutex;
use std::sync::OnceLock;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;

type TerminalType = ratatui::Terminal<CrosstermBackend<Stdout>>;
static INVALIDATE_TUI: AtomicBool = AtomicBool::new(false);

pub fn start_tui() -> AppResult<()> {
    terminal::enable_raw_mode()?;
    crossterm::execute!(io::stdout(), EnterAlternateScreen)?;
    let panic_hook = panic::take_hook();
    panic::set_hook(Box::new(move |panic| {
        forece_stop_tui();
        panic_hook(panic);
    }));

    terminal().hide_cursor()?;
    terminal().clear()?;
    Ok(())
}

pub fn stop_tui() -> AppResult<()> {
    terminal::disable_raw_mode()?;
    crossterm::execute!(io::stdout(), LeaveAlternateScreen)?;
    terminal().show_cursor()?;
    Ok(())
}

pub fn forece_stop_tui() {
    if terminal::is_raw_mode_enabled().unwrap_or(false) {
        let _ = terminal::disable_raw_mode();
    }
    let _ = crossterm::execute!(io::stdout(), LeaveAlternateScreen);
    let _ = ratatui::Terminal::new(CrosstermBackend::new(io::stdout()))
        .and_then(|mut term| term.show_cursor());
}

pub fn draw(app: &mut App) -> AppResult<()> {
    if INVALIDATE_TUI.swap(false, Ordering::Relaxed) {
        terminal().clear().unwrap_or_graceful_shutdown();
    }
    terminal().draw(|frame| {
        let area = frame.area();
        let buf = frame.buffer_mut();
        for cell in buf.content.iter_mut() {
            cell.set_style(theme().background());
        }
        app.render(area, buf, FocusState::Focused);
    })?;
    Ok(())
}

pub fn invalidate_tui() {
    INVALIDATE_TUI.store(true, Ordering::Relaxed);
}

fn terminal() -> impl DerefMut<Target = TerminalType> {
    static TERMINAL: OnceLock<Mutex<TerminalType>> = OnceLock::new();
    TERMINAL
        .get_or_init(|| {
            //
            Mutex::new(
                ratatui::Terminal::new(CrosstermBackend::new(io::stdout()))
                    .unwrap_or_graceful_shutdown(),
            )
        })
        .lock()
        .unwrap_or_graceful_shutdown()
}
