use itertools::Itertools;
use ratatui::widgets::{Borders, StatefulWidget, TableState, Widget};

use crate::tui::{component::Component, widgets::block::Block};

use super::{
    enumerated_list::{EnumeratedList, EnumeratedListState},
    pane::Pane,
    status_bar::{StatusBar, Tag},
};

#[derive(Debug)]
pub struct TabsState {
    tabulars: Vec<Pane>,
    side_panel: Option<EnumeratedListState>,
    idx: usize,
}

impl TabsState {
    pub fn add(&mut self, tabular: Pane) {
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

    pub fn selected(&self) -> Option<&Pane> {
        self.tabulars.get(self.idx)
    }

    pub fn selected_mut(&mut self) -> Option<&mut Pane> {
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

    pub fn iter(&self) -> impl Iterator<Item = &Pane> {
        self.tabulars.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Pane> {
        self.tabulars.iter_mut()
    }

    pub fn side_panel(&self) -> Option<&EnumeratedListState> {
        self.side_panel.as_ref()
    }

    pub fn side_panel_mut(&mut self) -> Option<&mut EnumeratedListState> {
        self.side_panel.as_mut()
    }

    pub fn show_side_panel(&mut self) {
        self.side_panel = Some(EnumeratedListState::new(self.idx));
    }

    pub fn take_side_panel(&mut self) -> Option<EnumeratedListState> {
        self.side_panel.take()
    }
}

impl FromIterator<Pane> for TabsState {
    fn from_iter<T: IntoIterator<Item = Pane>>(iter: T) -> Self {
        Self {
            tabulars: iter.into_iter().collect(),
            idx: 0,
            side_panel: None,
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
        // index of tabular to show
        let tabular_idx = state
            .side_panel
            .as_ref()
            .map(EnumeratedListState::list)
            .and_then(TableState::selected)
            .unwrap_or(state.idx)
            .min(state.tabulars.len().saturating_sub(1));

        // fix state (if invalid)
        state.idx = state.idx().min(state.len().saturating_sub(1));

        // build the status bar
        let status_bar = state
            .tabulars
            .get(tabular_idx)
            .map(|tabular| {
                StatusBar::default()
                    .tag(Tag::new(
                        "Tab",
                        format!("{} / {}", tabular_idx + 1, state.len()),
                    ))
                    .tag(match tabular.table_type() {
                        super::TableType::Help => Tag::new("Table", "Help"),
                        super::TableType::Name(name) => Tag::new("Table", name),
                        super::TableType::Query(query) => Tag::new("Query", query),
                    })
                    .tag(Tag::new(
                        "Auto-Fit",
                        if !tabular.table().expanded() {
                            "Yes"
                        } else {
                            " No"
                        },
                    ))
                    .tag(Tag::new(
                        "Row",
                        format!(
                            "{:>width$}",
                            tabular.table().selected() + 1,
                            width = tabular.table().data_frame().height().to_string().len()
                        ),
                    ))
                    .tag(Tag::new(
                        "Shape",
                        format!(
                            "{} x {}",
                            tabular.table().data_frame().height(),
                            tabular.table().data_frame().width()
                        ),
                    ))
            })
            .unwrap_or_default();

        // render block with status bar
        let area = {
            let blk = Block::default()
                .borders(if self.borders {
                    Borders::all()
                } else {
                    Borders::empty()
                })
                .bottom(status_bar);
            let new = blk.inner(area);
            blk.render(area, buf);
            new
        };

        // render tabular
        if let Some(pane) = state.tabulars.get_mut(tabular_idx) {
            // Pane.render(area, buf, tabular);
            pane.render(area, buf, focus_state);
        }

        // render tabs
        // TODO: fix later
        let tab_titles = state
            .iter()
            .map(|tabular| {
                match tabular.table_type() {
                    crate::tui::TableType::Help => "Help",
                    crate::tui::TableType::Name(name) => name.as_str(),
                    crate::tui::TableType::Query(query) => query.as_str(),
                }
                .to_owned()
            })
            .collect_vec();
        if let Some(side_panel_state) = state.side_panel.as_mut() {
            let side_panel = EnumeratedList::default().items(tab_titles).title("Tabs");
            side_panel.render(area, buf, side_panel_state);
        }
    }
}
