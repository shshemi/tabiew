use std::borrow::Cow;

use ratatui::{
    layout::{Constraint, Flex, Layout},
    text::Line,
    widgets::{Clear, StatefulWidget, Widget, block::Title},
};

use crate::tui::widgets::{
    block::Block,
    input::{Input, Input, InputType},
};

#[derive(Debug, Default)]
pub struct TextPickerState {
    input: Input,
}

impl TextPickerState {
    pub fn with_max_len(self, max_len: usize) -> Self {
        Self {
            input: self.input.with_max_len(max_len),
        }
    }

    pub fn with_value(self, value: String) -> Self {
        Self {
            input: self.input.with_value(value),
        }
    }

    pub fn with_input_type(self, input_type: InputType) -> Self {
        Self {
            input: self.input.with_input_type(input_type),
        }
    }

    pub fn input(&self) -> &Input {
        &self.input
    }

    pub fn input_mut(&mut self) -> &mut Input {
        &mut self.input
    }

    pub fn value(&self) -> &str {
        self.input.value()
    }
}

#[derive(Debug, Default)]
pub struct TextPicker<'a> {
    block: Block<'a>,
    hint: Cow<'a, str>,
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

    pub fn hint<T: Into<Cow<'a, str>>>(mut self, hint: T) -> Self {
        self.hint = hint.into();
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
            Input::default().block(self.block).hint(self.hint),
            area,
            buf,
            &mut state.input,
        );
    }
}
