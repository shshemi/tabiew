use polars::frame::DataFrame;
use ratatui::{
    layout::{Constraint, Flex, Layout, Margin, Rect},
    widgets::StatefulWidget,
};

use super::{
    data_frame_table::{DataFrameTable, DataFrameTableState},
    search_bar::{SearchBar, SearchBarState},
    sheet::{Sheet, SheetState},
};
use crate::{
    misc::{
        globals::sql,
        polars_ext::GetSheetSections,
        sql::{Source, TableInfo},
    },
    tui::{
        data_frame_info::{DataFrameInfo, DataFrameInfoState},
        scatter_plot::{ScatterPlot, ScatterPlotState},
    },
};

#[derive(Debug, Default)]
pub enum Modal {
    Sheet(SheetState),
    SearchBar(SearchBarState),
    DataFrameInfo(DataFrameInfoState),
    ScatterPlot(ScatterPlotState),
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TableType {
    Help,
    Name(String),
    Query(String),
}

impl AsRef<str> for TableType {
    fn as_ref(&self) -> &str {
        match self {
            TableType::Help => "Help",
            TableType::Name(name) => name.as_str(),
            TableType::Query(query) => query.as_str(),
        }
    }
}

#[derive(Debug)]
pub struct TabContentState {
    table: DataFrameTableState,
    modal: Modal,
    table_type: TableType,
}

impl TabContentState {
    /// Constructs a new instance of [`App`].
    pub fn new(data_frame: DataFrame, table_type: TableType) -> Self {
        Self {
            table: DataFrameTableState::new(data_frame.clone()),
            modal: Default::default(),
            table_type,
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
            Modal::DataFrameInfo(_) => (),
            Modal::None => (),
            Modal::ScatterPlot(_) => (),
        }
    }

    pub fn table(&self) -> &DataFrameTableState {
        &self.table
    }

    pub fn table_mut(&mut self) -> &mut DataFrameTableState {
        &mut self.table
    }

    pub fn table_type(&self) -> &TableType {
        &self.table_type
    }

    pub fn show_sheet(&mut self) {
        self.modal = Modal::Sheet(Default::default());
    }

    pub fn show_fuzzy_search(&mut self) {
        self.modal = Modal::SearchBar(SearchBarState::fuzzy(self.table.data_frame().clone()));
    }

    pub fn show_exact_search(&mut self) {
        self.modal = Modal::SearchBar(SearchBarState::exact(self.table.data_frame().clone()));
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

#[derive(Debug, Default)]
pub struct TabContent {}

impl StatefulWidget for TabContent {
    type State = TabContentState;

    fn render(self, area: Rect, buf: &mut ratatui::prelude::Buffer, state: &mut Self::State) {
        let (search_bar_area, table_area) = match state.modal {
            Modal::SearchBar(_) => {
                let [a0, a1] =
                    Layout::vertical([Constraint::Length(3), Constraint::Fill(1)]).areas(area);
                (a0, a1)
            }
            _ => (Rect::default(), area),
        };
        DataFrameTable::new().render(table_area, buf, &mut state.table);

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
            Modal::DataFrameInfo(data_frame_info) => {
                //
                let [area] = Layout::horizontal([Constraint::Length(100)])
                    .flex(Flex::Center)
                    .areas(area);
                let [_, area] = Layout::vertical([Constraint::Length(3), Constraint::Length(25)])
                    // .flex(Flex::Center)
                    .areas(area);
                match &state.table_type {
                    TableType::Help => todo!(),
                    TableType::Name(name) => {
                        if let Some(tbl_info) = sql().schema().get(name) {
                            let source = tbl_info.source().clone();
                            DataFrameInfo::new(&TableInfo::new(
                                source,
                                state.table.data_frame_mut(),
                            ))
                            .render(area, buf, data_frame_info);
                        }
                    }
                    TableType::Query(_) => {
                        DataFrameInfo::new(&TableInfo::new(
                            Source::User,
                            state.table.data_frame_mut(),
                        ))
                        .render(area, buf, data_frame_info);
                    }
                };
            }
            Modal::ScatterPlot(state) => {
                let [area] = Layout::horizontal([Constraint::Length(120)])
                    .flex(Flex::Center)
                    .areas(area);
                let [_, area] = Layout::vertical([Constraint::Length(3), Constraint::Length(40)])
                    // .flex(Flex::Center)
                    .areas(area);
                ScatterPlot::default().render(area, buf, state);
            }
            Modal::None => (),
        }
    }
}
