/// User interface.
pub mod tui;

/// User interface, take two: State/Widget split, built incrementally
/// alongside `tui` until it can replace it. See `sw` module docs.
pub mod sw;

/// SQL autocompletion logic.
pub mod sql_completion;

/// Utils
pub mod misc;

/// CLI arguments
pub mod args;

/// Event, keybind, and commands
pub mod handler;

/// App
pub mod app;

/// IO
pub mod io;

pub type AppResult<T> = anyhow::Result<T>;
