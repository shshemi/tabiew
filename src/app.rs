use std::error;

use polars::{
    frame::DataFrame,
    io::{csv::CsvReader, SerReader},
};

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,

    // pub data_frame: DataFrame,
    pub data_frame: DataFrame,

    pub table_offset: (i64, usize),
    pub table_select: usize,
    pub table_height: u16,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            data_frame: CsvReader::from_path("sample.csv")
                .unwrap()
                .infer_schema(None)
                .has_header(true)
                .finish()
                .unwrap(),
            table_offset: (0, 0),
            table_select: 0,
            table_height: 0,
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }
}
