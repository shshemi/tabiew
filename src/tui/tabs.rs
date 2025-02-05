use std::marker::PhantomData;

use polars::frame::DataFrame;
use ratatui::widgets::StatefulWidget;

use super::{
    status_bar::StatusBarTag,
    tab_content::{TabContent, TabContentState},
    Styler,
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

    pub fn selected_data_frame(&self) -> Option<DataFrame> {
        self.selected().map(|tab| tab.data_frame().clone())
    }

    pub fn remove(&mut self, idx: usize) {
        if idx < self.tabulars.len() {
            self.tabulars.remove(idx);
        }
    }

    pub fn select(&mut self, idx: usize) {
        self.idx = idx;
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
    borders: bool,
    _theme: PhantomData<Theme>,
}

impl<Theme> Tabs<Theme> {
    pub fn new() -> Self {
        Self {
            selection: false,
            borders: true,
            _theme: Default::default(),
        }
    }

    pub fn selection(mut self, selection: bool) -> Self {
        self.selection = selection;
        self
    }

    pub fn with_borders(mut self, borders: bool) -> Self {
        self.borders = borders;
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
                .with_tag(StatusBarTag::new("Tab", tag_value))
                .with_borders(self.borders)
                .render(area, buf, tabular);
        }
    }
}
