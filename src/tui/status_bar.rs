use std::borrow::Cow;

use ratatui::{
    layout::Alignment,
    style::Modifier,
    text::{Line, Span},
};

use crate::misc::globals::theme;

pub struct StatusBar<'a, 'b> {
    tags: Vec<Tag<'a, 'b>>,
}

impl<'a, 'b> StatusBar<'a, 'b> {
    pub fn new() -> Self {
        Self {
            tags: Default::default(),
        }
    }

    pub fn tag(mut self, tag: Tag<'a, 'b>) -> Self {
        self.tags.push(tag);
        self
    }
}

impl Default for StatusBar<'_, '_> {
    fn default() -> Self {
        Self::new()
    }
}

impl From<StatusBar<'_, '_>> for Line<'_> {
    fn from(value: StatusBar) -> Self {
        Line::from_iter(
            itertools::intersperse(
                value
                    .tags
                    .into_iter()
                    .enumerate()
                    .map(|(idx, tag)| tag.into_span(idx).into_iter()),
                [Span::raw(" "), Span::raw("")].into_iter(),
            )
            .flatten(),
        )
        .alignment(Alignment::Right)
    }
}

pub struct Tag<'a, 'b> {
    key: Cow<'a, str>,
    value: Cow<'b, str>,
}

impl<'a, 'b> Tag<'a, 'b> {
    pub fn new(key: impl Into<Cow<'a, str>>, value: impl Into<Cow<'b, str>>) -> Self {
        Self {
            key: key.into(),
            value: value.into(),
        }
    }

    fn into_span(self, pos: usize) -> [Span<'static>; 2] {
        [
            Span::raw(format!(" {} ", self.key)).style(theme().tag(pos)),
            Span::raw(format!(" {} ", self.value))
                .style(theme().tag(pos).add_modifier(Modifier::REVERSED)),
        ]
    }
}
