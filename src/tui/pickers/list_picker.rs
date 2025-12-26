use std::fmt::Display;

use crossterm::event::{KeyCode, KeyModifiers};
use ratatui::{
    layout::{Constraint, Flex, Layout},
    widgets::{Clear, List, ListItem, ListState, StatefulWidget, Widget},
};

use crate::{
    misc::globals::theme,
    tui::{component::Component, widgets::block::Block},
};

#[derive(Debug)]
pub struct ListPicker<T> {
    title: String,
    list: ListState,
    items: Vec<T>,
    strings: Vec<String>,
}

impl<T> ListPicker<T>
where
    T: Display,
{
    pub fn new(items: Vec<T>) -> Self {
        Self {
            list: ListState::default().with_selected(0.into()),
            strings: items.iter().map(ToString::to_string).collect(),
            title: Default::default(),
            items,
        }
    }

    pub fn with_title(self, title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            ..self
        }
    }

    pub fn selected(&self) -> Option<usize> {
        self.list.selected()
    }

    pub fn selected_item(&self) -> Option<&T> {
        self.selected().and_then(|i| self.items.get(i))
    }

    pub fn selected_str(&self) -> Option<&str> {
        self.selected()
            .and_then(|i| self.strings.get(i).map(String::as_str))
    }

    pub fn select_up(&mut self) {
        self.list.select_previous();
    }

    pub fn select_down(&mut self) {
        self.list.select_next();
    }

    // pub fn list(&self) -> &ListState {
    //     &self.list
    // }

    // pub fn list_mut(&mut self) -> &mut ListState {
    //     &mut self.list
    // }
}

impl<T> Component for ListPicker<T> {
    fn render(
        &mut self,
        _area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        _focus_state: crate::tui::component::FocusState,
    ) {
        let width = 80;
        let height = self.strings.len().saturating_add(2).min(25) as u16;

        let [area] = Layout::horizontal([Constraint::Length(width)])
            .flex(Flex::Center)
            .areas(buf.area);
        let [_, area] =
            Layout::vertical([Constraint::Length(3), Constraint::Length(height)]).areas(area);
        Clear.render(area, buf);

        StatefulWidget::render(
            List::default()
                .style(theme().text())
                .highlight_style(theme().row_highlighted())
                .items(self.strings.iter().map(|s| ListItem::from(s.as_str())))
                .block(Block::default().title(self.title.as_str()).into_widget()),
            area,
            buf,
            &mut self.list,
        );
    }

    fn handle(&mut self, event: crossterm::event::KeyEvent) -> bool {
        match (event.code, event.modifiers) {
            (KeyCode::Up, KeyModifiers::NONE)
            | (KeyCode::Char('k'), KeyModifiers::NONE)
            | (KeyCode::Char('p'), KeyModifiers::CONTROL) => {
                if self.list.selected() != Some(0) {
                    self.list.select_previous();
                } else {
                    self.list.select(Some(self.items.len().saturating_sub(1)));
                }
                true
            }
            (KeyCode::Down, KeyModifiers::NONE)
            | (KeyCode::Char('j'), KeyModifiers::NONE)
            | (KeyCode::Char('n'), KeyModifiers::CONTROL) => {
                if self.list.selected() != Some(self.items.len().saturating_sub(1)) {
                    self.list.select_next();
                } else {
                    self.list.select_first();
                }
                true
            }
            _ => false,
        }
    }
}
