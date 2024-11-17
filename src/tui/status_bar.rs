use std::marker::PhantomData;

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    layout::{Alignment, Rect},
    text::{Line, Span},
    widgets::{StatefulWidget, Widget},
};

use crate::tui::theme::Styler;

use super::{
    utils::invert_style,
    prompt::{Prompt, PromptState},
};
use crate::AppResult;

#[derive(Debug)]
pub enum StatusBarView {
    Info,
    Error(String),
    Prompt(PromptState),
}

#[derive(Debug)]
pub struct StatusBarState {
    view: StatusBarView,
    prompt_history: Vec<String>,
}

impl StatusBarState {
    pub fn new() -> Self {
        Self {
            view: StatusBarView::Info,
            prompt_history: Default::default(),
        }
    }

    pub fn view(&self) -> &StatusBarView {
        &self.view
    }

    pub fn switch_info(&mut self) -> AppResult<()> {
        self.view = StatusBarView::Info;
        Ok(())
    }

    pub fn switch_error(&mut self, msg: impl ToString) -> AppResult<()> {
        self.view = StatusBarView::Error(msg.to_string());
        Ok(())
    }

    pub fn switch_prompt(&mut self, prefix: impl AsRef<str>) -> AppResult<()> {
        let mut history = self.prompt_history.clone();
        history.push(format!(":{}", prefix.as_ref()));
        self.view = StatusBarView::Prompt(history.into());
        Ok(())
    }

    pub fn commit_prompt(&mut self) -> Option<String> {
        if let StatusBarView::Prompt(prompt) = &self.view {
            let command = prompt.command();
            self.prompt_history.push(command.clone());
            Some(command)
        } else {
            None
        }
    }

    pub fn tick(&mut self) -> AppResult<()> {
        Ok(())
    }

    pub fn input(&mut self, input: KeyEvent) -> AppResult<()> {
        if let StatusBarView::Prompt(prompt) = &mut self.view {
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
                        self.switch_info()?;
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
        Ok(())
    }
}

impl Default for StatusBarState {
    fn default() -> Self {
        Self::new()
    }
}
pub struct StatusBar<'a, Theme> {
    info: &'a [(&'a str, &'a str)],
    _theme: PhantomData<Theme>,
}

impl<'a, Theme: Styler> StatusBar<'a, Theme> {
    pub fn new(info: &'a [(&'a str, &'a str)]) -> Self {
        Self {
            info,
            _theme: Default::default(),
        }
    }
}

impl<'a, Theme: Styler> StatefulWidget for StatusBar<'a, Theme> {
    type State = StatusBarState;

    fn render(self, area: Rect, buf: &mut ratatui::prelude::Buffer, state: &mut Self::State) {
        match &mut state.view {
            StatusBarView::Info => Widget::render(
                Line::default()
                    .spans(self.info.iter().enumerate().flat_map(|(i, (k, v))| {
                        [
                            Span::raw(format!(" {} ", k)).style(Theme::status_bar_info_key(i)),
                            Span::raw(format!(" {} ", v)).style(Theme::status_bar_info_val(i)),
                            Span::raw(" "),
                        ]
                    }))
                    .alignment(Alignment::Right)
                    .style(Theme::status_bar_info()),
                area,
                buf,
            ),

            StatusBarView::Error(msg) => Widget::render(
                Line::raw(msg.as_str())
                    .alignment(Alignment::Center)
                    .style(Theme::status_bar_error()),
                area,
                buf,
            ),

            StatusBarView::Prompt(text) => {
                StatefulWidget::render(
                    Prompt::new(
                        Theme::status_bar_prompt(),
                        invert_style(Theme::status_bar_prompt()),
                    ),
                    area,
                    buf,
                    text,
                );
            }
        }
    }
}
