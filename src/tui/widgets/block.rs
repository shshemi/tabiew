use ratatui::{
    layout::{Alignment, Rect},
    style::Style,
    symbols::border::Set,
    text::Line,
    widgets::{BorderType, Borders, Padding, Widget},
};

use crate::misc::globals::theme;

#[derive(Debug)]
pub struct Block<'a> {
    inner: ratatui::widgets::Block<'a>,
}

impl<'a> Default for Block<'a> {
    fn default() -> Self {
        Self {
            inner: ratatui::widgets::Block::bordered()
                .border_type(BorderType::Rounded)
                .style(theme().block()),
        }
    }
}

impl<'a> Block<'a> {
    pub fn inner(&self, area: Rect) -> Rect {
        self.inner.inner(area)
    }

    pub fn title<T: Into<Line<'a>>>(mut self, title: T) -> Self {
        self.inner = self.inner.title(title);
        self
    }

    pub fn title_alignment(mut self, alignment: Alignment) -> Self {
        self.inner = self.inner.title_alignment(alignment);
        self
    }

    pub fn bottom<T: Into<Line<'a>>>(mut self, bottom: T) -> Self {
        self.inner = self.inner.title_bottom(bottom);
        self
    }

    pub fn padding(mut self, padding: Padding) -> Self {
        self.inner = self.inner.padding(padding);
        self
    }

    pub fn borders(mut self, borders: Borders) -> Self {
        self.inner = self.inner.borders(borders);
        self
    }

    pub fn border_set(mut self, border_set: Set) -> Self {
        self.inner = self.inner.border_set(border_set);
        self
    }

    pub fn style(mut self, style: Style) -> Self {
        self.inner = self.inner.style(style);
        self
    }

    pub fn into_widget(self) -> ratatui::widgets::Block<'a> {
        self.inner
    }
}

impl<'a> Widget for Block<'a> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        self.inner.render(area, buf);
    }
}
