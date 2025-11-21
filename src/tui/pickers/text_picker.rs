use ratatui::{
    layout::{Constraint, Flex, Layout},
    widgets::{Clear, Widget},
};

use crate::tui::{
    component::Component,
    widgets::{
        block::Block,
        input::{Input, InputType},
    },
};

#[derive(Debug, Default)]
pub struct TextPicker {
    input: Input,
    title: String,
}

impl TextPicker {
    pub fn with_max_len(self, max_len: usize) -> Self {
        Self {
            input: self.input.with_max_len(max_len),
            ..self
        }
    }

    pub fn with_value(self, value: String) -> Self {
        Self {
            input: self.input.with_value(value),
            ..self
        }
    }

    pub fn with_input_type(self, input_type: InputType) -> Self {
        Self {
            input: self.input.with_input_type(input_type),
            ..self
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

impl Component for TextPicker {
    fn render(
        &mut self,
        _: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        focus_state: crate::tui::component::FocusState,
    ) {
        let [area] = Layout::horizontal([Constraint::Length(80)])
            .flex(Flex::Center)
            .areas(buf.area);
        let [_, area] =
            Layout::vertical([Constraint::Length(3), Constraint::Length(3)]).areas(area);
        Widget::render(Clear, area, buf);

        let area = {
            let block = Block::default();
            let inner = block.inner(area);
            block.render(area, buf);
            inner
        };

        self.input.render(area, buf, focus_state);
    }

    fn handle(&mut self, event: crossterm::event::KeyEvent) -> bool {
        self.input.handle(event)
    }
}

// #[derive(Debug, Default)]
// pub struct TextPicker<'a> {
//     block: Block<'a>,
//     hint: Cow<'a, str>,
// }

// impl<'a> TextPicker<'a> {
//     pub fn title<T: Into<Title<'a>>>(mut self, title: T) -> Self {
//         self.block = self.block.title(title);
//         self
//     }

//     pub fn bottom<T: Into<Line<'a>>>(mut self, bottom: T) -> Self {
//         self.block = self.block.bottom(bottom);
//         self
//     }

//     pub fn hint<T: Into<Cow<'a, str>>>(mut self, hint: T) -> Self {
//         self.hint = hint.into();
//         self
//     }
// }

// impl StatefulWidget for TextPicker<'_> {
//     type State = TextPickerState;

//     fn render(
//         self,
//         _: ratatui::prelude::Rect,
//         buf: &mut ratatui::prelude::Buffer,
//         state: &mut Self::State,
//     ) {
//         let [area] = Layout::horizontal([Constraint::Length(80)])
//             .flex(Flex::Center)
//             .areas(buf.area);
//         let [_, area] =
//             Layout::vertical([Constraint::Length(3), Constraint::Length(3)]).areas(area);
//         Widget::render(Clear, area, buf);

//         StatefulWidget::render(
//             Input::default().block(self.block).hint(self.hint),
//             area,
//             buf,
//             &mut state.input,
//         );
//     }
// }
