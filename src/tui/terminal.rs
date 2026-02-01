use crate::AppResult;
use crate::app::App;
use crate::misc::type_ext::UnwrapOrGracefulShutdown;
use crate::tui::component::Component;
use crate::tui::component::FocusState;
use crossterm::terminal::{self, EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::backend::Backend;
use std::io;
use std::panic;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;

/// Representation of a terminal user interface.
///
/// It is responsible for setting up the terminal,
/// initializing the interface and handling the draw events.
pub static INVALIDATE_DRAW: AtomicBool = AtomicBool::new(false);

#[derive(Debug)]
pub struct Terminal<B: Backend> {
    /// Interface to the Terminal.
    terminal: ratatui::Terminal<B>,
}

impl<B: Backend> Terminal<B> {
    /// Constructs a new instance of [`Tui`].
    pub fn new(terminal: ratatui::Terminal<B>) -> Self {
        Self { terminal }
    }

    /// Initializes the terminal interface.
    ///
    /// It enables the raw mode and sets terminal properties.
    pub fn init(&mut self) {
        terminal::enable_raw_mode().unwrap_or_graceful_shutdown();
        crossterm::execute!(io::stdout(), EnterAlternateScreen).unwrap_or_graceful_shutdown();

        // Define a custom panic hook to reset the terminal properties.
        // This way, you won't have your terminal messed up if an unexpected error happens.
        let panic_hook = panic::take_hook();
        panic::set_hook(Box::new(move |panic| {
            Self::reset().expect("failed to reset the terminal");
            panic_hook(panic);
        }));

        self.terminal.hide_cursor().unwrap_or_graceful_shutdown();
        self.terminal.clear().unwrap_or_graceful_shutdown();
    }

    /// [`Draw`] the terminal interface by [`rendering`] the widgets.
    ///
    /// [`Draw`]: ratatui::Terminal::draw
    /// [`rendering`]: crate::ui::render
    pub fn draw(&mut self, app: &mut App) {
        if INVALIDATE_DRAW.swap(false, Ordering::Relaxed) {
            self.terminal.clear().unwrap_or_graceful_shutdown();
        }
        self.terminal
            .draw(|frame| {
                let area = frame.area();
                let buf = frame.buffer_mut();
                app.render(area, buf, FocusState::Focused);
            })
            .unwrap_or_graceful_shutdown();
    }

    /// Resets the terminal interface.
    ///
    /// This function is also used for the panic hook to revert
    /// the terminal properties if unexpected errors occur.
    fn reset() -> AppResult<()> {
        terminal::disable_raw_mode()?;
        crossterm::execute!(io::stdout(), LeaveAlternateScreen)?;
        Ok(())
    }

    /// Exits the terminal interface.
    ///
    /// It disables the raw mode and reverts back the terminal properties.
    pub fn exit(&mut self) {
        Self::reset().unwrap_or_graceful_shutdown();
        self.terminal.show_cursor().unwrap_or_graceful_shutdown();
    }
}
