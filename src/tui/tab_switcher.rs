use crossterm::event::{KeyCode, KeyModifiers};
use ratatui::{
    layout::{Constraint, Rect},
    widgets::{Cell, Clear, Row, StatefulWidget, Table, TableState, Widget},
};
use unicode_width::UnicodeWidthStr;

use crate::{
    handler::message::Message,
    misc::globals::theme,
    tui::{component::Component, widgets::block::Block},
};

#[derive(Debug)]
pub struct TabSwitcher {
    items: Vec<String>,
    title: String,
    list_state: TableState,
    rollback: usize,
}

impl TabSwitcher {
    pub fn new(title: impl Into<String>, items: Vec<String>, idx: usize) -> TabSwitcher {
        Self {
            list_state: TableState::default().with_selected(idx),
            items,
            title: title.into(),
            rollback: idx,
        }
    }

    pub fn selected(&self) -> Option<usize> {
        self.list_state.selected()
    }

    pub fn select_prev(&mut self) {
        self.list_state.select_previous();
    }

    pub fn select_next(&mut self) {
        let idx = self
            .list_state
            .selected()
            .unwrap_or_default()
            .saturating_add(1)
            .min(self.items.len().saturating_sub(1));
        self.list_state.select(Some(idx));
    }

    pub fn select_first(&mut self) {
        self.list_state.select(Some(0));
    }
    pub fn select_last(&mut self) {
        self.list_state
            .select(Some(self.items.len().saturating_sub(1)));
    }
}

impl Component for TabSwitcher {
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
            &mut self.list_state,
        );
    }

    fn handle(&mut self, event: crossterm::event::KeyEvent) -> bool {
        match (event.code, event.modifiers) {
            (KeyCode::Up, KeyModifiers::NONE) | (KeyCode::Char('k'), KeyModifiers::NONE) => {
                self.select_prev();
                if let Some(select) = self.list_state.selected() {
                    Message::TabsSelect(select).enqueue();
                }
                true
            }
            (KeyCode::Down, KeyModifiers::NONE) | (KeyCode::Char('j'), KeyModifiers::NONE) => {
                self.select_next();
                if let Some(select) = self.list_state.selected() {
                    Message::TabsSelect(select).enqueue();
                }
                true
            }
            (KeyCode::Home, KeyModifiers::NONE) | (KeyCode::Char('g'), KeyModifiers::NONE) => {
                self.select_first();
                if let Some(select) = self.list_state.selected() {
                    Message::TabsSelect(select).enqueue();
                }
                true
            }
            (KeyCode::End, KeyModifiers::NONE) | (KeyCode::Char('G'), KeyModifiers::SHIFT) => {
                self.select_last();
                if let Some(select) = self.list_state.selected() {
                    Message::TabsSelect(select).enqueue();
                }
                true
            }
            (KeyCode::Enter, KeyModifiers::NONE) => {
                Message::TabsDismissSwitcher.enqueue();
                Message::TabsDismissSwitcher.enqueue();
                true
            }
            (KeyCode::Esc, KeyModifiers::NONE) => {
                Message::TabsDismissSwitcher.enqueue();
                Message::TabsSelect(self.rollback).enqueue();
                true
            }
            _ => false,
        }
    }
}
