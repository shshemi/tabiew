use crossterm::event::KeyCode;
use ratatui::widgets::{Borders, Widget};

use crate::{
    misc::type_ext::VecExt,
    tui::{component::Component, widgets::block::Block},
};

use super::{
    enumerated_list::EnumeratedList,
    pane::Pane,
    status_bar::{StatusBar, Tag},
};

#[derive(Debug)]
pub struct TabsState {
    panes: Vec<Pane>,
    panel: Option<EnumeratedList>,
    idx: usize,
    borders: bool,
}

impl TabsState {
    pub fn add(&mut self, tabular: Pane) {
        self.panes.push(tabular);
    }

    pub fn len(&self) -> usize {
        self.panes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn idx(&self) -> usize {
        self.idx
    }

    pub fn selected(&self) -> Option<&Pane> {
        self.panes.get(self.idx)
    }

    pub fn selected_mut(&mut self) -> Option<&mut Pane> {
        self.panes.get_mut(self.idx)
    }

    pub fn remove(&mut self, idx: usize) {
        if idx < self.panes.len() {
            self.panes.remove(idx);
        }
    }

    pub fn select(&mut self, idx: usize) {
        self.idx = idx;
    }

    pub fn iter(&self) -> impl Iterator<Item = &Pane> {
        self.panes.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Pane> {
        self.panes.iter_mut()
    }

    pub fn side_panel(&self) -> Option<&EnumeratedList> {
        self.panel.as_ref()
    }

    pub fn side_panel_mut(&mut self) -> Option<&mut EnumeratedList> {
        self.panel.as_mut()
    }

    pub fn show_panel(&mut self) {
        self.panel = Some(
            EnumeratedList::new(
                "Tabs",
                self.panes
                    .iter()
                    .map(|pane| pane.table_type().title())
                    .collect(),
            )
            .with_selected(self.idx),
        );
    }

    pub fn hide_side_panel(&mut self) -> Option<EnumeratedList> {
        self.panel.take()
    }
}

impl Component for TabsState {
    fn render(
        &mut self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        focus_state: super::component::FocusState,
    ) {
        // index of tabular to show
        let tabular_idx = self
            .panel
            .as_ref()
            .and_then(|list| list.selected())
            .unwrap_or(self.idx);
        // let tabular_idx = state
        //     .side_panel
        //     .as_ref()
        //     .map(EnumeratedList::list)
        //     .and_then(TableState::selected)
        //     .unwrap_or(state.idx)
        //     .min(state.tabulars.len().saturating_sub(1));

        // fix state (if invalid)
        self.idx = self.idx().min(self.len().saturating_sub(1));

        // build the status bar
        let status_bar = self
            .panes
            .get(tabular_idx)
            .map(|tabular| {
                StatusBar::default()
                    .tag(Tag::new(
                        "Tab",
                        format!("{} / {}", tabular_idx + 1, self.len()),
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
        if let Some(pane) = self.panes.get_mut(tabular_idx) {
            // Pane.render(area, buf, tabular);
            pane.render(area, buf, focus_state);
        }

        // render tabs
        // TODO: fix later
        // let tab_titles = self
        //     .iter()
        //     .map(|tabular| {
        //         match tabular.table_type() {
        //             crate::tui::TableType::Help => "Help",
        //             crate::tui::TableType::Name(name) => name.as_str(),
        //             crate::tui::TableType::Query(query) => query.as_str(),
        //         }
        //         .to_owned()
        //     })
        //     .collect_vec();
        if let Some(side_panel) = self.panel.as_mut() {
            side_panel.render(area, buf, focus_state);
        }
    }

    fn handle(&mut self, event: crossterm::event::KeyEvent) -> bool {
        self.panel
            .as_mut()
            .map(|list| list.handle(event))
            .unwrap_or_default()
            || self
                .panes
                .get_mut(self.idx)
                .map(|pane| pane.handle(event))
                .unwrap_or_default()
            || match event.code {
                KeyCode::Esc => self.panel.take().is_some(),
                KeyCode::Char('q') => {
                    self.panel.take().is_some() || self.panes.take(self.idx).is_some()
                }
                KeyCode::Char('t') => {
                    self.show_panel();
                    true
                }
                _ => false,
            }
    }
}

impl FromIterator<Pane> for TabsState {
    fn from_iter<T: IntoIterator<Item = Pane>>(iter: T) -> Self {
        Self {
            panes: iter.into_iter().collect(),
            idx: 0,
            panel: None,
            borders: true,
        }
    }
}

// pub struct Tabs {
//     selection: bool,
//     borders: bool,
// }

// impl Tabs {
//     pub fn new() -> Self {
//         Self {
//             selection: false,
//             borders: true,
//         }
//     }

//     pub fn selection(mut self, selection: bool) -> Self {
//         self.selection = selection;
//         self
//     }

//     pub fn with_borders(mut self, borders: bool) -> Self {
//         self.borders = borders;
//         self
//     }
// }

// impl Default for Tabs {
//     fn default() -> Self {
//         Self::new()
//     }
// }

// impl StatefulWidget for Tabs {
//     type State = TabsState;

//     fn render(
//         self,
//         area: ratatui::prelude::Rect,
//         buf: &mut ratatui::prelude::Buffer,
//         state: &mut Self::State,
//     ) {
//         // index of tabular to show
//         let tabular_idx = state
//             .side_panel
//             .as_ref()
//             .map(EnumeratedList::list)
//             .and_then(TableState::selected)
//             .unwrap_or(state.idx)
//             .min(state.tabulars.len().saturating_sub(1));

//         // fix state (if invalid)
//         state.idx = state.idx().min(state.len().saturating_sub(1));

//         // build the status bar
//         let status_bar = state
//             .tabulars
//             .get(tabular_idx)
//             .map(|tabular| {
//                 StatusBar::default()
//                     .tag(Tag::new(
//                         "Tab",
//                         format!("{} / {}", tabular_idx + 1, state.len()),
//                     ))
//                     .tag(match tabular.table_type() {
//                         super::TableType::Help => Tag::new("Table", "Help"),
//                         super::TableType::Name(name) => Tag::new("Table", name),
//                         super::TableType::Query(query) => Tag::new("Query", query),
//                     })
//                     .tag(Tag::new(
//                         "Auto-Fit",
//                         if !tabular.table().expanded() {
//                             "Yes"
//                         } else {
//                             " No"
//                         },
//                     ))
//                     .tag(Tag::new(
//                         "Row",
//                         format!(
//                             "{:>width$}",
//                             tabular.table().selected() + 1,
//                             width = tabular.table().data_frame().height().to_string().len()
//                         ),
//                     ))
//                     .tag(Tag::new(
//                         "Shape",
//                         format!(
//                             "{} x {}",
//                             tabular.table().data_frame().height(),
//                             tabular.table().data_frame().width()
//                         ),
//                     ))
//             })
//             .unwrap_or_default();

//         // render block with status bar
//         let area = {
//             let blk = Block::default()
//                 .borders(if self.borders {
//                     Borders::all()
//                 } else {
//                     Borders::empty()
//                 })
//                 .bottom(status_bar);
//             let new = blk.inner(area);
//             blk.render(area, buf);
//             new
//         };

//         // render tabular
//         if let Some(pane) = state.tabulars.get_mut(tabular_idx) {
//             // Pane.render(area, buf, tabular);
//             pane.render(area, buf, focus_state);
//         }

//         // render tabs
//         // TODO: fix later
//         let tab_titles = state
//             .iter()
//             .map(|tabular| {
//                 match tabular.table_type() {
//                     crate::tui::TableType::Help => "Help",
//                     crate::tui::TableType::Name(name) => name.as_str(),
//                     crate::tui::TableType::Query(query) => query.as_str(),
//                 }
//                 .to_owned()
//             })
//             .collect_vec();
//         if let Some(side_panel_state) = state.side_panel.as_mut() {
//             let side_panel = EnumeratedList::default().items(tab_titles).title("Tabs");
//             side_panel.render(area, buf, side_panel_state);
//         }
//     }
// }
