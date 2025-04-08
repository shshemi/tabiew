use std::borrow::Cow;

use ratatui::{
    layout::Alignment,
    style::Modifier,
    text::{Line, Span},
};

use crate::misc::globals::theme;

pub struct StatusBar {
    tags: Vec<StatusBarTag>,
}

impl StatusBar {
    pub fn new() -> Self {
        Self {
            tags: Default::default(),
        }
    }

    pub fn with_tag(mut self, tag: StatusBarTag) -> Self {
        self.tags.push(tag);
        self
    }

    pub fn with_tags<I: IntoIterator<Item = StatusBarTag>>(mut self, ext: I) -> Self {
        self.tags.extend(ext);
        self
    }
}

impl Default for StatusBar {
    fn default() -> Self {
        Self::new()
    }
}

impl From<StatusBar> for Line<'_> {
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

pub struct StatusBarTag {
    key: Cow<'static, str>,
    value: Cow<'static, str>,
}

impl StatusBarTag {
    pub fn new(key: impl Into<Cow<'static, str>>, value: impl Into<Cow<'static, str>>) -> Self {
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
