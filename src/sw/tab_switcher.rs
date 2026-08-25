use std::borrow::Cow;

use ratatui::{buffer::Buffer, layout::Rect, widgets::StatefulWidget};

use crate::sw::pickers::list_picker::{ListPicker, ListPickerState};

#[derive(Debug)]
pub struct TabSwitcherState {
    list: ListPickerState,
    rollback: usize,
}

impl TabSwitcherState {
    pub fn new(rollback: usize) -> Self {
        let mut list = ListPickerState::default();
        list.select(Some(rollback));

        Self { list, rollback }
    }

    pub fn list(&self) -> &ListPickerState {
        &self.list
    }

    pub fn rollback(&self) -> usize {
        self.rollback
    }

    pub fn selected(&self) -> Option<usize> {
        self.list.selected()
    }

    pub fn select(&mut self, idx: impl Into<Option<usize>>) {
        self.list.select(idx.into());
    }

    pub fn select_prev(&mut self) {
        self.list.select_up();
    }

    pub fn select_next(&mut self) {
        let idx = self
            .list
            .selected()
            .unwrap_or_default()
            .saturating_add(1)
            .min(self.list.len().saturating_sub(1));
        self.list.select(Some(idx));
    }

    pub fn select_first(&mut self) {
        self.list.select(Some(0));
    }

    pub fn select_last(&mut self) {
        self.list.select(Some(self.list.len().saturating_sub(1)));
    }
}

#[derive(Debug)]
pub struct TabSwitcher<'a> {
    names: &'a [String],
    title: Cow<'a, str>,
}

impl<'a> TabSwitcher<'a> {
    pub fn new(names: &'a [String]) -> Self {
        Self {
            names,
            title: Cow::Borrowed("Tabs"),
        }
    }

    pub fn title(mut self, title: impl Into<Cow<'a, str>>) -> Self {
        self.title = title.into();
        self
    }
}

impl StatefulWidget for TabSwitcher<'_> {
    type State = TabSwitcherState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        ListPicker::new(self.names)
            .title(self.title)
            .render(area, buf, &mut state.list);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn names(count: usize) -> Vec<String> {
        (0..count).map(|idx| format!("tab{idx}")).collect()
    }

    fn render(state: &mut TabSwitcherState, switcher: TabSwitcher) -> Buffer {
        let area = Rect::new(0, 0, 100, 30);
        let mut buf = Buffer::empty(area);
        switcher.render(area, &mut buf, state);
        buf
    }

    fn synced(rollback: usize, count: usize) -> (TabSwitcherState, Vec<String>) {
        let names = names(count);
        let mut state = TabSwitcherState::new(rollback);
        render(&mut state, TabSwitcher::new(&names));
        (state, names)
    }

    fn content(buf: &Buffer) -> String {
        buf.content().iter().map(|c| c.symbol()).collect()
    }

    mod state {
        use super::*;

        #[test]
        fn it_opens_on_the_current_tab() {
            let state = TabSwitcherState::new(2);

            assert_eq!(state.selected(), Some(2));
        }

        #[test]
        fn the_current_tab_is_remembered_for_rollback() {
            let (mut state, _) = synced(2, 5);
            state.select_first();

            assert_eq!(state.selected(), Some(0));
            assert_eq!(state.rollback(), 2);
        }

        #[test]
        fn moving_forward_stops_at_the_last_tab() {
            let (mut state, _) = synced(0, 3);
            for _ in 0..10 {
                state.select_next();
            }

            assert_eq!(state.selected(), Some(2));
        }

        #[test]
        fn moving_back_stops_at_the_first_tab() {
            let (mut state, _) = synced(2, 3);
            for _ in 0..10 {
                state.select_prev();
            }

            assert_eq!(state.selected(), Some(0));
        }

        #[test]
        fn moving_walks_one_tab_at_a_time() {
            let (mut state, _) = synced(0, 3);
            state.select_next();
            assert_eq!(state.selected(), Some(1));
            state.select_prev();
            assert_eq!(state.selected(), Some(0));
        }

        #[test]
        fn jumping_to_the_first_and_last_tab() {
            let (mut state, _) = synced(1, 4);
            state.select_last();
            assert_eq!(state.selected(), Some(3));
            state.select_first();
            assert_eq!(state.selected(), Some(0));
        }

        #[test]
        fn rendering_adopts_the_tab_count() {
            let (state, _) = synced(0, 4);

            assert_eq!(state.list().len(), 4);
        }

        #[test]
        fn a_single_tab_leaves_the_selection_alone() {
            let (mut state, _) = synced(0, 1);
            state.select_next();

            assert_eq!(state.selected(), Some(0));
        }
    }

    mod widget {
        use super::*;

        #[test]
        fn every_tab_name_is_listed() {
            let names = names(3);
            let buf = render(&mut TabSwitcherState::new(0), TabSwitcher::new(&names));

            let content = content(&buf);
            assert!(content.contains("tab0"));
            assert!(content.contains("tab2"));
        }

        #[test]
        fn defaults_to_a_tabs_title() {
            let names = names(2);
            let buf = render(&mut TabSwitcherState::new(0), TabSwitcher::new(&names));

            assert!(content(&buf).contains("Tabs"));
        }

        #[test]
        fn the_title_is_overridable() {
            let names = names(2);
            let buf = render(
                &mut TabSwitcherState::new(0),
                TabSwitcher::new(&names).title("Switch To"),
            );

            let content = content(&buf);
            assert!(content.contains("Switch To"));
            assert!(!content.contains("Tabs"));
        }

        #[test]
        fn no_tabs_renders_without_panicking() {
            let names = names(0);
            let mut state = TabSwitcherState::new(0);
            render(&mut state, TabSwitcher::new(&names));

            assert!(state.list().is_empty());
        }
    }
}
