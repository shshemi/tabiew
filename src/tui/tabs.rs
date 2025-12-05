use crossterm::event::{KeyCode, KeyModifiers};
use ratatui::widgets::{Borders, Widget};

use crate::{
    handler::message::Message,
    misc::type_ext::VecExt,
    tui::{component::Component, widgets::block::Block},
};

use super::{
    pane::Pane,
    status_bar::{StatusBar, Tag},
    tab_switcher::TabSwitcher,
};

#[derive(Debug)]
pub struct TabsState {
    panes: Vec<Pane>,
    switcher: Option<TabSwitcher>,
    idx: usize,
    borders: bool,
}

impl TabsState {
    fn add(&mut self, tabular: Pane) {
        self.panes.push(tabular);
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

    fn selected(&self) -> Option<&Pane> {
        self.panes.get(self.idx)
    }

    fn selected_mut(&mut self) -> Option<&mut Pane> {
        self.panes.get_mut(self.idx)
    }

    fn remove(&mut self, idx: usize) {
        if idx < self.panes.len() {
            self.panes.remove(idx);
        }
    }

    fn select(&mut self, idx: usize) {
        self.idx = idx;
    }

    fn iter(&self) -> impl Iterator<Item = &Pane> {
        self.panes.iter()
    }

    fn iter_mut(&mut self) -> impl Iterator<Item = &mut Pane> {
        self.panes.iter_mut()
    }

    fn side_panel(&self) -> Option<&TabSwitcher> {
        self.switcher.as_ref()
    }

    fn side_panel_mut(&mut self) -> Option<&mut TabSwitcher> {
        self.switcher.as_mut()
    }

    fn show_tab_switcher(&mut self) {
        self.switcher = Some(TabSwitcher::new(
            "Tabs",
            self.panes
                .iter()
                .map(|pane| pane.table_type().title())
                .collect(),
            self.idx,
        ));
    }

    fn dismiss_tab_switcher(&mut self) {
        self.switcher.take();
    }
}

impl Component for TabsState {
    fn render(
        &mut self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        focus_state: super::component::FocusState,
    ) {
        // fix state (if invalid)
        self.idx = self.idx().min(self.len().saturating_sub(1));

        // build the status bar
        let status_bar = self
            .panes
            .get(self.idx)
            .map(|tabular| {
                StatusBar::default()
                    .tag(Tag::new(
                        "Tab",
                        format!("{} / {}", self.idx + 1, self.len()),
                    ))
                    .tag(match tabular.table_type() {
                        super::TableType::Help => Tag::new("Table", "Help"),
                        super::TableType::Name(name) => Tag::new("Table", name),
                        super::TableType::Query(query) => Tag::new("Query", query),
                    })
                    .tag(Tag::new(
                        "View Mode",
                        match tabular.table().view_mode() {
                            crate::tui::table::ViewMode::Compact => "Compact",
                            crate::tui::table::ViewMode::Expanded(_) => "Expanded",
                        },
                    ))
                    .tag(Tag::new(
                        "Row",
                        format!(
                            "{:>width$}",
                            tabular.table().selected().unwrap_or_default() + 1,
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
        if let Some(pane) = self.panes.get_mut(self.idx) {
            // Pane.render(area, buf, tabular);
            pane.render(area, buf, focus_state);
        }

        if let Some(side_panel) = self.switcher.as_mut() {
            side_panel.render(area, buf, focus_state);
        }
    }

    fn handle(&mut self, event: crossterm::event::KeyEvent) -> bool {
        if let Some(switcher) = self.switcher.as_mut() {
            switcher.handle(event)
        } else {
            self.panes
                .get_mut(self.idx)
                .map(|pane| pane.handle(event))
                .unwrap_or(false)
                || match (event.code, event.modifiers) {
                    (KeyCode::Char('q'), KeyModifiers::NONE) => {
                        self.panes.take(self.idx);
                        if self.is_empty() {
                            Message::Quit.enqueue();
                        }
                        true
                    }
                    (KeyCode::Char('t'), KeyModifiers::NONE) => {
                        self.show_tab_switcher();
                        true
                    }
                    _ => false,
                }
        }
    }

    fn update(&mut self, action: &Message) {
        match action {
            Message::TabsSelect(idx) => self.select(*idx),
            Message::TabsDismissSwitcher => self.dismiss_tab_switcher(),
            _ => (),
        }
        if let Some(switcher) = self.switcher.as_mut() {
            switcher.update(action)
        }
        if let Some(selected) = self.selected_mut() {
            selected.update(action);
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

impl FromIterator<Pane> for TabsState {
    fn from_iter<T: IntoIterator<Item = Pane>>(iter: T) -> Self {
        Self {
            panes: iter.into_iter().collect(),
            idx: 0,
            switcher: None,
            borders: true,
        }
    }
}
