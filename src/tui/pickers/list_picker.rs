use std::fmt::Display;

use crossterm::event::{KeyCode, KeyModifiers};
use ratatui::widgets::{Block, List, ListItem, ListState, StatefulWidget};

use crate::{
    misc::{buffer_ext::BufferExt, config::theme, rect_ext::RectExt},
    tui::{app_default::AppDefault, component::Component},
};

#[derive(Debug)]
pub struct ListPicker<T> {
    title: String,
    list: ListState,
    items: Vec<T>,
    strings: Vec<String>,
    darken_bg: bool,
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
            darken_bg: true,
        }
    }

    pub fn with_title(self, title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            ..self
        }
    }

    pub fn no_darken_bg(self) -> Self {
        Self {
            darken_bg: false,
            ..self
        }
    }

    pub fn select(&mut self, idx: impl Into<Option<usize>>) {
        self.list.select(idx.into());
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

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<T> Component for ListPicker<T> {
    fn render(
        &mut self,
        _area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        _focus_state: crate::tui::component::FocusState,
    ) {
        if self.darken_bg {
            buf.darken();
        }

        let height = self.strings.len().saturating_add(2).min(25) as u16;
        let area = buf.area.palette(height);
        buf.clear(area);

        StatefulWidget::render(
            List::default()
                .style(theme().text())
                .highlight_style(theme().row_highlighted())
                .items(self.strings.iter().map(|s| ListItem::from(s.as_str())))
                .block(Block::app_default().title(self.title.as_str())),
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
            (KeyCode::Home, KeyModifiers::NONE) | (KeyCode::Char('g'), KeyModifiers::NONE) => {
                self.list.select_first();
                true
            }
            (KeyCode::End, KeyModifiers::NONE) | (KeyCode::Char('G'), KeyModifiers::SHIFT) => {
                self.list.select_last();
                true
            }
            _ => false,
        }
    }
}
