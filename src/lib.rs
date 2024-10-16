use std::error;

/// User interface.
pub mod tui;

/// Utils
pub mod utils;

/// CLI arguments
pub mod args;

/// SQL
pub mod sql;

/// Event, keybind, and commands
pub mod handler;

/// App
pub mod app;

/// Readers
pub mod reader;

/// Writers
pub mod writer;

pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;
