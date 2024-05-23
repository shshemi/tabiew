use std::error;

use crossterm::event::{KeyCode, KeyEvent};
use polars::frame::DataFrame;
use ratatui::style::Style;
use tui_textarea::{CursorMove, TextArea};

use crate::{
    theme::{Styler, Theme},
    utils::{widths_from_dataframe, Scroll},
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
    pub detailed_view: Option<Scroll>,
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

    pub fn switch_view(&mut self) {
        if self.detailed_view.is_none() {
            self.detailed_view = Scroll::default().into();
        } else {
            self.detailed_view = None;
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
pub struct StatusBar<'a> {
    pub state: StatusBarState<'a>,
    prompt_history: Vec<String>,
}

#[derive(Debug, Default)]
pub enum StatusBarState<'a> {
    #[default]
    Normal,
    Error(String),
    Command(TextArea<'a>),
}

impl<'a> StatusBar<'a> {
    pub fn normal(&mut self) {
        self.state = StatusBarState::Normal;
    }

    pub fn error(&mut self, msg: impl ToString) {
        self.state = StatusBarState::Error(msg.to_string());
    }

    pub fn command(&mut self, prefix: impl ToString) {
        let mut history = self.prompt_history.clone();
        history.push(prefix.to_string());
        let mut text_area = TextArea::new(history);
        text_area.set_style(Theme::status_bar_green());
        text_area.set_cursor_line_style(Style::default());
        text_area.move_cursor(tui_textarea::CursorMove::Bottom);
        text_area.move_cursor(tui_textarea::CursorMove::End);
        self.state = StatusBarState::Command(text_area);
    }

    pub fn commit_prompt(&mut self) -> Option<&String> {
        if let StatusBarState::Command(prompt) = &self.state {
            let row = prompt.cursor().0;
            let prompt = &prompt.lines()[row];
            self.prompt_history.push(prompt.clone());
            Some(prompt)
        } else {
            None
        }
    }

    pub fn tick(&mut self) {}

    pub fn input(&mut self, input: KeyEvent) {
        if let StatusBarState::Command(prompt) = &mut self.state {
            match input.code {
                KeyCode::Up => {
                    prompt.move_cursor(CursorMove::Up);
                    prompt.move_cursor(CursorMove::End);
                }
                KeyCode::Down => {
                    prompt.move_cursor(CursorMove::Down);
                    prompt.move_cursor(CursorMove::End);
                }
                KeyCode::Left => {
                    if prompt.cursor().1 > 1 {
                        prompt.input(input);
                    }
                }
                KeyCode::Right => {
                    prompt.input(input);
                }

                KeyCode::Backspace => {
                    if prompt.lines()[prompt.cursor().0].len() == 1 {
                        self.normal()
                    } else if prompt.cursor().1 > 1 {
                        prompt.input(input);
                    }
                }

                KeyCode::Delete => {
                    let (row, col) = prompt.cursor();
                    if col < prompt.lines()[row].len() {
                        prompt.input(input);
                    }
                }

                KeyCode::Home => {
                    prompt.move_cursor(CursorMove::Head);
                    prompt.move_cursor(CursorMove::Forward);
                }

                KeyCode::End => {
                    let (row, col) = prompt.cursor();
                    if col < prompt.lines()[row].len() {
                        prompt.move_cursor(CursorMove::End);
                    }
                }

                KeyCode::PageUp | KeyCode::PageDown => (),

                KeyCode::Char(_) => {
                    prompt.input(input);
                }

                _ => (),
            }
        }
    }
}
