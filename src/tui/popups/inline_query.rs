use crossterm::event::KeyEvent;

use crate::tui::{component::Component, pickers::text_picker::TextPicker};

#[derive(Debug)]
pub struct InlineQuery {
    text_picker: TextPicker,
    query_type: InlineQueryType,
}

impl InlineQuery {
    pub fn new(query_type: InlineQueryType) -> Self {
        Self {
            text_picker: TextPicker::default().with_title(
                match query_type {
                    InlineQueryType::Filter => "Filter",
                    InlineQueryType::Order => "Order",
                }
                .to_owned(),
            ),
            query_type,
        }
    }

    pub fn value(&self) -> &str {
        self.text_picker.input().value()
    }

    pub fn query_type(&self) -> &InlineQueryType {
        &self.query_type
    }
}

impl Component for InlineQuery {
    fn render(
        &mut self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        focus_state: crate::tui::component::FocusState,
    ) {
        self.text_picker.render(area, buf, focus_state);
    }
    fn handle(&mut self, event: KeyEvent) -> bool {
        self.text_picker.handle(event)
    }
}

// #[derive(Debug, Default)]
// pub struct InlineQuery<'a> {
//     hint: &'a str,
// }

// impl<'a> StatefulWidget for InlineQuery<'a> {
//     type State = InlineQueryState;

//     fn render(
//         self,
//         area: ratatui::prelude::Rect,
//         buf: &mut ratatui::prelude::Buffer,
//         state: &mut Self::State,
//     ) {
//         TextPicker::default()
//             .title(match state.query_type {
//                 InlineQueryType::Filter => "Filter",
//                 InlineQueryType::Order => "Order",
//             })
//             .hint(self.hint)
//             .render(area, buf, &mut state.text_picker);
//     }
// }

#[derive(Debug, Clone, Copy)]
pub enum InlineQueryType {
    Filter,
    Order,
}
