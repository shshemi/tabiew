use std::{borrow::Cow, marker::PhantomData};

use ratatui::{
    layout::Alignment,
    text::{Line, Span},
};

use crate::tui::theme::Styler;

pub struct NewStatusBar<Theme> {
    tags: Vec<NewStatusBarTag<Theme>>,
}

impl<Theme> NewStatusBar<Theme> {
    pub fn new() -> Self {
        Self {
            tags: Default::default(),
        }
    }

    pub fn with_tag(mut self, tag: NewStatusBarTag<Theme>) -> Self {
        self.tags.push(tag);
        self
    }

    pub fn with_tags<I: IntoIterator<Item = NewStatusBarTag<Theme>>>(mut self, ext: I) -> Self {
        self.tags.extend(ext);
        self
    }
}

impl<'a, Theme: Styler> From<NewStatusBar<Theme>> for Line<'a> {
    fn from(value: NewStatusBar<Theme>) -> Self {
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

pub struct NewStatusBarTag<Theme> {
    key: Cow<'static, str>,
    value: Cow<'static, str>,
    _theme: PhantomData<Theme>,
}

impl<Theme: Styler> NewStatusBarTag<Theme> {
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
