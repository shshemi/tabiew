use crossterm::event::{KeyCode, KeyModifiers};

use ratatui::{
    layout::{Constraint, Layout, Rect},
    widgets::{Borders, Widget},
};

use crate::{
    handler::message::Message,
    misc::config::config,
    tui::{
        component::{Component, FocusState},
        pane::TableDescription,
        widgets::{block::Block, status_bar::StatusBar},
    },
};

use super::{pane::Pane, tab_switcher::TabSwitcher};

#[derive(Debug)]
pub struct Tabs {
    panes: Vec<Pane>,
    switcher: Option<TabSwitcher>,
    idx: usize,
}

impl Tabs {
    pub fn selected(&self) -> Option<&Pane> {
        self.panes.get(self.idx)
    }

    pub fn has_unsaved_changes(&self) -> bool {
        self.panes.iter().any(Pane::has_unsaved_changes)
    }

    fn add(&mut self, tabular: Pane) {
        self.panes.push(tabular);
        self.idx = self.panes.len().saturating_sub(1);
    }

    fn len(&self) -> usize {
        self.panes.len()
    }

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn idx(&self) -> usize {
        self.idx
    }

    fn select(&mut self, idx: usize) {
        if let Some(switcher) = self.switcher.as_mut() {
            switcher.select(idx);
        }
        self.idx = idx;
    }

    fn remove_selected(&mut self) {
        if self.idx < self.panes.len() {
            self.panes.remove(self.idx);
        }
        if self.switcher.is_some() {
            self.show_tab_switcher();
        }
    }

    fn select_prev(&mut self) {
        self.select(self.idx().saturating_sub(1));
    }

    fn select_next(&mut self) {
        self.select(
            self.idx()
                .saturating_add(1)
                .min(self.len().saturating_sub(1)),
        );
    }
    fn show_tab_switcher(&mut self) {
        self.switcher = Some(TabSwitcher::new(
            "Tabs",
            self.panes
                .iter()
                .map(|pane| pane.title().to_owned())
                .collect(),
            self.idx,
        ));
    }

    fn dismiss_tab_switcher(&mut self) {
        self.switcher.take();
    }
}

impl Component for Tabs {
    fn render(
        &mut self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        focus_state: super::component::FocusState,
    ) {
        self.idx = self.idx().min(self.len().saturating_sub(1));

        let area = {
            if config().show_table_borders() {
                let blk = Block::default().borders(Borders::all());
                let new = blk.inner(area);
                blk.render(area, buf);
                if let Some(pane) = self.panes.get(self.idx) {
                    let status_bar = StatusBar::new(pane, self.idx, self.len());
                    status_bar.render(
                        Rect {
                            x: area.x + 1,
                            y: area.height.saturating_sub(1),
                            width: area.width.saturating_sub(2),
                            height: 1,
                        },
                        buf,
                    );
                }
                new
            } else {
                let [pane_area, statusbar_area] =
                    Layout::vertical([Constraint::Fill(1), Constraint::Length(1)]).areas(area);

                if let Some(pane) = self.panes.get(self.idx) {
                    let status_bar = StatusBar::new(pane, self.idx, self.len());
                    status_bar.render(statusbar_area, buf);
                }
                pane_area
            }
        };

        // render tabular
        match (self.switcher.as_mut(), self.panes.get_mut(self.idx)) {
            (Some(switcher), Some(pane)) => {
                pane.render(area, buf, FocusState::NotFocused);
                switcher.render(area, buf, focus_state);
            }
            (Some(switcher), None) => {
                switcher.render(area, buf, focus_state);
            }
            (None, Some(pane)) => {
                pane.render(area, buf, focus_state);
            }
            (None, None) => (),
        }
    }

    fn handle(&mut self, event: crossterm::event::KeyEvent) -> bool {
        if let Some(switcher) = self.switcher.as_mut() {
            switcher.handle(event)
        } else {
            self.panes
                .get_mut(self.idx)
                .map(|pane| pane.handle(event))
                .unwrap_or_default()
                || match (event.code, event.modifiers) {
                    (KeyCode::Char('q'), KeyModifiers::NONE) => {
                        if self
                            .panes
                            .get(self.idx)
                            .is_some_and(Pane::has_unsaved_changes)
                        {
                            Message::AppConfirmTabClose.enqueue();
                        } else {
                            self.remove_selected();
                            if self.is_empty() {
                                Message::Quit.enqueue();
                            }
                        }
                        true
                    }
                    (KeyCode::Char('t'), KeyModifiers::NONE) => {
                        self.show_tab_switcher();
                        true
                    }
                    (KeyCode::Char('H'), KeyModifiers::SHIFT)
                    | (KeyCode::Left, KeyModifiers::SHIFT) => {
                        self.select_prev();
                        true
                    }
                    (KeyCode::Char('L'), KeyModifiers::SHIFT)
                    | (KeyCode::Right, KeyModifiers::SHIFT) => {
                        self.select_next();
                        true
                    }
                    _ => false,
                }
        }
    }

    fn update(&mut self, action: &Message, focus_state: FocusState) {
        match action {
            Message::TabsAddNamePane(df, name) => {
                self.add(Pane::new(
                    df.clone(),
                    TableDescription::Table(name.to_owned()),
                ));
            }
            Message::TabsAddQueryPane(df, query) => {
                self.add(Pane::new(
                    df.clone(),
                    TableDescription::Query(query.to_owned()),
                ));
            }
            Message::TabsSelect(idx) if focus_state.is_focused() => self.select(*idx),
            Message::TabsCloseSelected => {
                self.remove_selected();
                if self.is_empty() {
                    Message::Quit.enqueue();
                }
            }
            Message::TabsCloseSelectedIfClean => {
                if !self
                    .panes
                    .get(self.idx)
                    .is_some_and(Pane::has_unsaved_changes)
                {
                    self.remove_selected();
                    if self.is_empty() {
                        Message::Quit.enqueue();
                    }
                }
            }
            Message::TabsDismissSwitcher if focus_state.is_focused() => self.dismiss_tab_switcher(),
            _ => (),
        }
        if let Some(switcher) = self.switcher.as_mut() {
            switcher.update(action, focus_state);
            for pane in self.panes.iter_mut() {
                pane.update(action, FocusState::NotFocused);
            }
        } else {
            for (idx, pane) in self.panes.iter_mut().enumerate() {
                pane.update(
                    action,
                    if idx == self.idx {
                        focus_state
                    } else {
                        FocusState::NotFocused
                    },
                );
            }
        }
    }

    fn tick(&mut self) {
        if let Some(switcher) = self.switcher.as_mut() {
            switcher.tick();
        }
        for pane in self.panes.iter_mut() {
            pane.tick();
        }
    }
}

impl FromIterator<Pane> for Tabs {
    fn from_iter<T: IntoIterator<Item = Pane>>(iter: T) -> Self {
        Self {
            panes: iter.into_iter().collect(),
            idx: 0,
            switcher: None,
        }
    }
}
