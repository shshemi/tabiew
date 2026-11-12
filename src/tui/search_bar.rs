use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::widgets::{Block, Widget};

use crate::{
    handler::message::Message,
    tui::{
        app_default::{AppDefault, AppTitle},
        component::Component,
        icons,
    },
};

use super::widgets::input::Input;

#[derive(Debug)]
pub enum SearchType {
    Fuzzy,
    Exact,
}

#[derive(Debug)]
pub struct SearchBar {
    input: Input,
    search_type: SearchType,
}

impl SearchBar {
    pub fn duckdb(search_type: SearchType) -> Self {
        SearchBar {
            input: Default::default(),
            search_type,
        }
    }

    pub fn search_type(&self) -> &SearchType {
        &self.search_type
    }

    pub fn value(&self) -> &str {
        self.input.value()
    }
}

impl Component for SearchBar {
    fn render(
        &mut self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        focus_state: super::component::FocusState,
    ) {
        let title = match &self.search_type {
            SearchType::Fuzzy => icons::FUZZY_SEARCH.title("Fuzzy Search"),
            SearchType::Exact => icons::SEARCH.title("Search"),
        };
        let area = {
            let block = Block::app_default().app_title(title);
            let inner = block.inner(area);
            block.render(area, buf);
            inner
        };
        self.input.render(area, buf, focus_state);
    }

    fn handle(&mut self, event: KeyEvent) -> bool {
        if self.input.handle(event) {
            true
        } else {
            match (event.code, event.modifiers) {
                (KeyCode::Esc, KeyModifiers::NONE) => {
                    Message::PaneDismissModal.enqueue();
                    Message::PanePopDataFrame.enqueue();
                    true
                }
                (KeyCode::Enter, KeyModifiers::NONE) => {
                    Message::PaneDismissModal.enqueue();
                    true
                }
                _ => false,
            }
        }
    }
}
