use crossterm::event::{KeyCode, KeyModifiers};
use ratatui::{
    layout::{Alignment, Rect},
    text::{Line, Span},
    widgets::{Block, List, ListItem, ListState, StatefulWidget},
};

use crate::{
    handler::message::Message,
    misc::{buffer_ext::BufferExt, config::theme},
    tui::{
        app_default::{AppDefault, AppTitle},
        component::Component,
        icons,
        tag_line::{Tag, TagLine},
    },
};

#[derive(Debug)]
pub struct TabSwitcher {
    title: String,
    list: ListState,
    tabs: Vec<String>,
    rollback: usize,
}

impl TabSwitcher {
    pub fn new(title: impl Into<String>, tabs: Vec<String>, idx: usize) -> TabSwitcher {
        Self {
            title: title.into(),
            rollback: idx,
            list: ListState::default().with_selected(Some(idx)),
            tabs,
        }
    }

    pub fn selected(&self) -> Option<usize> {
        self.list.selected()
    }

    pub fn select(&mut self, idx: impl Into<Option<usize>>) {
        self.list.select(idx.into());
    }

    pub fn select_prev(&mut self) {
        self.list.select_previous();
    }

    pub fn select_next(&mut self) {
        let idx = self
            .list
            .selected()
            .unwrap_or_default()
            .saturating_add(1)
            .min(self.tabs.len().saturating_sub(1));
        self.list.select(Some(idx));
    }

    pub fn select_first(&mut self) {
        self.list.select(Some(0));
    }
    pub fn select_last(&mut self) {
        self.list.select(Some(self.tabs.len().saturating_sub(1)));
    }
}

impl Component for TabSwitcher {
    fn render(
        &mut self,
        area: Rect,
        buf: &mut ratatui::prelude::Buffer,
        _focus_state: super::component::FocusState,
    ) {
        buf.clear(area);
        let num_width = self.tabs.len().to_string().len();
        StatefulWidget::render(
            List::app_default()
                .items(self.tabs.iter().enumerate().map(|(idx, tab)| {
                    ListItem::from(Line::from(vec![
                        Span::raw(format!("{:>num_width$}. ", idx + 1)).style(theme().subtext()),
                        Span::raw(tab.as_str()).style(theme().text()),
                    ]))
                }))
                .block(
                    Block::app_default()
                        .app_title(self.title.as_str())
                        .title_bottom(
                            TagLine::new()
                                .mono_color()
                                .centered()
                                .tag(Tag::new(icons::TRASH.str("Close"), "Backspace")),
                        )
                        .title_alignment(Alignment::Center),
                ),
            area,
            buf,
            &mut ListState::default().with_selected(self.list.selected()),
        );
    }

    fn handle(&mut self, event: crossterm::event::KeyEvent) -> bool {
        match (event.code, event.modifiers) {
            (KeyCode::Up, KeyModifiers::NONE)
            | (KeyCode::Char('k'), KeyModifiers::NONE)
            | (KeyCode::Char('p'), KeyModifiers::CONTROL) => {
                self.select_prev();
                Message::TabsSelect(self.selected().unwrap_or_default()).enqueue();
                true
            }
            (KeyCode::Down, KeyModifiers::NONE)
            | (KeyCode::Char('j'), KeyModifiers::NONE)
            | (KeyCode::Char('n'), KeyModifiers::CONTROL) => {
                self.select_next();
                Message::TabsSelect(self.selected().unwrap_or_default()).enqueue();
                true
            }
            (KeyCode::Home, KeyModifiers::NONE) | (KeyCode::Char('g'), KeyModifiers::NONE) => {
                self.select_first();
                Message::TabsSelect(self.selected().unwrap_or_default()).enqueue();
                true
            }
            (KeyCode::End, KeyModifiers::NONE) | (KeyCode::Char('G'), KeyModifiers::SHIFT) => {
                self.select_last();
                Message::TabsSelect(self.selected().unwrap_or_default()).enqueue();
                true
            }
            (KeyCode::Enter, KeyModifiers::NONE) => {
                Message::TabsDismissSwitcher.enqueue();
                true
            }
            (KeyCode::Esc, KeyModifiers::NONE)
            | (KeyCode::Char('q'), KeyModifiers::NONE)
            | (KeyCode::Char('t'), KeyModifiers::NONE) => {
                Message::TabsDismissSwitcher.enqueue();
                Message::TabsSelect(self.rollback).enqueue();
                true
            }
            _ => false,
        }
    }
}
