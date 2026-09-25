use itertools::Itertools;
use ratatui::text::{Span, Text};

use crate::tui::misc::line_builder::LineBuilder;

pub struct TextBuilder {
    builders: Vec<LineBuilder>,
}

impl TextBuilder {
    pub fn new() -> Self {
        TextBuilder {
            builders: vec![LineBuilder::default()],
        }
    }

    pub fn push_span(&mut self, span: Span<'static>) {
        if let Some(last) = self.builders.last_mut() {
            last.push_span(span);
        }
    }

    pub fn new_line(&mut self) {
        self.builders.push(Default::default());
    }

    pub fn into_text(self) -> Text<'static> {
        self.builders
            .into_iter()
            .map(|lb| lb.into_line())
            .collect_vec()
            .into()
    }
}

impl Default for TextBuilder {
    fn default() -> Self {
        Self::new()
    }
}
