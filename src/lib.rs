/// User interface.
pub mod tui;

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

pub mod parsers;

pub mod iters;

pub mod net;

pub type AppResult<T> = anyhow::Result<T>;
