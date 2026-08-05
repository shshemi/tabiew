use crossterm::event::{KeyCode, KeyModifiers};
use ratatui::layout::Rect;

use crate::{
    handler::message::Message,
    tui::{component::Component, pickers::list_picker::ListPicker},
};

#[derive(Debug)]
pub struct TabSwitcher {
    picker: ListPicker<String>,
    rollback: usize,
}

impl TabSwitcher {
    pub fn new(title: impl Into<String>, tabs: Vec<String>, idx: usize) -> TabSwitcher {
        let title = title.into();
        let mut picker = ListPicker::new(tabs.clone()).with_title(title.clone());
        picker.select(idx);
        Self {
            picker,
            rollback: idx,
        }
    }

    pub fn selected(&self) -> Option<usize> {
        self.picker.selected()
    }

    pub fn select(&mut self, idx: impl Into<Option<usize>>) {
        self.picker.select(idx);
    }

    pub fn select_prev(&mut self) {
        self.picker.select_up();
    }

    pub fn select_next(&mut self) {
        let idx = self
            .picker
            .selected()
            .unwrap_or_default()
            .saturating_add(1)
            .min(self.picker.len().saturating_sub(1));
        self.picker.select(Some(idx));
    }

    pub fn select_first(&mut self) {
        self.picker.select(Some(0));
    }
    pub fn select_last(&mut self) {
        self.picker
            .select(Some(self.picker.len().saturating_sub(1)));
    }
}

impl Component for TabSwitcher {
    fn render(
        &mut self,
        area: Rect,
        buf: &mut ratatui::prelude::Buffer,
        focus_state: super::component::FocusState,
    ) {
        self.picker.render(area, buf, focus_state);
    }

    fn handle(&mut self, event: crossterm::event::KeyEvent) -> bool {
        if self.picker.handle(event) {
            if let Some(select) = self.picker.selected() {
                Message::TabsSelect(select).enqueue();
            }
            true
        } else {
            match (event.code, event.modifiers) {
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
}
