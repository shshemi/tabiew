use ratatui::widgets::{Block, Widget};

use crate::{
    misc::{buffer_ext::BufferExt, rect_ext::RectExt},
    tui::{
        app_default::{AppDefault, AppTitle},
        component::Component,
        widgets::input::{Input, InputType},
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

    pub fn with_title(self, title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            ..self
        }
    }

    pub fn with_hint(self, hint: impl Into<String>) -> Self {
        Self {
            input: self.input.with_hint(hint.into()),
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
        let area = buf.area.palette(3);
        buf.clear(area);

        let area = {
            let block = Block::app_default().app_title(self.title.as_str());
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
