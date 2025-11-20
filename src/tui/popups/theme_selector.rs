use std::ops::Deref;

use ratatui::widgets::StatefulWidget;

use crate::{
    misc::globals::{config, set_theme},
    tui::{
        pickers::search_picker::{SearchPicker, SearchPicker},
        themes::theme::Theme,
    },
};

#[derive(Debug)]
pub struct ThemeSelectorState {
    search_picker: SearchPicker,
    rollback: Theme,
}

impl ThemeSelectorState {
    pub fn into_rollback_theme(self) -> Theme {
        self.rollback
    }

    pub fn search_picker(&self) -> &SearchPicker {
        &self.search_picker
    }

    pub fn search_picker_mut(&mut self) -> &mut SearchPicker {
        &mut self.search_picker
    }
}

impl Default for ThemeSelectorState {
    fn default() -> Self {
        let mut search_picker = SearchPicker::default();
        let rollback = config().theme().deref().clone();
        let idx = Theme::all()
            .iter()
            .enumerate()
            .find_map(|(i, t)| (t == &rollback.app_theme()).then_some(i))
            .unwrap_or_default();
        search_picker.list_mut().select(Some(idx));

        Self {
            search_picker,
            rollback,
        }
    }
}

#[derive(Debug, Default)]
pub struct ThemeSelector {}

impl StatefulWidget for ThemeSelector {
    type State = ThemeSelectorState;

    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        state: &mut Self::State,
    ) {
        SearchPicker::default()
            .items(Theme::all().iter().map(|t| t.title()))
            .render(area, buf, &mut state.search_picker);
        if let Some(theme) = state
            .search_picker
            .selected()
            .and_then(|idx| Theme::all().get(idx).cloned())
        {
            set_theme(theme.into());
        }
    }
}
