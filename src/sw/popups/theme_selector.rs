use std::{borrow::Cow, sync::LazyLock};

use ratatui::{buffer::Buffer, layout::Rect, widgets::StatefulWidget};
use strum::IntoEnumIterator;

use crate::{
    sw::pickers::search_picker::{SearchPicker, SearchPickerState},
    tui::themes::theme::{LoadedTheme, Theme},
};

static THEMES: LazyLock<Vec<Theme>> = LazyLock::new(|| Theme::iter().collect());

#[derive(Debug)]
pub struct ThemeSelectorState {
    search: SearchPickerState,
    rollback: LoadedTheme,
}

impl ThemeSelectorState {
    pub fn new(rollback: LoadedTheme) -> Self {
        let idx = THEMES
            .iter()
            .position(|theme| rollback.app_theme() == theme)
            .unwrap_or_default();
        let mut search = SearchPickerState::default();
        search.select(Some(idx));

        Self { search, rollback }
    }

    pub fn search(&self) -> &SearchPickerState {
        &self.search
    }

    pub fn search_mut(&mut self) -> &mut SearchPickerState {
        &mut self.search
    }

    pub fn selected(&self) -> Option<Theme> {
        self.search
            .selected()
            .and_then(|idx| THEMES.get(idx))
            .copied()
    }

    pub fn rollback(&self) -> &LoadedTheme {
        &self.rollback
    }

    pub fn into_rollback_theme(self) -> LoadedTheme {
        self.rollback
    }
}

#[derive(Debug, Default)]
pub struct ThemeSelector<'a> {
    title: Cow<'a, str>,
}

impl<'a> ThemeSelector<'a> {
    pub fn title(mut self, title: impl Into<Cow<'a, str>>) -> Self {
        self.title = title.into();
        self
    }
}

impl StatefulWidget for ThemeSelector<'_> {
    type State = ThemeSelectorState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        SearchPicker::new(&THEMES)
            .title(self.title)
            .no_darken_bg()
            .render(area, buf, &mut state.search);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn typed(state: &mut ThemeSelectorState, text: &str) {
        for c in text.chars() {
            state.search_mut().input_mut().insert(c);
        }
    }

    fn render(state: &mut ThemeSelectorState, selector: ThemeSelector) -> Buffer {
        let area = Rect::new(0, 0, 100, 30);
        let mut buf = Buffer::empty(area);
        selector.render(area, &mut buf, state);
        buf
    }

    fn content(buf: &Buffer) -> String {
        buf.content().iter().map(|c| c.symbol()).collect()
    }

    mod state {
        use super::*;

        #[test]
        fn opens_on_the_rollback_theme() {
            let state = ThemeSelectorState::new(Theme::Dracula.into());

            assert_eq!(state.selected(), Some(Theme::Dracula));
        }

        #[test]
        fn a_different_rollback_opens_elsewhere() {
            let state = ThemeSelectorState::new(Theme::Nord.into());

            assert_eq!(state.selected(), Some(Theme::Nord));
        }

        #[test]
        fn the_rollback_theme_is_kept_while_browsing() {
            let mut state = ThemeSelectorState::new(Theme::Dracula.into());
            state.search_mut().select(Some(0));

            assert_eq!(state.rollback().app_theme(), &Theme::Dracula);
            assert_eq!(state.selected(), THEMES.first().copied());
        }

        #[test]
        fn the_rollback_theme_can_be_taken_back() {
            let state = ThemeSelectorState::new(Theme::Nord.into());

            assert_eq!(state.into_rollback_theme().app_theme(), &Theme::Nord);
        }

        #[test]
        fn no_selection_has_no_theme() {
            let mut state = ThemeSelectorState::new(Theme::Dracula.into());
            state.search_mut().select(None);

            assert_eq!(state.selected(), None);
        }

        #[test]
        fn moving_the_selection_changes_the_theme() {
            let mut state = ThemeSelectorState::new(Theme::Dracula.into());
            let opened_on = state.selected();

            state.search_mut().select_down();

            assert_ne!(state.selected(), opened_on);
        }

        #[test]
        fn filtering_maps_the_selection_back_to_the_theme() {
            let mut state = ThemeSelectorState::new(Theme::Dracula.into());
            typed(&mut state, "dracula");
            render(&mut state, ThemeSelector::default());

            state.search_mut().select(Some(0));

            assert_eq!(state.selected(), Some(Theme::Dracula));
        }

        #[test]
        fn every_theme_is_offered() {
            let mut state = ThemeSelectorState::new(Theme::Dracula.into());
            render(&mut state, ThemeSelector::default());

            assert_eq!(state.search().len(), Theme::iter().count());
        }
    }

    mod widget {
        use super::*;

        #[test]
        fn renders_theme_names() {
            let mut state = ThemeSelectorState::new(Theme::Dracula.into());
            typed(&mut state, "dracula");
            let buf = render(&mut state, ThemeSelector::default());

            assert!(content(&buf).to_lowercase().contains("dracula"));
        }

        #[test]
        fn renders_the_title() {
            let mut state = ThemeSelectorState::new(Theme::Dracula.into());
            let buf = render(&mut state, ThemeSelector::default().title("Theme"));

            assert!(content(&buf).contains("Theme"));
        }

        #[test]
        fn does_not_darken_the_background() {
            let area = Rect::new(0, 0, 100, 30);
            let mut buf = Buffer::empty(area);
            buf[(0, 0)].set_bg(ratatui::style::Color::Rgb(100, 150, 200));

            let mut state = ThemeSelectorState::new(Theme::Dracula.into());
            ThemeSelector::default().render(area, &mut buf, &mut state);

            assert_eq!(buf[(0, 0)].bg, ratatui::style::Color::Rgb(100, 150, 200));
        }
    }
}
