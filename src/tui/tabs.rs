use ratatui::widgets::StatefulWidget;

use super::{
    status_bar::StatusBarTag,
    tabular::{Tabular, TabularState},
};

#[derive(Debug)]
pub enum Tab {
    Tabular(TabularState),
}

impl Tab {
    pub fn tabular(&self) -> Option<&TabularState> {
        if let Tab::Tabular(tabular) = self {
            Some(tabular)
        } else {
            None
        }
    }

    pub fn tabular_mut(&mut self) -> Option<&mut TabularState> {
        if let Tab::Tabular(tabular) = self {
            Some(tabular)
        } else {
            None
        }
    }

    pub fn tick(&mut self) {
        match self {
            Tab::Tabular(tab_content_state) => tab_content_state.tick(),
        }
    }
}

impl From<TabularState> for Tab {
    fn from(value: TabularState) -> Self {
        Tab::Tabular(value)
    }
}

#[derive(Debug)]
pub struct TabsState {
    tabulars: Vec<Tab>,
    idx: usize,
}

impl TabsState {
    pub fn add(&mut self, tabular: impl Into<Tab>) {
        self.tabulars.push(tabular.into());
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

    pub fn selected(&self) -> Option<&Tab> {
        self.tabulars.get(self.idx)
    }

    pub fn selected_mut(&mut self) -> Option<&mut Tab> {
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

    pub fn iter(&self) -> impl Iterator<Item = &Tab> {
        self.tabulars.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Tab> {
        self.tabulars.iter_mut()
    }
}

impl FromIterator<TabularState> for TabsState {
    fn from_iter<T: IntoIterator<Item = TabularState>>(iter: T) -> Self {
        Self {
            tabulars: iter.into_iter().map(Into::into).collect(),
            idx: 0,
        }
    }
}

pub struct Tabs {
    selection: bool,
    borders: bool,
}

impl Tabs {
    pub fn new() -> Self {
        Self {
            selection: false,
            borders: true,
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

impl Default for Tabs {
    fn default() -> Self {
        Self::new()
    }
}

impl StatefulWidget for Tabs {
    type State = TabsState;

    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        state: &mut Self::State,
    ) {
        state.idx = state.idx().min(state.len().saturating_sub(1));
        let tag_value = format!("{} / {}", state.idx + 1, state.len());
        if let Some(tab) = state.selected_mut() {
            match tab {
                Tab::Tabular(tab_content_state) => {
                    Tabular::new()
                        .with_tag(StatusBarTag::new("Tab", tag_value))
                        .with_borders(self.borders)
                        .render(area, buf, tab_content_state);
                }
            }
        }
    }
}
