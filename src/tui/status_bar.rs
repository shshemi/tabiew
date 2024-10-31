use std::marker::PhantomData;

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    layout::{Alignment, Rect},
    style::Style,
    text::{Line, Span},
    Frame,
};

use crate::tui::theme::Styler;

use super::widget::prompt::{Prompt, PromptState};
use crate::AppResult;

#[derive(Debug)]
pub struct StatusBar<Theme> {
    state: StatusBarState,
    prompt_history: Vec<String>,
    _theme: PhantomData<Theme>,
}

#[derive(Debug, Default)]
pub enum StatusBarState {
    #[default]
    Info,
    Error(String),
    Prompt(PromptState),
}

impl<Theme: Styler> Default for StatusBar<Theme> {
    fn default() -> Self {
        Self {
            state: Default::default(),
            prompt_history: Default::default(),
            _theme: PhantomData,
        }
    }
}

impl<Theme: Styler> StatusBar<Theme> {
    pub fn state(&self) -> &StatusBarState {
        &self.state
    }

    pub fn show_info(&mut self) -> AppResult<()> {
        self.state = StatusBarState::Info;
        Ok(())
    }

    pub fn show_error(&mut self, msg: impl ToString) -> AppResult<()> {
        self.state = StatusBarState::Error(msg.to_string());
        Ok(())
    }

    pub fn show_prompt(&mut self, prefix: impl AsRef<str>) -> AppResult<()> {
        let mut history = self.prompt_history.clone();
        history.push(format!(":{}", prefix.as_ref()));
        self.state = StatusBarState::Prompt(history.into());
        Ok(())
    }

    pub fn commit_prompt(&mut self) -> Option<String> {
        if let StatusBarState::Prompt(prompt) = &self.state {
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
        if let StatusBarState::Prompt(prompt) = &mut self.state {
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
                        self.show_info()?;
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

    pub fn render(
        &mut self,
        frame: &mut Frame,
        layout: Rect,
        info: &[(&str, &str)],
    ) -> AppResult<()> {
        match &mut self.state {
            StatusBarState::Info => frame.render_widget(
                Line::default()
                    .spans(
                        info.iter()
                            .enumerate()
                            .flat_map(|(i, (k, v))| info_key_value::<Theme>(i, k, v)),
                    )
                    .alignment(Alignment::Right)
                    .style(Theme::status_bar_info()),
                layout,
            ),

            StatusBarState::Error(msg) => frame.render_widget(
                Line::raw(msg.as_str())
                    .alignment(Alignment::Center)
                    .style(Theme::status_bar_error()),
                layout,
            ),

            StatusBarState::Prompt(text) => {
                frame.render_stateful_widget(
                    Prompt::new(
                        Theme::status_bar_prompt(),
                        invert_style(Theme::status_bar_prompt()),
                    ),
                    layout,
                    text,
                );
            }
        }
        Ok(())
    }
}

fn info_key_value<'a, Theme: Styler>(idx: usize, key: &'a str, value: &'a str) -> [Span<'a>; 3] {
    [
        Span::raw(format!(" {} ", key)).style(Theme::status_bar_info_key(idx)),
        Span::raw(format!(" {} ", value)).style(Theme::status_bar_info_val(idx)),
        Span::raw(" "),
    ]
}
fn invert_style(mut style: Style) -> Style {
    std::mem::swap(&mut style.bg, &mut style.fg);
    style
}
