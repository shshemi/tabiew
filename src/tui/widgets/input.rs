use crate::tui::widgets::block::Block;
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
}

#[derive(Debug, Default)]
pub struct Input<'a> {
    block: Option<Block<'a>>,
    style: Style,
    selection: bool,
}

impl<'a> Input<'a> {
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
            let new_area = block.inner(area);
            block.render(area, buf);
            new_area
        } else {
            area
        };

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
