use std::error;

use crossterm::event::{KeyCode, KeyEvent};
use polars::frame::DataFrame;
use rand::Rng;

use crate::{
    command_pallete::CommandPalleteState,
    utils::{data_frame_widths, Scroll, ValuePool2D},
};

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application.
#[derive(Debug)]
pub struct Tabular {
    pub offset: usize,
    pub select: usize,
    pub rendered_rows: u16,
    pub widths: Vec<usize>,
    pub headers: Vec<String>,
    pub value_pool: ValuePool2D,
    pub detailed_view: Option<Scroll>,
}

impl Tabular {
    /// Constructs a new instance of [`App`].
    pub fn new(data_frame: DataFrame) -> Self {
        Self {
            offset: 0,
            select: 0,
            rendered_rows: 0,
            widths: data_frame_widths(&data_frame),
            headers: data_frame.get_column_names().into_iter().map(ToOwned::to_owned).collect(),
            value_pool: ValuePool2D::from_dataframe(data_frame),
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

    pub fn select_random(&mut self) {
        let mut rng = rand::thread_rng();
        self.select(rng.gen_range(0..self.value_pool.height()))
    }

    pub fn select(&mut self, select: usize) {
        self.select = select.min(self.value_pool.height().saturating_sub(1))
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
        self.widths = data_frame_widths(&data_frame);
        self.offset = 0;
        self.select = 0;
        self.headers = data_frame.get_column_names().into_iter().map(ToOwned::to_owned).collect();
        self.value_pool.replace_dataframe(data_frame);
    }
}

#[derive(Debug, Default)]
pub struct StatusBar {
    pub state: StatusBarState,
    prompt_history: Vec<String>,
}

#[derive(Debug, Default)]
pub enum StatusBarState {
    #[default]
    Normal,
    Error(String),
    Command(CommandPalleteState),
}

impl StatusBar {
    pub fn normal(&mut self) {
        self.state = StatusBarState::Normal;
    }

    pub fn error(&mut self, msg: impl ToString) {
        self.state = StatusBarState::Error(msg.to_string());
    }

    pub fn command(&mut self, prefix: impl ToString) {
        let mut history = self.prompt_history.clone();
        history.push(prefix.to_string());
        self.state = StatusBarState::Command(history.into());
    }

    pub fn commit_prompt(&mut self) -> Option<String> {
        if let StatusBarState::Command(prompt) = &self.state {
            let command = prompt.command();
            self.prompt_history.push(command.clone());
            Some(command)
        } else {
            None
        }
    }

    pub fn tick(&mut self) {}

    pub fn input(&mut self, input: KeyEvent) {
        if let StatusBarState::Command(prompt) = &mut self.state {
            match input.code {
                KeyCode::Up => {
                    prompt.move_up().move_eol();
                }
                KeyCode::Down => {
                    prompt.move_down().move_eol();
                }
                KeyCode::Left => {
                    if prompt.cursor().1 > 1 {
                        prompt.move_left();
                    }
                }
                KeyCode::Right => {
                    prompt.move_right();
                }

                KeyCode::Backspace => {
                    if prompt.command_len() == 1 {
                        self.normal()
                    } else if prompt.cursor().1 > 1 {
                        prompt.delete_backward();
                    }
                }

                KeyCode::Delete => {
                    prompt.delete();
                }

                KeyCode::Home => {
                    prompt.move_bol().move_right();
                }

                KeyCode::End => {
                    prompt.move_eol();
                }

                KeyCode::PageUp | KeyCode::PageDown => (),

                KeyCode::Char(c) => {
                    prompt.input_char(c);
                }

                _ => (),
            }
        }
    }
}
