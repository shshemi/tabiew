use std::borrow::Cow;

use ratatui::{
    layout::Alignment,
    style::Modifier,
    text::{Line, Span},
};

use crate::misc::config::theme;

#[derive(Debug)]
pub enum StatusBarStyle {
    MultiColor,
    MonoColor,
}

#[derive(Debug)]
pub struct StatusBar<'a, 'b> {
    tags: Vec<Tag<'a, 'b>>,
    style: StatusBarStyle,
    alignment: Alignment,
}

impl<'a, 'b> StatusBar<'a, 'b> {
    pub fn new() -> Self {
        Self {
            tags: Default::default(),
            style: StatusBarStyle::MultiColor,
            alignment: Alignment::Right,
        }
    }

    pub fn tag(mut self, tag: Tag<'a, 'b>) -> Self {
        self.tags.push(tag);
        self
    }

    pub fn multi_color(mut self) -> Self {
        self.style = StatusBarStyle::MultiColor;
        self
    }

    pub fn mono_color(mut self) -> Self {
        self.style = StatusBarStyle::MonoColor;
        self
    }

    pub fn centered(mut self) -> Self {
        self.alignment = Alignment::Center;
        self
    }

    pub fn right_aligned(mut self) -> Self {
        self.alignment = Alignment::Right;
        self
    }

    pub fn left_aligned(mut self) -> Self {
        self.alignment = Alignment::Left;
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
                    .map(|(idx, tag)| match value.style {
                        StatusBarStyle::MultiColor => tag.into_multi_color_span(idx).into_iter(),
                        StatusBarStyle::MonoColor => tag.into_mono_color_span().into_iter(),
                    }),
                [Span::raw(" "), Span::raw("")].into_iter(),
            )
            .flatten(),
        )
        .alignment(value.alignment)
    }
}

#[derive(Debug)]
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

    fn into_multi_color_span(self, pos: usize) -> [Span<'static>; 2] {
        [
            Span::raw(format!(" {} ", self.key)).style(theme().tag(pos)),
            Span::raw(format!(" {} ", self.value))
                .style(theme().tag(pos).add_modifier(Modifier::REVERSED)),
        ]
    }

    fn into_mono_color_span(self) -> [Span<'static>; 2] {
        [
            Span::raw(format!(" {} ", self.key)).style(theme().block_tag()),
            Span::raw(format!(" {} ", self.value))
                .style(theme().block_tag().add_modifier(Modifier::REVERSED)),
        ]
    }
}
