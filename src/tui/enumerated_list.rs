use ratatui::{
    layout::{Constraint, Rect},
    text::Text,
    widgets::{Cell, Clear, Row, StatefulWidget, Table, TableState, Widget, block::Title},
};

use crate::{
    misc::globals::theme,
    tui::{themes::styler::Styler, widgets::block::Block},
};

#[derive(Debug)]
pub struct EnumeratedListState {
    list: TableState,
}

impl EnumeratedListState {
    pub fn new(selected: usize) -> EnumeratedListState {
        Self {
            list: TableState::default().with_selected(Some(selected)),
        }
    }
}

impl EnumeratedListState {
    pub fn list(&self) -> &TableState {
        &self.list
    }

    pub fn list_mut(&mut self) -> &mut TableState {
        &mut self.list
    }
}

#[derive(Debug, Default)]
pub struct EnumeratedList<'a> {
    items: Vec<Text<'a>>,
    block: Block<'a>,
}

impl<'a> EnumeratedList<'a> {
    pub fn title<T: Into<Title<'a>>>(mut self, title: T) -> Self {
        self.block = self.block.title(title);
        self
    }

    pub fn items<T>(mut self, items: T) -> Self
    where
        T: IntoIterator,
        T::Item: Into<Text<'a>>,
    {
        self.items = items.into_iter().map(Into::into).collect();
        self
    }
}

impl StatefulWidget for EnumeratedList<'_> {
    type State = EnumeratedListState;

    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        state: &mut Self::State,
    ) {
        let num_width = (self.items.len().ilog10() + 1) as u16;
        let text_width = self
            .items
            .iter()
            .map(|s| s.width() as u16)
            .max()
            .map(|w| w.clamp(34, area.width.saturating_div(2)))
            .unwrap_or(34);
        let width = num_width + text_width + 3;
        let area = Rect::new(area.x + area.width - width, area.y, width, area.height);

        Widget::render(Clear, area, buf);

        let rows = self.items.into_iter().enumerate().map(|(i, s)| {
            Row::new([
                Cell::new(format!(" {:>width$}", i + 1, width = num_width as usize))
                    .style(theme().subtext()),
                Cell::new(s).style(theme().text()),
            ])
        });
        StatefulWidget::render(
            Table::default()
                .rows(rows)
                .style(theme().text())
                .row_highlight_style(theme().row_highlighted())
                .widths([
                    Constraint::Length(num_width + 1),
                    Constraint::Length(text_width),
                ])
                .column_spacing(1)
                .block(self.block.into_widget()),
            area,
            buf,
            &mut state.list,
        );
    }
}
