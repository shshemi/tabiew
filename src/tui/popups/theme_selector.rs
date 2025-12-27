use std::ops::Deref;

use crossterm::event::KeyCode;
use strum::IntoEnumIterator;

use crate::{
    handler::message::Message,
    misc::{
        config::{config, theme},
        type_ext::UnwrapOrEnqueueError,
    },
    tui::{
        component::Component,
        pickers::search_picker::SearchPicker,
        themes::theme::{LoadedTheme, Theme},
    },
};

#[derive(Debug)]
pub struct ThemeSelector {
    search_picker: SearchPicker<Theme>,
    rollback: LoadedTheme,
}

impl ThemeSelector {
    pub fn into_rollback_theme(self) -> LoadedTheme {
        self.rollback
    }

    pub fn selected(&self) -> Option<Theme> {
        self.search_picker.selected_item().cloned()
    }

    pub fn search_picker(&self) -> &SearchPicker<Theme> {
        &self.search_picker
    }

    pub fn search_picker_mut(&mut self) -> &mut SearchPicker<Theme> {
        &mut self.search_picker
    }
}

impl Component for ThemeSelector {
    fn render(
        &mut self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        focus_state: crate::tui::component::FocusState,
    ) {
        self.search_picker.render(area, buf, focus_state);
        if let Some(t) = self.search_picker.selected_item()
            && t != theme().app_theme()
        {
            config().set_theme(*t);
        }
    }

    fn handle(&mut self, event: crossterm::event::KeyEvent) -> bool {
        self.search_picker.handle(event)
            || match event.code {
                KeyCode::Esc => {
                    Message::AppDismissOverlay.enqueue();
                    config().set_theme(self.rollback.clone());
                    true
                }
                KeyCode::Enter => {
                    Message::AppDismissOverlay.enqueue();
                    config().store().unwrap_or_enqueue_error();
                    true
                }
                _ => false,
            }
    }
}

impl Default for ThemeSelector {
    fn default() -> Self {
        let mut search_picker = SearchPicker::new(Theme::iter().collect());
        let rollback = config().theme().deref().clone();
        let idx = Theme::iter()
            .enumerate()
            .find_map(|(i, t)| rollback.app_theme().eq(&t).then_some(i))
            .unwrap_or_default();
        search_picker.select(Some(idx));

        Self {
            search_picker,
            rollback,
        }
    }
}
