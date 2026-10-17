use crossterm::event::{KeyCode, KeyModifiers};

use ratatui::{
    layout::{Constraint, Layout, Rect},
    text::Line,
    widgets::{Block, Clear, Paragraph, Widget},
};

use crate::{
    handler::message::Message,
    misc::config::theme,
    tui::{
        app_default::AppDefault,
        component::{Component, FocusState},
        icons,
        pane::TableDescription,
        widgets::tabiew_logo::TabiewLogo,
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
    pub fn new(panes: Vec<Pane>) -> Self {
        Self {
            panes,
            idx: 0,
            switcher: None,
        }
    }

    pub fn selected(&self) -> Option<&Pane> {
        self.panes.get(self.idx)
    }

    pub fn iter(&self) -> impl Iterator<Item = &Pane> {
        self.panes.iter()
    }

    fn add(&mut self, tabular: Pane) {
        self.panes.push(tabular);
        self.idx = self.panes.len().saturating_sub(1);
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

    fn select(&mut self, idx: usize) {
        if let Some(switcher) = self.switcher.as_mut() {
            switcher.select(idx);
        }
        self.idx = idx;
    }

    fn close_selected(&mut self) {
        if !self.is_empty() {
            self.remove_selected();
        }
    }

    fn remove_selected(&mut self) {
        if self.idx < self.panes.len() {
            self.panes.remove(self.idx);
        }
        self.sync_switcher();
    }

    fn remove(&mut self, idx: usize) {
        if idx < self.panes.len() {
            self.panes.remove(idx);
            if idx < self.idx {
                self.idx = self.idx().saturating_sub(1);
            }
            self.idx = self.idx.min(self.len().saturating_sub(1));
            self.sync_switcher();
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
            icons::TAB.title("Tabs"),
            self.panes
                .iter()
                .map(|pane| pane.title().to_owned())
                .collect(),
            self.idx,
        ));
    }

    fn sync_switcher(&mut self) {
        if let Some(switcher) = self.switcher.as_mut() {
            switcher.set_tabs(
                self.panes
                    .iter()
                    .map(|pane| pane.title().to_owned())
                    .collect(),
            );
            switcher.set_idx(self.idx);
        }
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

        let (sw_area, pane_area) = if self.switcher.is_some() {
            let [tabs_area, table_area] = tabs_table_areas(area);
            (tabs_area, table_area)
        } else {
            (Default::default(), area)
        };

        if let Some(switcher) = self.switcher.as_mut() {
            switcher.render(sw_area, buf, focus_state);
        }

        if let Some(pane) = self.panes.get_mut(self.idx) {
            //
            pane.render(
                pane_area,
                buf,
                if self.switcher.is_some() {
                    FocusState::NotFocused
                } else {
                    focus_state
                },
            );
        } else {
            //
            render_empty(pane_area, buf);
        }

        // render tabular
        // match (self.switcher.as_mut(), self.panes.get_mut(self.idx)) {
        //     (Some(switcher), Some(pane)) => {
        //         pane.render(pane_area, buf, FocusState::NotFocused);
        //         if let Some(tabs_area) = sw_area {
        //             switcher.render(tabs_area, buf, focus_state);
        //         }
        //     }
        //     (Some(switcher), None) => {
        //         if let Some(tabs_area) = sw_area {
        //             switcher.render(tabs_area, buf, focus_state);
        //         }
        //     }
        //     (None, Some(pane)) => {
        //         pane.render(pane_area, buf, focus_state);
        //     }
        //     (None, None) => render_empty(pane_area, buf),
        // }
    }

    fn handle(&mut self, event: crossterm::event::KeyEvent) -> bool {
        if let Some(switcher) = self.switcher.as_mut() {
            switcher.handle(event)
                || match (event.code, event.modifiers) {
                    (KeyCode::Delete, KeyModifiers::NONE) => {
                        self.close_selected();
                        true
                    }
                    _ => false,
                }
        } else {
            self.panes
                .get_mut(self.idx)
                .map(|pane| pane.handle(event))
                .unwrap_or_default()
                || match (event.code, event.modifiers) {
                    (KeyCode::Char('q'), KeyModifiers::NONE) => {
                        if self.is_empty() {
                            Message::Quit.enqueue();
                        } else {
                            self.remove_selected();
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

    fn update(&mut self, action: &Message) {
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
            Message::TabsSelect(idx) => self.select(*idx),
            Message::TabsDismissSwitcher => self.dismiss_tab_switcher(),
            Message::TabsCloseSelected => self.close_selected(),
            Message::TabsRemove(idx) => self.remove(*idx),
            _ => (),
        }
        if let Some(switcher) = self.switcher.as_mut() {
            switcher.update(action);
        }
        if let Some(pane) = self.panes.get_mut(self.idx) {
            pane.update(action);
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

fn render_empty(area: Rect, buf: &mut ratatui::prelude::Buffer) {
    const GAP: u16 = 1;
    let hints = vec![
        Line::from("Press : and choose Import to import and open a file".to_owned()),
        Line::from(format!(
            "or run `tw ctl import <FORMAT> --pid {} <PATH>` from another terminal",
            std::process::id()
        )),
    ];
    let hints_height = hints.len() as u16;

    Clear.render(area, buf);
    let block = Block::app_default();
    let inner = block.inner(area);
    block.render(area, buf);
    let area = inner;

    let logo = TabiewLogo::size(Rect {
        height: area.height.saturating_sub(hints_height + GAP),
        ..area
    });
    let (logo_width, logo_height, gap) = logo.map_or((0, 0, 0), |(w, h)| (w, h, GAP));
    let height = (logo_height + gap + hints_height).min(area.height);
    let top = area.y + (area.height - height) / 2;

    if logo.is_some() {
        TabiewLogo.render(
            Rect {
                x: area.x + (area.width - logo_width) / 2,
                y: top,
                width: logo_width,
                height: logo_height,
            },
            buf,
        );
    }
    Paragraph::new(hints)
        .style(theme().subtext())
        .centered()
        .render(
            Rect {
                y: top + logo_height + gap,
                height: height - logo_height - gap,
                ..area
            },
            buf,
        );
}

fn tabs_table_areas(area: Rect) -> [Rect; 2] {
    const TABS_WIDTH: u16 = 34;
    Layout::horizontal([Constraint::Length(TABS_WIDTH), Constraint::Fill(1)]).areas(area)
}
