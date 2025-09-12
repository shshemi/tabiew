use std::borrow::Cow;

use ratatui::{
    layout::{Alignment, Constraint},
    text::Span,
    widgets::{Row, StatefulWidget, Table, TableState},
};

use crate::{
    misc::globals::theme,
    tui::{
        status_bar::{StatusBar, Tag},
        widgets::block::Block,
    },
};

#[derive(Debug)]
pub struct DataFrameNamesState {
    table: TableState,
}

impl Default for DataFrameNamesState {
    fn default() -> Self {
        Self {
            table: TableState::default().with_selected(0),
        }
    }
}

impl DataFrameNamesState {
    pub fn table(&self) -> &TableState {
        &self.table
    }

    pub fn table_mut(&mut self) -> &mut TableState {
        &mut self.table
    }
}
pub struct DataFrameNames<I> {
    names: I,
}

impl<T> DataFrameNames<T> {
    pub fn new(names: T) -> Self {
        DataFrameNames { names }
    }
}

impl<'a, I> StatefulWidget for DataFrameNames<I>
where
    I: IntoIterator,
    I::Item: Into<Cow<'a, str>>,
{
    type State = DataFrameNamesState;

    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        state: &mut Self::State,
    ) {
        let items = self.names.into_iter().collect::<Vec<_>>();
        let num_width = items.len().to_string().len();

        Table::default()
            .rows(items.into_iter().enumerate().map(|(i, s)| {
                Row::new([
                    Span::raw(format!(" {:>width$}", i + 1, width = num_width))
                        .style(theme().subtext()),
                    Span::raw(s.into()).style(theme().text()),
                ])
            }))
            .row_highlight_style(theme().highlight())
            .widths([
                Constraint::Length(num_width as u16 + 1),
                Constraint::Fill(1),
            ])
            .column_spacing(1)
            .block(
                Block::default()
                    .title("Tables")
                    .bottom(
                        StatusBar::new()
                            .mono_color()
                            .centered()
                            .tag(Tag::new(" Open ", " Enter"))
                            .tag(Tag::new(" Unload ", " Delete ")),
                    )
                    .title_alignment(Alignment::Center)
                    .into_widget(),
            )
            .render(area, buf, &mut state.table);
    }
}
