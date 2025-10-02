use ratatui::widgets::StatefulWidget;

use crate::tui::pickers::text_picker::{TextPicker, TextPickerState};

#[derive(Debug)]
pub struct InlineQueryState {
    text_picker: TextPickerState,
    query_type: InlineQueryType,
}

impl InlineQueryState {
    pub fn new(query_type: InlineQueryType) -> Self {
        Self {
            text_picker: TextPickerState::default(),
            query_type,
        }
    }
}

#[derive(Debug, Default)]
pub struct InlineQuery {}

impl StatefulWidget for InlineQuery {
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
            .hint("housing > 900")
            .render(area, buf, &mut state.text_picker);
    }
}

#[derive(Debug)]
pub enum InlineQueryType {
    Filter,
    Order,
}
