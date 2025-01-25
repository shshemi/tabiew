use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    widgets::{Block, Paragraph, StatefulWidget, Widget},
};
use std::marker::PhantomData;

use super::Styler;

#[derive(Debug, Default)]
pub struct InputState {
    input: tui_input::Input,
    scroll: usize,
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

    pub fn value(&self) -> &str {
        self.input.value()
    }
}

#[derive(Debug)]
pub struct Input<'a, Theme> {
    scroll_pad: u16,
    block: Option<Block<'a>>,
    style: Style,
    selection: bool,
    _theme: PhantomData<Theme>,
}

impl<T> Input<'_, T> {
    pub fn new() -> Self {
        Input::<T> {
            scroll_pad: 4,
            block: Default::default(),
            style: Default::default(),
            selection: false,
            _theme: Default::default(),
        }
    }
}

impl<'a, T> Input<'a, T> {
    pub fn scroll_pad(mut self, pad: u16) -> Self {
        self.scroll_pad = pad;
        self
    }

    pub fn block(mut self, block: Block<'a>) -> Self {
        self.block = Some(block);
        self
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn selection(mut self, selection: bool) -> Self {
        self.selection = selection;
        self
    }
}

impl<'a, Theme: Styler> StatefulWidget for Input<'a, Theme> {
    type State = InputState;

    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        state: &mut Self::State,
    ) {
        // calculate scroll to stay between locks
        state.scroll = {
            let input_len = state.input.value().chars().count();
            let cursor = state.input.visual_cursor();
            let min_scroll = if input_len - cursor < self.scroll_pad as usize {
                cursor.saturating_sub(area.width as usize)
            } else {
                cursor
                    .saturating_add(self.scroll_pad as usize)
                    .saturating_sub(area.width as usize)
            };
            let max_scroll = cursor.saturating_sub(self.scroll_pad as usize);
            state.scroll.clamp(min_scroll, max_scroll)
        };

        // calculate text space
        let text_area = self
            .block
            .as_ref()
            .map(|block| block.inner(area))
            .unwrap_or(area);

        // draw block
        if let Some(block) = self.block {
            block.render(area, buf);
        }

        // draw text
        Paragraph::new(
            state
                .input
                .value()
                .chars()
                .skip(state.scroll)
                .collect::<String>(),
        )
        .style(self.style)
        .render(text_area, buf);

        // draw cursor
        if self.selection {
            buf.set_style(
                Rect {
                    x: text_area.x + (state.input.visual_cursor() - state.scroll) as u16,
                    y: text_area.y,
                    width: 1,
                    height: 1,
                },
                self.style.add_modifier(Modifier::REVERSED),
            );
        }
    }
}
