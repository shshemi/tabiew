use std::borrow::Cow;

use crate::{
    misc::globals::theme,
    tui::{themes::styler::Styler, widgets::block::Block},
};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    widgets::{Paragraph, StatefulWidget, Widget},
};

#[derive(Debug, Default)]
pub struct InputState {
    input: tui_input::Input,
}

impl InputState {
    pub fn delete_prev(&mut self) {
        self.input.handle(tui_input::InputRequest::DeletePrevChar);
    }

    pub fn delete_next(&mut self) {
        self.input.handle(tui_input::InputRequest::DeleteNextChar);
    }

    pub fn goto_prev(&mut self) {
        self.input.handle(tui_input::InputRequest::GoToPrevChar);
    }

    pub fn goto_next(&mut self) {
        self.input.handle(tui_input::InputRequest::GoToNextChar);
    }

    pub fn goto_start(&mut self) {
        self.input.handle(tui_input::InputRequest::GoToStart);
    }

    pub fn goto_end(&mut self) {
        self.input.handle(tui_input::InputRequest::GoToEnd);
    }

    pub fn insert(&mut self, c: char) {
        self.input.handle(tui_input::InputRequest::InsertChar(c));
    }

    pub fn goto_next_word(&mut self) {
        self.input.handle(tui_input::InputRequest::GoToNextWord);
    }

    pub fn goto_prev_word(&mut self) {
        self.input.handle(tui_input::InputRequest::GoToPrevWord);
    }

    pub fn delete_next_word(&mut self) {
        self.input.handle(tui_input::InputRequest::DeleteNextWord);
    }

    pub fn delete_prev_word(&mut self) {
        self.input.handle(tui_input::InputRequest::DeletePrevWord);
    }

    pub fn value(&self) -> &str {
        self.input.value()
    }

    pub fn handle(&mut self, event: KeyEvent) {
        match event.code {
            KeyCode::Backspace if event.modifiers == KeyModifiers::CONTROL => {
                self.delete_prev_word();
            }
            KeyCode::Backspace => self.delete_prev(),
            KeyCode::Left if event.modifiers == KeyModifiers::ALT => self.goto_prev_word(),
            KeyCode::Left => self.goto_prev(),
            KeyCode::Right if event.modifiers == KeyModifiers::ALT => self.goto_next_word(),
            KeyCode::Right => self.goto_next(),
            KeyCode::Home => self.goto_start(),
            KeyCode::End => self.goto_end(),
            KeyCode::Delete => self.delete_next(),
            KeyCode::Char(c) => self.insert(c),
            _ => (),
        }
    }
}

#[derive(Debug)]
pub struct Input<'a> {
    block: Option<Block<'a>>,
    hint: Cow<'a, str>,
    style: Style,
    hint_style: Style,
    selection: bool,
}

impl<'a> Input<'a> {
    pub fn block(mut self, block: Block<'a>) -> Self {
        self.block = Some(block);
        self
    }

    pub fn hint(mut self, hint: impl Into<Cow<'a, str>>) -> Self {
        self.hint = hint.into();
        self
    }

    pub fn hint_style(mut self, style: impl Into<Style>) -> Self {
        self.hint_style = style.into();
        self
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn with_show_cursor(mut self, selection: bool) -> Self {
        self.selection = selection;
        self
    }
}

impl<'a> Default for Input<'a> {
    fn default() -> Self {
        Self {
            block: None,
            hint: Default::default(),
            style: theme().text(),
            hint_style: theme().subtext(),
            selection: true,
        }
    }
}

impl StatefulWidget for Input<'_> {
    type State = InputState;

    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        state: &mut Self::State,
    ) {
        // draw block and update area
        let area = if let Some(block) = self.block {
            let new_area = block.inner(area);
            block.render(area, buf);
            new_area
        } else {
            area
        };

        if state.input.value().is_empty() {
            // draw hint
            Paragraph::new(self.hint)
                .style(self.hint_style)
                .render(area, buf);
            // draw cursor
            if self.selection {
                buf.set_style(
                    Rect {
                        x: area.x,
                        y: area.y,
                        width: 1,
                        height: 1,
                    },
                    self.style.add_modifier(Modifier::REVERSED),
                );
            }
        } else {
            // draw text
            let scroll = state
                .input
                .visual_scroll(area.width.saturating_sub(1).into());
            Paragraph::new(state.input.value().chars().skip(scroll).collect::<String>())
                .style(self.style)
                .render(area, buf);
            // draw cursor
            if self.selection {
                buf.set_style(
                    Rect {
                        x: area.x + state.input.visual_cursor().saturating_sub(scroll) as u16,
                        y: area.y,
                        width: 1,
                        height: 1,
                    },
                    self.style.add_modifier(Modifier::REVERSED),
                );
            }
        }
    }
}
