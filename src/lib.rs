/// User interface.
pub mod tui;

/// Utils
pub mod misc;

/// CLI arguments
pub mod args;

/// Event, keybind, and commands
pub mod handler;

/// App
pub mod app;

/// Readers
pub mod reader;

/// Writers
pub mod writer;

pub type AppResult<T> = anyhow::Result<T>;
