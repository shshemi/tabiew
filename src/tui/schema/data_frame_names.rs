use crossterm::event::{KeyCode, KeyModifiers};
use ratatui::{
    layout::{Alignment, Constraint},
    text::Span,
    widgets::{Row, StatefulWidget, Table, TableState},
};

use crate::{
    misc::{config::theme, globals::sql},
    tui::{
        component::Component,
        status_bar::{StatusBar, Tag},
        widgets::block::Block,
    },
};

#[derive(Debug)]
pub struct DataFrameNames {
    table: TableState,
}

impl DataFrameNames {
    pub fn selected(&self) -> Option<usize> {
        self.table.selected()
    }

    fn select_up(&mut self) {
        self.table.select_previous();
    }

    fn select_down(&mut self) {
        self.table.select_next();
    }

    fn select_first(&mut self) {
        self.table.select_first();
    }

    fn select_last(&mut self) {
        self.table.select_last();
    }
}

impl Default for DataFrameNames {
    fn default() -> Self {
        Self {
            table: TableState::default().with_selected(0),
        }
    }
}

impl Component for DataFrameNames {
    fn render(
        &mut self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        focus_state: crate::tui::component::FocusState,
    ) {
        let num_width = sql().schema().len().to_string().len();

        let table = Table::default()
            .rows(sql().schema().iter().enumerate().map(|(i, (s, _))| {
                Row::new([
                    Span::raw(format!(" {:>width$}", i + 1, width = num_width))
                        .style(theme().subtext()),
                    Span::raw(s.to_owned()).style(theme().text()),
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
            );
        if focus_state.is_focused() {
            table.render(area, buf, &mut self.table);
        } else {
            table.render(area, buf, &mut self.table.clone().with_selected(None));
        }
    }

    fn handle(&mut self, event: crossterm::event::KeyEvent) -> bool {
        match (event.code, event.modifiers) {
            (KeyCode::Up, KeyModifiers::NONE)
            | (KeyCode::Char('k'), KeyModifiers::NONE)
            | (KeyCode::Char('p'), KeyModifiers::CONTROL) => {
                self.select_up();
                true
            }
            (KeyCode::Down, KeyModifiers::NONE)
            | (KeyCode::Char('j'), KeyModifiers::NONE)
            | (KeyCode::Char('n'), KeyModifiers::CONTROL) => {
                self.select_down();
                true
            }
            (KeyCode::Home, KeyModifiers::NONE) | (KeyCode::Char('g'), KeyModifiers::NONE) => {
                self.select_first();
                true
            }
            (KeyCode::End, KeyModifiers::NONE) | (KeyCode::Char('G'), KeyModifiers::SHIFT) => {
                self.select_last();
                true
            }
            (KeyCode::Delete, KeyModifiers::NONE) => {
                if let Some(name) = self
                    .selected()
                    .and_then(|idx| sql().schema().get_by_index(idx).map(|(s, _)| s.to_owned()))
                {
                    sql().unregister(&name);
                }
                true
            }

            _ => false,
        }
    }
}
