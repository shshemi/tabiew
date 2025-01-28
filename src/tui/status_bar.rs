use std::{borrow::Cow, marker::PhantomData};

use ratatui::{
    layout::Alignment,
    text::{Line, Span},
};

use crate::tui::theme::Styler;

pub struct StatusBar<Theme> {
    tags: Vec<StatusBarTag<Theme>>,
}

impl<Theme> StatusBar<Theme> {
    pub fn new() -> Self {
        Self {
            tags: Default::default(),
        }
    }

    pub fn with_tag(mut self, tag: StatusBarTag<Theme>) -> Self {
        self.tags.push(tag);
        self
    }

    pub fn with_tags<I: IntoIterator<Item = StatusBarTag<Theme>>>(mut self, ext: I) -> Self {
        self.tags.extend(ext);
        self
    }
}

impl<Theme> Default for StatusBar<Theme> {
    fn default() -> Self {
        Self::new()
    }
}

impl<Theme: Styler> From<StatusBar<Theme>> for Line<'_> {
    fn from(value: StatusBar<Theme>) -> Self {
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

pub struct StatusBarTag<Theme> {
    key: Cow<'static, str>,
    value: Cow<'static, str>,
    _theme: PhantomData<Theme>,
}

impl<Theme: Styler> StatusBarTag<Theme> {
    pub fn new(key: impl Into<Cow<'static, str>>, value: impl Into<Cow<'static, str>>) -> Self {
        Self {
            key: key.into(),
            value: value.into(),
            _theme: Default::default(),
        }
    }

    fn into_span(self, pos: usize) -> [Span<'static>; 2] {
        [
            Span::raw(format!(" {} ", self.key)).style(Theme::status_bar_info_key(pos)),
            Span::raw(format!(" {} ", self.value)).style(Theme::status_bar_info_val(pos)),
        ]
    }
}
