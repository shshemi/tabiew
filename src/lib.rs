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

/// DuckDB
pub mod duckdb;

pub mod indexer;

pub mod reader;

pub mod iters;

pub mod net;

pub mod collections;

pub mod writers;

pub mod ctl;

pub type AppResult<T> = anyhow::Result<T>;
