use std::error;

use polars::frame::DataFrame;
use ratatui::style::Style;
use tui_textarea::TextArea;

use crate::theme::{Styler, Theme};

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application.
#[derive(Debug)]
pub struct App<'a> {
    /// Is the application running?
    pub running: bool,

    pub data_frame: DataFrame,
    pub offset: usize,
    pub select: usize,
    pub rendered_rows: u16,
    pub status: AppStatus<'a>,
}

impl<'a> App<'a> {
    /// Constructs a new instance of [`App`].
    pub fn new(data_frame: DataFrame) -> Self {
        Self {
            running: true,
            data_frame,
            offset: 0,
            select: 0,
            rendered_rows: 0,
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
        self.select(self.select.saturating_sub(len))
    }

    pub fn select_down(&mut self, len: usize) {
        self.select(self.select + len);
    }

    pub fn select_first(&mut self) {
        self.select(usize::MIN)
    }

    pub fn select_last(&mut self) {
        self.select(usize::MAX);
    }

    pub fn select(&mut self, select: usize) {
        self.select = select.min(self.data_frame.height().saturating_sub(1))
    }

    pub fn adjust_offset(&mut self) {
        self.offset = self.offset.clamp(
            self.select
                .saturating_sub(self.rendered_rows.saturating_sub(1).into()),
            self.select,
        );
    }

    pub fn set_data_frame(&mut self, data_frame: DataFrame) {
        self.data_frame = data_frame;
        self.offset = 0;
        self.select = 0;
    }
}

#[derive(Debug)]
pub enum AppStatus<'a> {
    Normal,
    Error(String, usize),
    Command(TextArea<'a>),
}

impl<'a> AppStatus<'a> {
    pub fn normal(&mut self) {
        self.update(AppStatus::Normal);
    }

    pub fn error(&mut self, msg: impl ToString, ticks: usize) {
        self.update(AppStatus::Error(msg.to_string(), ticks));
    }

    pub fn command(&mut self) -> &mut TextArea<'a> {
        if let AppStatus::Command(text_area) = self {
            text_area
        } else {
            let mut text_area = TextArea::default();
            text_area.set_style(Theme::status_bar_green());
            text_area.set_cursor_line_style(Style::default());
            self.update(AppStatus::Command(text_area));
            self.command()
        }
    }

    pub fn update(&mut self, status: AppStatus<'a>) {
        *self = status;
    }
}
