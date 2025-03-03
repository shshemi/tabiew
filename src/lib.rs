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

/// Search
pub mod search;

/// Config
pub mod config;

pub type AppResult<T> = anyhow::Result<T>;
