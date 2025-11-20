use crossterm::event::KeyEvent;
use ratatui::widgets::StatefulWidget;

use crate::tui::pickers::text_picker::{TextPicker, TextPicker};

#[derive(Debug)]
pub struct InlineQueryState {
    text_picker: TextPicker,
    query_type: InlineQueryType,
}

impl InlineQueryState {
    pub fn new(query_type: InlineQueryType) -> Self {
        Self {
            text_picker: TextPicker::default(),
            query_type,
        }
    }

    pub fn handle(&mut self, event: KeyEvent) {
        self.text_picker.input_mut().handle(event);
    }

    pub fn value(&self) -> &str {
        self.text_picker.input().value()
    }

    pub fn query_type(&self) -> &InlineQueryType {
        &self.query_type
    }
}

#[derive(Debug, Default)]
pub struct InlineQuery<'a> {
    hint: &'a str,
}

impl<'a> StatefulWidget for InlineQuery<'a> {
    type State = InlineQueryState;

    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        state: &mut Self::State,
    ) {
        TextPicker::default()
            .title(match state.query_type {
                InlineQueryType::Filter => "Filter",
                InlineQueryType::Order => "Order",
            })
            .hint(self.hint)
            .render(area, buf, &mut state.text_picker);
    }
}

#[derive(Debug, Clone, Copy)]
pub enum InlineQueryType {
    Filter,
    Order,
}
