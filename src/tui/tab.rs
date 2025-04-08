use ratatui::widgets::{Block, BorderType, Borders, StatefulWidget, Widget};

use crate::misc::globals::{sql, theme};

use super::{
    schema::{Schema, SchemaState},
    status_bar::{StatusBar, Tag},
    tabular::{Tabular, TabularState},
};

#[derive(Debug)]
pub enum Content {
    Tabular(TabularState),
    Schema(SchemaState),
}

impl Content {
    pub fn tabular(&self) -> Option<&TabularState> {
        if let Content::Tabular(tabular) = self {
            Some(tabular)
        } else {
            None
        }
    }

    pub fn tabular_mut(&mut self) -> Option<&mut TabularState> {
        if let Content::Tabular(tabular) = self {
            Some(tabular)
        } else {
            None
        }
    }

    pub fn schema(&self) -> Option<&SchemaState> {
        if let Content::Schema(schema) = self {
            Some(schema)
        } else {
            None
        }
    }

    pub fn schema_mut(&mut self) -> Option<&mut SchemaState> {
        if let Content::Schema(schema) = self {
            Some(schema)
        } else {
            None
        }
    }

    pub fn tick(&mut self) {
        match self {
            Content::Tabular(tab_content_state) => tab_content_state.tick(),
            Content::Schema(_schema_state) => (),
        }
    }
}

impl From<TabularState> for Content {
    fn from(value: TabularState) -> Self {
        Content::Tabular(value)
    }
}

impl From<SchemaState> for Content {
    fn from(value: SchemaState) -> Self {
        Content::Schema(value)
    }
}

#[derive(Debug)]
pub struct TabState {
    tabulars: Vec<Content>,
    idx: usize,
}

impl TabState {
    pub fn add(&mut self, tabular: impl Into<Content>) {
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

    pub fn selected(&self) -> Option<&Content> {
        self.tabulars.get(self.idx)
    }

    pub fn selected_mut(&mut self) -> Option<&mut Content> {
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

    pub fn iter(&self) -> impl Iterator<Item = &Content> {
        self.tabulars.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Content> {
        self.tabulars.iter_mut()
    }
}

impl FromIterator<TabularState> for TabState {
    fn from_iter<T: IntoIterator<Item = TabularState>>(iter: T) -> Self {
        Self {
            tabulars: iter.into_iter().map(Into::into).collect(),
            idx: 0,
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
        let status_bar = match state.selected() {
            Some(Content::Tabular(tabular)) => StatusBar::default()
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
                )),

            Some(Content::Schema(_)) => StatusBar::default()
                .tag(Tag::new(
                    "Tab",
                    format!("{} / {}", state.idx + 1, state.len()),
                ))
                .tag(Tag::new("Tables", sql().schema().len().to_string())),
            None => StatusBar::default().tag(Tag::new(
                "Tab",
                format!("{} / {}", state.idx + 1, state.len()),
            )),
        };

        // draw block with status bar
        let area = {
            let blk = Block::bordered()
                .border_type(BorderType::Rounded)
                .border_style(theme().block())
                .borders(if self.borders {
                    Borders::all()
                } else {
                    Borders::empty()
                })
                .title_bottom(status_bar);
            let new = blk.inner(area);
            blk.render(area, buf);
            new
        };

        // draw tab content
        match state.selected_mut() {
            Some(Content::Tabular(state)) => {
                Tabular::default().render(area, buf, state);
            }
            Some(Content::Schema(schema_state)) => {
                Schema::default().render(area, buf, schema_state)
            }
            None => (),
        }
    }
}
