use polars::frame::DataFrame;
use ratatui::{
    layout::{Constraint, Layout, Margin, Rect},
    widgets::{Block, BorderType, Borders, StatefulWidget},
};

use super::{
    data_frame_table::{DataFrameTable, DataFrameTableState},
    search_bar::{SearchBar, SearchBarState},
    sheet::{Sheet, SheetState},
    status_bar::{StatusBar, StatusBarTag},
};
use crate::{misc::config::theme, misc::polars_ext::GetSheetSections};

#[derive(Debug, Default)]
pub enum Modal {
    Sheet(SheetState),
    SearchBar(SearchBarState),
    #[default]
    None,
}

impl Modal {
    pub fn sheet(&self) -> Option<&SheetState> {
        if let Modal::Sheet(sheet) = self {
            Some(sheet)
        } else {
            None
        }
    }

    pub fn sheet_mut(&mut self) -> Option<&mut SheetState> {
        if let Modal::Sheet(sheet) = self {
            Some(sheet)
        } else {
            None
        }
    }

    pub fn into_sheet(self) -> Option<SheetState> {
        if let Modal::Sheet(sheet) = self {
            Some(sheet)
        } else {
            None
        }
    }

    pub fn search_bar(&self) -> Option<&SearchBarState> {
        if let Modal::SearchBar(search_bar) = self {
            Some(search_bar)
        } else {
            None
        }
    }

    pub fn search_bar_mut(&mut self) -> Option<&mut SearchBarState> {
        if let Modal::SearchBar(search_bar) = self {
            Some(search_bar)
        } else {
            None
        }
    }

    pub fn into_search_bar(self) -> Option<SearchBarState> {
        if let Modal::SearchBar(search_bar) = self {
            Some(search_bar)
        } else {
            None
        }
    }

    pub fn is_none(&self) -> bool {
        matches!(self, Modal::None)
    }
}

#[derive(Debug)]
pub enum Source {
    Help,
    Schema,
    Name(String),
    Query(String),
}

impl AsRef<str> for Source {
    fn as_ref(&self) -> &str {
        match self {
            Source::Help => "Help",
            Source::Schema => "Schema",
            Source::Name(name) => name.as_str(),
            Source::Query(query) => query.as_str(),
        }
    }
}

#[derive(Debug)]
pub struct TabularState {
    table: DataFrameTableState,
    modal: Modal,
    source: Source,
}

impl TabularState {
    /// Constructs a new instance of [`App`].
    pub fn new(data_frame: DataFrame, tabular_source: Source) -> Self {
        Self {
            table: DataFrameTableState::new(data_frame.clone()),
            modal: Default::default(),
            source: tabular_source,
        }
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&mut self) {
        match &mut self.modal {
            Modal::SearchBar(search_bar) => {
                if let Some(df) = search_bar.search().latest() {
                    self.table.set_data_frame(df);
                }
            }
            Modal::Sheet(_) => (),
            Modal::None => (),
        }
    }

    pub fn table(&self) -> &DataFrameTableState {
        &self.table
    }

    pub fn table_mut(&mut self) -> &mut DataFrameTableState {
        &mut self.table
    }

    pub fn tabular_source(&self) -> &Source {
        &self.source
    }

    pub fn show_sheet(&mut self) {
        self.modal = Modal::Sheet(Default::default());
    }

    pub fn show_search(&mut self) {
        self.modal = Modal::SearchBar(SearchBarState::new(self.table.data_frame().clone()));
    }

    pub fn modal(&self) -> &Modal {
        &self.modal
    }

    pub fn modal_mut(&mut self) -> &mut Modal {
        &mut self.modal
    }

    pub fn modal_take(&mut self) -> Modal {
        std::mem::replace(&mut self.modal, Modal::None)
    }
}

pub struct Tabular {
    status_bar: StatusBar,
    borders: bool,
}

impl Tabular {
    pub fn new() -> Self {
        Self {
            status_bar: StatusBar::new(),
            borders: true,
        }
    }

    pub fn with_tag(mut self, tag: StatusBarTag) -> Self {
        self.status_bar = self.status_bar.with_tag(tag);
        self
    }

    pub fn with_borders(mut self, border: bool) -> Self {
        self.borders = border;
        self
    }
}

impl Default for Tabular {
    fn default() -> Self {
        Self::new()
    }
}

impl StatefulWidget for Tabular {
    type State = TabularState;

    fn render(self, area: Rect, buf: &mut ratatui::prelude::Buffer, state: &mut Self::State) {
        let (search_bar_area, table_area) = match state.modal {
            Modal::SearchBar(_) => {
                let [a0, a1] =
                    Layout::vertical([Constraint::Length(3), Constraint::Fill(1)]).areas(area);
                (a0, a1)
            }
            _ => (Rect::default(), area),
        };
        DataFrameTable::new()
            .with_block(
                Block::new()
                    .borders(if self.borders {
                        Borders::all()
                    } else {
                        Borders::empty()
                    })
                    .border_style(theme().block())
                    .border_type(BorderType::Rounded)
                    .title_bottom(self.status_bar.with_tags([
                        match &state.source {
                            Source::Help => StatusBarTag::new("App", "Help"),
                            Source::Schema => StatusBarTag::new("App", "Schema"),
                            Source::Name(name) => StatusBarTag::new("Table", name.to_owned()),
                            Source::Query(query) => StatusBarTag::new("Query", query.to_owned()),
                        },
                        StatusBarTag::new(
                            "Auto-Fit",
                            if !state.table.expanded() {
                                "Yes"
                            } else {
                                " No"
                            },
                        ),
                        StatusBarTag::new(
                            "Row",
                            format!(
                                "{:>width$}",
                                state.table.selected() + 1,
                                width = state.table.height().to_string().len()
                            ),
                        ),
                        StatusBarTag::new(
                            "Shape",
                            format!(
                                "{} x {}",
                                state.table.height(),
                                state.table.data_frame().width()
                            ),
                        ),
                    ])),
            )
            .render(table_area, buf, &mut state.table);

        match &mut state.modal {
            Modal::Sheet(sheet_state) => {
                let area = area.inner(Margin::new(13, 3));
                let sections = state
                    .table
                    .data_frame()
                    .get_sheet_sections(state.table.selected());
                Sheet::new()
                    .with_sections(sections)
                    .render(area, buf, sheet_state);
            }

            Modal::SearchBar(search_bar_state) => {
                SearchBar::new().with_selection(true).render(
                    search_bar_area,
                    buf,
                    search_bar_state,
                );
            }

            _ => (),
        }
    }
}
