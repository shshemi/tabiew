use itertools::Itertools;
use ratatui::widgets::{Block, BorderType, Borders, ListState, StatefulWidget, Widget};

use crate::misc::globals::theme;

use super::{
    side_panel::{SidePanel, SidePanelState},
    status_bar::{StatusBar, Tag},
    tabular::{Tabular, TabularState},
};

#[derive(Debug)]
pub struct TabState {
    tabulars: Vec<TabularState>,
    side_panel: Option<SidePanelState>,
    idx: usize,
}

impl TabState {
    pub fn add(&mut self, tabular: TabularState) {
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

    pub fn selected(&self) -> Option<&TabularState> {
        self.tabulars.get(self.idx)
    }

    pub fn selected_mut(&mut self) -> Option<&mut TabularState> {
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

    pub fn iter(&self) -> impl Iterator<Item = &TabularState> {
        self.tabulars.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut TabularState> {
        self.tabulars.iter_mut()
    }

    pub fn side_panel(&self) -> Option<&SidePanelState> {
        self.side_panel.as_ref()
    }

    pub fn side_panel_mut(&mut self) -> Option<&mut SidePanelState> {
        self.side_panel.as_mut()
    }

    pub fn show_side_panel(&mut self) {
        self.side_panel = Some(SidePanelState::new(self.idx));
    }

    pub fn take_side_panel(&mut self) -> Option<SidePanelState> {
        self.side_panel.take()
    }
}

impl FromIterator<TabularState> for TabState {
    fn from_iter<T: IntoIterator<Item = TabularState>>(iter: T) -> Self {
        Self {
            tabulars: iter.into_iter().collect(),
            idx: 0,
            side_panel: None,
        }
    }
}

pub struct Tab {
    selection: bool,
    borders: bool,
}

impl Tab {
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

impl Default for Tab {
    fn default() -> Self {
        Self::new()
    }
}

impl StatefulWidget for Tab {
    type State = TabState;

    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        state: &mut Self::State,
    ) {
        // fix state (if invalid)
        state.idx = state.idx().min(state.len().saturating_sub(1));

        // build the status bar
        let status_bar = state
            .selected()
            .map(|tabular| {
                StatusBar::default()
                    .tag(Tag::new(
                        "Tab",
                        format!("{} / {}", state.idx + 1, state.len()),
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
                .border_type(BorderType::Rounded)
                .border_style(theme().block())
                .title_bottom(status_bar);
            let new = blk.inner(area);
            blk.render(area, buf);
            new
        };

        // render tabular
        if let Some(tabular) = state
            .side_panel
            .as_ref()
            .map(SidePanelState::list)
            .and_then(ListState::selected)
            .and_then(|idx| state.tabulars.get_mut(idx))
        {
            Tabular::default().render(area, buf, tabular);
        } else if let Some(tabular) = state.selected_mut() {
            Tabular::default().render(area, buf, tabular);
        }

        // render tabs
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
            let side_panel = SidePanel::new(&tab_titles).title("Tabs");
            side_panel.render(area, buf, side_panel_state);
        }
    }
}
