use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    widgets::{Block, Paragraph, StatefulWidget, Widget},
};
use std::ops::Add;

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
pub struct Input<'a> {
    scroll_pad: u16,
    block: Option<Block<'a>>,
    style: Style,
    selection: bool,
}

impl Input<'_> {
    pub fn new() -> Self {
        Input {
            scroll_pad: 4,
            block: Default::default(),
            style: Default::default(),
            selection: false,
        }
    }
}

impl Default for Input<'_> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> Input<'a> {
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
            (&block).render(area, buf);
            block.inner(area)
        } else {
            area
        };

        // calculate scroll to stay between locks
        state.scroll = {
            let input_len = state.input.value().chars().count();
            let cursor = state.input.visual_cursor();
            let pad = self.scroll_pad as usize;
            let width = area.width as usize;
            let min_scroll = if input_len - cursor < pad {
                cursor.add(1).saturating_sub(width)
            } else {
                cursor.add(1).add(pad).saturating_sub(width)
            };
            let max_scroll = cursor.saturating_sub(pad);
            state.scroll.clamp(min_scroll, max_scroll)
        };

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
        .render(area, buf);

        // draw cursor
        if self.selection {
            buf.set_style(
                Rect {
                    x: area.x + (state.input.visual_cursor() - state.scroll) as u16,
                    y: area.y,
                    width: 1,
                    height: 1,
                },
                self.style.add_modifier(Modifier::REVERSED),
            );
        }
    }
}
