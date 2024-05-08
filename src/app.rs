use std::{default, error};

use polars::frame::DataFrame;
use ratatui::style::Style;
use tui_textarea::TextArea;

use crate::{
    theme::{Styler, Theme},
    utils::widths_from_dataframe,
};

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application.
#[derive(Debug)]
pub struct Table {
    pub data_frame: DataFrame,
    pub offset: usize,
    pub select: usize,
    pub rendered_rows: u16,
    pub widths: Vec<usize>,
    pub detailed_view: Option<u16>,
}

impl Table {
    /// Constructs a new instance of [`App`].
    pub fn new(data_frame: DataFrame) -> Self {
        Self {
            offset: 0,
            select: 0,
            rendered_rows: 0,
            widths: widths_from_dataframe(&data_frame),
            data_frame,
            detailed_view: None,
        }
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&mut self) {}


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

    pub fn item_view(&mut self) {
        self.detailed_view = 0.into();
    }

    pub fn table_view(&mut self) {
        self.detailed_view = None
    }

    pub fn toggle_detailed_view(&mut self) {
        if self.detailed_view.is_none() {
            self.item_view()
        } else {
            self.table_view();
        }
    }

    pub fn detailed_view_scroll_up(&mut self) {
        if let Some(v) = self.detailed_view {
            self.detailed_view = Some(v.saturating_sub(1))
        }
    }

    pub fn detailed_view_scroll_down(&mut self) {
        if let Some(v) = self.detailed_view {
            self.detailed_view = Some(v.saturating_add(1))
        }
    }

    pub fn set_data_frame(&mut self, data_frame: DataFrame) {
        self.widths = widths_from_dataframe(&data_frame);
        self.offset = 0;
        self.select = 0;
        self.data_frame = data_frame;
    }
}

#[derive(Debug, Default)]
pub enum StatusBar<'a> {
    #[default]
    Normal,
    Error(String, usize),
    Command(TextArea<'a>),
}


impl<'a> StatusBar<'a> {
    pub fn normal(&mut self) {
        self.update(StatusBar::Normal);
    }

    pub fn error(&mut self, msg: impl ToString, ticks: usize) {
        self.update(StatusBar::Error(msg.to_string(), ticks));
    }

    pub fn command(&mut self) -> &mut TextArea<'a> {
        if let StatusBar::Command(text_area) = self {
            text_area
        } else {
            let mut text_area = TextArea::default();
            text_area.set_style(Theme::status_bar_green());
            text_area.set_cursor_line_style(Style::default());
            self.update(StatusBar::Command(text_area));
            self.command()
        }
    }

    pub fn update(&mut self, status: StatusBar<'a>) {
        *self = status;
    }

    pub fn tick(&mut self) {
        if let StatusBar::Error(_, ref mut ticks) = self {
            if ticks == &0 {
                *self = StatusBar::Normal;
            } else {
                *ticks -= 1;
            }
        }
    }
}
