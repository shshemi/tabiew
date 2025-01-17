use std::marker::PhantomData;

use ratatui::{
    layout::{Alignment, Rect},
    text::{Line, Span},
    widgets::{StatefulWidget, Widget},
};

use crate::tui::theme::Styler;

use super::{
    prompt::{Prompt, PromptState},
    utils::invert_style,
};
use crate::AppResult;

#[derive(Debug)]
pub enum StatusBarView {
    Info,
    Error(String),
    Prompt(PromptState),
    Search(PromptState),
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

    pub fn view_mut(&mut self) -> &mut StatusBarView {
        &mut self.view
    }

    pub fn switch_info(&mut self) {
        self.view = StatusBarView::Info;
    }

    pub fn switch_error(&mut self, msg: impl ToString) {
        self.view = StatusBarView::Error(msg.to_string());
    }

    pub fn switch_prompt(&mut self, prefix: impl AsRef<str>) {
        let mut history = self.prompt_history.clone();
        history.push(format!(":{}", prefix.as_ref()));
        self.view = StatusBarView::Prompt(history.into());
    }

    pub fn switch_search(&mut self, prefix: impl AsRef<str>) {
        self.view = StatusBarView::Search(vec![format!("/{}", prefix.as_ref())].into());
    }

    pub fn commit_prompt(&mut self) -> Option<String> {
        if let StatusBarView::Prompt(prompt) = &self.view {
            let command = prompt.line();
            self.prompt_history.push(command.clone());
            Some(command)
        } else {
            None
        }
    }

    pub fn search_string(&self) -> Option<String> {
        if let StatusBarView::Search(prompt) = &self.view {
            Some(prompt.line())
        } else {
            None
        }
    }

    pub fn tick(&mut self) -> AppResult<()> {
        Ok(())
    }
}

impl Default for StatusBarState {
    fn default() -> Self {
        Self::new()
    }
}
pub struct StatusBar<'a, Theme> {
    tags: &'a [StatusBarTag<'a, Theme>],
    _theme: PhantomData<Theme>,
}

pub struct StatusBarTag<'a, Theme> {
    key: &'a str,
    value: &'a str,
    _theme: PhantomData<Theme>,
}

impl<'a, Theme: Styler> StatusBarTag<'a, Theme> {
    pub fn new(key: &'a str, value: &'a str) -> Self {
        Self {
            key,
            value,
            _theme: Default::default(),
        }
    }

    fn spans(&self, position: usize) -> [Span; 3] {
        [
            Span::raw(format!(" {} ", self.key)).style(Theme::status_bar_info_key(position)),
            Span::raw(format!(" {} ", self.value)).style(Theme::status_bar_info_val(position)),
            Span::raw(" "),
        ]
    }
}

impl<'a, Theme: Styler> StatusBar<'a, Theme> {
    pub fn new(tags: &'a [StatusBarTag<Theme>]) -> Self {
        Self {
            tags,
            _theme: Default::default(),
        }
    }
}

impl<Theme: Styler> StatefulWidget for StatusBar<'_, Theme> {
    type State = StatusBarState;

    fn render(self, area: Rect, buf: &mut ratatui::prelude::Buffer, state: &mut Self::State) {
        match &mut state.view {
            StatusBarView::Info => Widget::render(
                Line::default()
                    .spans(
                        self.tags
                            .iter()
                            .enumerate()
                            .flat_map(|(i, tag)| tag.spans(i)),
                    )
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
            StatusBarView::Search(text) => {
                StatefulWidget::render(
                    Prompt::new(
                        Theme::status_bar_search(),
                        invert_style(Theme::status_bar_search()),
                    ),
                    area,
                    buf,
                    text,
                );
            }
        }
    }
}
