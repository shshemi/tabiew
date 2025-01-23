use std::marker::PhantomData;

use ratatui::widgets::StatefulWidget;

use super::{
    status_bar::NewStatusBarTag, tab_content::{TabContent, TabContentState}, Styler
};

#[derive(Debug)]
pub struct TabsState {
    tabulars: Vec<TabContentState>,
    idx: usize,
}

impl TabsState {
    pub fn add(&mut self, tabular: TabContentState) {
        self.tabulars.push(tabular);
    }

    pub fn len(&self) -> usize {
        self.tabulars.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn idx(&self) -> usize {
        self.idx
    }

    pub fn selected(&self) -> Option<&TabContentState> {
        self.tabulars.get(self.idx)
    }

    pub fn selected_mut(&mut self) -> Option<&mut TabContentState> {
        self.tabulars.get_mut(self.idx)
    }

    pub fn remove(&mut self, idx: usize) {
        if idx < self.tabulars.len() {
            self.tabulars.remove(idx);
        }
    }

    pub fn select(&mut self, idx: usize) {
        self.idx = idx;
    }

    pub fn select_next(&mut self) {
        self.select(self.idx.saturating_add(1))
    }

    pub fn select_prev(&mut self) {
        self.select(self.idx.saturating_sub(1))
    }

    pub fn select_first(&mut self) {
        self.select(0)
    }

    pub fn select_last(&mut self) {
        self.select(usize::MAX)
    }

    pub fn iter(&self) -> impl Iterator<Item = &TabContentState> {
        self.tabulars.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut TabContentState> {
        self.tabulars.iter_mut()
    }
}

impl FromIterator<TabContentState> for TabsState {
    fn from_iter<T: IntoIterator<Item = TabContentState>>(iter: T) -> Self {
        Self {
            tabulars: iter.into_iter().collect(),
            idx: 0,
        }
    }
}

pub struct Tabs<Theme> {
    selection: bool,
    _theme: PhantomData<Theme>,
}

impl<Theme> Tabs<Theme> {
    pub fn new() -> Self {
        Self {
            selection: false,
            _theme: Default::default(),
        }
    }

    pub fn selection(mut self, selection: bool) -> Self {
        self.selection = selection;
        self
    }
}

impl<Theme> Default for Tabs<Theme> {
    fn default() -> Self {
        Self::new()
    }
}

impl<Theme: Styler> StatefulWidget for Tabs<Theme> {
    type State = TabsState;

    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        state: &mut Self::State,
    ) {
        state.idx = state.idx().min(state.len().saturating_sub(1));
        let tag_value = format!("{} / {}", state.idx + 1, state.len());
        if let Some(tabular) = state.selected_mut() {
            TabContent::<Theme>::new()
                .with_tag(NewStatusBarTag::new("Tab", tag_value))
                .render(area, buf, tabular);
        }
    }
}
