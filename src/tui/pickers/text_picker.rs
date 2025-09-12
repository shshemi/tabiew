use ratatui::{
    layout::{Constraint, Flex, Layout},
    text::Line,
    widgets::{Clear, StatefulWidget, Widget, block::Title},
};

use crate::tui::widgets::{
    block::Block,
    input::{Input, InputState},
};

#[derive(Debug, Default)]
pub struct TextPickerState {
    input: InputState,
}

impl TextPickerState {
    pub fn input(&self) -> &InputState {
        &self.input
    }

    pub fn input_mut(&mut self) -> &mut InputState {
        &mut self.input
    }
}

#[derive(Debug, Default)]
pub struct TextPicker<'a> {
    block: Block<'a>,
}

impl<'a> TextPicker<'a> {
    pub fn title<T: Into<Title<'a>>>(mut self, title: T) -> Self {
        self.block = self.block.title(title);
        self
    }

    pub fn bottom<T: Into<Line<'a>>>(mut self, bottom: T) -> Self {
        self.block = self.block.bottom(bottom);
        self
    }
}

impl StatefulWidget for TextPicker<'_> {
    type State = TextPickerState;

    fn render(
        self,
        _: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        state: &mut Self::State,
    ) {
        let [area] = Layout::horizontal([Constraint::Length(80)])
            .flex(Flex::Center)
            .areas(buf.area);
        let [_, area] =
            Layout::vertical([Constraint::Length(3), Constraint::Length(3)]).areas(area);
        Widget::render(Clear, area, buf);

        StatefulWidget::render(
            Input::default().block(self.block),
            area,
            buf,
            &mut state.input,
        );
    }
}
