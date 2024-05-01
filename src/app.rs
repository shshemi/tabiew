use std::error;

use polars::frame::DataFrame;
use ratatui::widgets::{Table, TableState};

use crate::utils::tabulate;

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application.
#[derive(Debug)]
pub struct App<'a> {
    /// Is the application running?
    pub running: bool,

    pub table: Table<'a>,
    pub table_state: TableState,
    pub rows: usize,
    pub cols: usize,
    pub visible_rows: u16,
    pub status: AppStatus,
}

impl<'a> App<'a> {
    /// Constructs a new instance of [`App`].
    pub fn new(data_frame: &'a DataFrame) -> Self {
        Self {
            running: true,
            table: tabulate(data_frame),
            table_state: TableState::new().with_offset(0).with_selected(0),
            rows: data_frame.height(),
            cols: data_frame.width(),
            visible_rows: 0,
            status: AppStatus::Normal,
        }
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&mut self) {
        if let AppStatus::Error(_, ref mut ticks) = self.status {
            if ticks == &0 {
                self.status = AppStatus::Normal;
            } else {
                *ticks -= 1;
            }
        }
    }

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn select_up(&mut self, len: usize) {
        self.table_state.select(
            self.table_state
                .selected()
                .map(|idx| idx.saturating_sub(len)),
        )
    }

    pub fn select_down(&mut self, len: usize) {
        self.table_state.select(
            self.table_state
                .selected()
                .map(|idx| idx.saturating_add(len).min(self.rows - 1)),
        )
    }

    pub fn select_first(&mut self) {
        self.table_state.select(0.into())
    }

    pub fn select_last(&mut self) {
        self.table_state.select(self.rows.saturating_sub(1).into())
    }

    pub fn set_data_frame(&mut self, data_frame: &'a DataFrame) {
        self.table = tabulate(data_frame);
        self.table_state = TableState::new().with_offset(0).with_selected(0);
        self.rows = data_frame.height();
        self.cols = data_frame.width();
    }

    pub fn state_normal(&mut self) {
        self.status = AppStatus::Normal;
    }

    pub fn state_error(&mut self, msg: String, ticks: usize) {
        self.status = AppStatus::Error(msg, ticks);
    }
}

#[derive(Debug)]
pub enum AppStatus {
    Normal,
    Error(String, usize),
}
