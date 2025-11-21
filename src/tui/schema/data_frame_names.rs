use ratatui::{
    layout::{Alignment, Constraint},
    text::Span,
    widgets::{Row, StatefulWidget, Table, TableState},
};

use crate::{
    misc::globals::{sql, theme},
    tui::{
        component::Component,
        status_bar::{StatusBar, Tag},
        widgets::block::Block,
    },
};

#[derive(Debug)]
pub struct DataFrameNames {
    table: TableState,
    items: Vec<String>,
}

impl DataFrameNames {
    pub fn selected(&self) -> Option<usize> {
        self.table.selected()
    }
    pub fn table(&self) -> &TableState {
        &self.table
    }

    pub fn table_mut(&mut self) -> &mut TableState {
        &mut self.table
    }
}

impl Component for DataFrameNames {
    fn render(
        &mut self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        _focus_state: crate::tui::component::FocusState,
    ) {
        let num_width = self.items.len().to_string().len();

        Table::default()
            .rows(self.items.iter().enumerate().map(|(i, s)| {
                Row::new([
                    Span::raw(format!(" {:>width$}", i + 1, width = num_width))
                        .style(theme().subtext()),
                    Span::raw(s).style(theme().text()),
                ])
            }))
            .row_highlight_style(theme().row_highlighted())
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
            .render(area, buf, &mut self.table);
    }
}

impl Default for DataFrameNames {
    fn default() -> Self {
        Self {
            table: TableState::default().with_selected(0),
            items: sql()
                .schema()
                .iter()
                .map(|(name, _)| name.to_owned())
                .collect(),
        }
    }
}
// pub struct DataFrameNames<I> {
//     names: I,
// }

// impl<T> DataFrameNames<T> {
//     pub fn new(names: T) -> Self {
//         DataFrameNames { names }
//     }
// }

// impl<'a, I> StatefulWidget for DataFrameNames<I>
// where
//     I: IntoIterator,
//     I::Item: Into<Cow<'a, str>>,
// {
//     type State = DataFrameNamesState;

//     fn render(
//         self,
//         area: ratatui::prelude::Rect,
//         buf: &mut ratatui::prelude::Buffer,
//         state: &mut Self::State,
//     ) {
//         let items = self.names.into_iter().collect::<Vec<_>>();
//         let num_width = items.len().to_string().len();

//         Table::default()
//             .rows(items.into_iter().enumerate().map(|(i, s)| {
//                 Row::new([
//                     Span::raw(format!(" {:>width$}", i + 1, width = num_width))
//                         .style(theme().subtext()),
//                     Span::raw(s.into()).style(theme().text()),
//                 ])
//             }))
//             .row_highlight_style(theme().row_highlighted())
//             .widths([
//                 Constraint::Length(num_width as u16 + 1),
//                 Constraint::Fill(1),
//             ])
//             .column_spacing(1)
//             .block(
//                 Block::default()
//                     .title("Tables")
//                     .bottom(
//                         StatusBar::new()
//                             .mono_color()
//                             .centered()
//                             .tag(Tag::new(" Open ", " Enter"))
//                             .tag(Tag::new(" Unload ", " Delete ")),
//                     )
//                     .title_alignment(Alignment::Center)
//                     .into_widget(),
//             )
//             .render(area, buf, &mut state.table);
//     }
// }
