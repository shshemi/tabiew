use std::ops::Deref;

use crate::{
    misc::globals::config,
    tui::{
        component::Component,
        pickers::search_picker::SearchPicker,
        themes::theme::{AppTheme, Theme},
    },
};

#[derive(Debug)]
pub struct ThemeSelector {
    search_picker: SearchPicker<AppTheme>,
    rollback: Theme,
}

impl ThemeSelector {
    pub fn into_rollback_theme(self) -> Theme {
        self.rollback
    }

    pub fn selected(&self) -> Option<AppTheme> {
        self.search_picker.selected_item().cloned()
    }

    pub fn search_picker(&self) -> &SearchPicker<AppTheme> {
        &self.search_picker
    }

    pub fn search_picker_mut(&mut self) -> &mut SearchPicker<AppTheme> {
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
    }

    fn handle(&mut self, event: crossterm::event::KeyEvent) -> bool {
        self.search_picker.handle(event)
    }
}

impl Default for ThemeSelector {
    fn default() -> Self {
        let mut search_picker = SearchPicker::new(Theme::all().to_vec());
        let rollback = config().theme().deref().clone();
        let idx = Theme::all()
            .iter()
            .enumerate()
            .find_map(|(i, t)| (t == &rollback.app_theme()).then_some(i))
            .unwrap_or_default();
        search_picker.select(Some(idx));

        Self {
            search_picker,
            rollback,
        }
    }
}

// #[derive(Debug, Default)]
// pub struct ThemeSelector {}

// impl StatefulWidget for ThemeSelector {
//     type State = ThemeSelectorState;

//     fn render(
//         self,
//         area: ratatui::prelude::Rect,
//         buf: &mut ratatui::prelude::Buffer,
//         state: &mut Self::State,
//     ) {
//         if let Some(theme) = state
//             .search_picker
//             .selected()
//             .and_then(|idx| Theme::all().get(idx).cloned())
//         {
//             set_theme(theme.into());
//         }
//     }
// }
