use crossterm::event::KeyCode;
use ratatui::{
    layout::{Constraint, Rect},
    widgets::{Cell, Clear, Row, StatefulWidget, Table, TableState, Widget},
};
use unicode_width::UnicodeWidthStr;

use crate::{
    misc::globals::theme,
    tui::{component::Component, widgets::block::Block},
};

#[derive(Debug)]
pub struct EnumeratedList {
    items: Vec<String>,
    title: String,
    list: TableState,
}

impl EnumeratedList {
    pub fn new(title: impl Into<String>, items: Vec<String>) -> EnumeratedList {
        Self {
            list: TableState::default(),
            items,
            title: title.into(),
        }
    }

    pub fn with_selected(self, selected: usize) -> EnumeratedList {
        Self {
            list: self.list.with_selected(selected),
            ..self
        }
    }

    pub fn selected(&self) -> Option<usize> {
        self.list.selected()
    }
}

impl Component for EnumeratedList {
    fn render(
        &mut self,
        area: Rect,
        buf: &mut ratatui::prelude::Buffer,
        _focus_state: super::component::FocusState,
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

        let rows = self.items.iter().enumerate().map(|(i, s)| {
            Row::new([
                Cell::new(format!(" {:>width$}", i + 1, width = num_width as usize))
                    .style(theme().subtext()),
                Cell::new(s.as_str()).style(theme().text()),
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
                .block(Block::default().title(self.title.as_str()).into_widget()),
            area,
            buf,
            &mut self.list,
        );
    }

    fn handle(&mut self, event: crossterm::event::KeyEvent) -> bool {
        match event.code {
            KeyCode::Up => {
                self.list.select_previous();
                true
            }
            KeyCode::Down => {
                self.list.select_next();
                true
            }
            KeyCode::Home => {
                self.list.select_first();
                true
            }
            KeyCode::End => {
                self.list.select_last();
                true
            }
            KeyCode::Char('k') => {
                self.list.select_previous();
                true
            }
            KeyCode::Char('j') => {
                self.list.select_next();
                true
            }
            _ => false,
        }
    }
}

// #[derive(Debug, Default)]
// pub struct EnumeratedList<'a> {
//     items: Vec<Text<'a>>,
//     block: Block<'a>,
// }

// impl<'a> EnumeratedList<'a> {
//     pub fn title<T: Into<Title<'a>>>(mut self, title: T) -> Self {
//         self.block = self.block.title(title);
//         self
//     }

//     pub fn items<T>(mut self, items: T) -> Self
//     where
//         T: IntoIterator,
//         T::Item: Into<Text<'a>>,
//     {
//         self.items = items.into_iter().map(Into::into).collect();
//         self
//     }
// }

// impl StatefulWidget for EnumeratedList<'_> {
//     type State = EnumeratedListState;

//     fn render(
//         self,
//         area: ratatui::prelude::Rect,
//         buf: &mut ratatui::prelude::Buffer,
//         state: &mut Self::State,
//     ) {
//         let num_width = (self.items.len().ilog10() + 1) as u16;
//         let text_width = self
//             .items
//             .iter()
//             .map(|s| s.width() as u16)
//             .max()
//             .map(|w| w.clamp(34, area.width.saturating_div(2)))
//             .unwrap_or(34);
//         let width = num_width + text_width + 3;
//         let area = Rect::new(area.x + area.width - width, area.y, width, area.height);

//         Widget::render(Clear, area, buf);

//         let rows = self.items.into_iter().enumerate().map(|(i, s)| {
//             Row::new([
//                 Cell::new(format!(" {:>width$}", i + 1, width = num_width as usize))
//                     .style(theme().subtext()),
//                 Cell::new(s).style(theme().text()),
//             ])
//         });
//         StatefulWidget::render(
//             Table::default()
//                 .rows(rows)
//                 .style(theme().text())
//                 .row_highlight_style(theme().row_highlighted())
//                 .widths([
//                     Constraint::Length(num_width + 1),
//                     Constraint::Length(text_width),
//                 ])
//                 .column_spacing(1)
//                 .block(self.block.into_widget()),
//             area,
//             buf,
//             &mut state.list,
//         );
//     }
// }
