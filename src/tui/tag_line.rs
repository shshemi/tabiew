use std::borrow::Cow;

use ratatui::{
    layout::Alignment,
    style::Modifier,
    text::{Line, Span},
};

use crate::misc::config::theme;

#[derive(Debug)]
pub struct TagLine<'a, 'b> {
    tags: Vec<Tag<'a, 'b>>,
    style: TagLineStyle,
    alignment: Alignment,
}

impl<'a, 'b> TagLine<'a, 'b> {
    pub fn new() -> Self {
        Self {
            tags: Default::default(),
            style: TagLineStyle::MultiColor,
            alignment: Alignment::Right,
        }
    }

    pub fn tag(mut self, tag: Tag<'a, 'b>) -> Self {
        self.tags.push(tag);
        self
    }

    pub fn multi_color(mut self) -> Self {
        self.style = TagLineStyle::MultiColor;
        self
    }

    pub fn mono_color(mut self) -> Self {
        self.style = TagLineStyle::MonoColor;
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

impl Default for TagLine<'_, '_> {
    fn default() -> Self {
        Self::new()
    }
}

impl From<TagLine<'_, '_>> for Line<'_> {
    fn from(value: TagLine) -> Self {
        itertools::intersperse(
            value
                .tags
                .into_iter()
                .enumerate()
                .map(|(idx, tag)| match value.style {
                    TagLineStyle::MultiColor => tag.into_multi_color_span(idx).into_iter(),
                    TagLineStyle::MonoColor => tag.into_mono_color_span().into_iter(),
                }),
            [Span::raw(" "), Span::raw("")].into_iter(),
        )
        .flatten()
        .collect::<Line<'_>>()
        .alignment(value.alignment)
    }
}

#[derive(Debug)]
enum TagLineStyle {
    MultiColor,
    MonoColor,
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
