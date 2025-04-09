use std::borrow::Cow;

use ratatui::{
    layout::{Alignment, Constraint},
    style::{Modifier, Stylize},
    symbols::{
        border::{ROUNDED, Set},
        line::{HORIZONTAL_DOWN, HORIZONTAL_UP},
    },
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Row, StatefulWidget, Table, TableState},
};

use crate::misc::globals::theme;

#[derive(Debug)]
pub struct TableNamesTableState {
    table: TableState,
}

impl Default for TableNamesTableState {
    fn default() -> Self {
        Self {
            table: TableState::default().with_selected(0),
        }
    }
}

impl TableNamesTableState {
    pub fn table(&self) -> &TableState {
        &self.table
    }

    pub fn table_mut(&mut self) -> &mut TableState {
        &mut self.table
    }
}
pub struct TableNamesTable<I> {
    names: I,
}

impl<T> TableNamesTable<T> {
    pub fn new(names: T) -> Self {
        TableNamesTable { names }
    }
}

impl<'a, I> StatefulWidget for TableNamesTable<I>
where
    I: IntoIterator,
    I::Item: Into<Cow<'a, str>>,
{
    type State = TableNamesTableState;

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
                Block::new()
                    .borders(Borders::LEFT | Borders::TOP | Borders::BOTTOM)
                    .border_type(BorderType::Rounded)
                    .border_set(Set {
                        top_right: HORIZONTAL_DOWN,
                        bottom_right: HORIZONTAL_UP,
                        ..ROUNDED
                    })
                    .border_style(theme().block())
                    .title("Tables")
                    .title_bottom(Line::from_iter([
                        Span::raw("Up "),
                        Span::raw(" K ").add_modifier(Modifier::REVERSED),
                        Span::raw(" "),
                        Span::raw(" \u{2191} ").add_modifier(Modifier::REVERSED),
                        Span::raw(" ─── "),
                        Span::raw("Down "),
                        Span::raw(" J ").add_modifier(Modifier::REVERSED),
                        Span::raw(" "),
                        Span::raw(" \u{2193} ").add_modifier(Modifier::REVERSED),
                    ]))
                    .title_alignment(Alignment::Center),
            )
            .render(area, buf, &mut state.table);
    }
}
