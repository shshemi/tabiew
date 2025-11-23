use crossterm::event::KeyCode;
use polars::frame::DataFrame;
use ratatui::{
    layout::{Constraint, Flex, Layout, Margin, Rect},
    widgets::StatefulWidget,
};

use super::{
    data_frame_table::{DataFrameTable, DataFrameTableState},
    search_bar::SearchBar,
    sheet::Sheet,
};
use crate::{
    misc::{globals::sql, polars_ext::GetSheetSections, sql::Source},
    tui::{
        component::Component,
        plots::{histogram_plot::HistogramPlot, scatter_plot::ScatterPlot},
        popups::{
            export_wizard::ExportWizard,
            go_to_line::GoToLine,
            histogram_wizard::HistogramWizard,
            inline_query::{InlineQuery, InlineQueryType},
        },
        schema::data_frame_info::DataFrameInfo,
    },
};

#[derive(Debug, Default)]
pub enum Modal {
    Sheet(Sheet),
    SearchBar(SearchBar),
    DataFrameInfo(DataFrameInfo),
    ScatterPlot(ScatterPlot),
    HistogramPlot(HistogramPlot),
    InlineQuery(InlineQuery),
    GoToLine(GoToLine),
    ExportWizard(ExportWizard),
    HistogramWizard(HistogramWizard),
    #[default]
    None,
}

impl Modal {
    fn responder(&mut self) -> Option<&mut dyn Component> {
        match self {
            Modal::Sheet(sheet) => Some(sheet),
            Modal::SearchBar(search_bar) => Some(search_bar),
            Modal::DataFrameInfo(data_frame_info) => Some(data_frame_info),
            Modal::ScatterPlot(scatter_plot_state) => Some(scatter_plot_state),
            Modal::HistogramPlot(histogram_plot_state) => Some(histogram_plot_state),
            Modal::InlineQuery(inline_query) => Some(inline_query),
            Modal::GoToLine(go_to_line) => Some(go_to_line),
            Modal::ExportWizard(export_wizard) => Some(export_wizard),
            Modal::HistogramWizard(histogram_wizard) => Some(histogram_wizard),
            Modal::None => None,
        }
    }

    fn take(&mut self) -> Option<Modal> {
        if matches!(self, Modal::None) {
            None
        } else {
            Some(std::mem::take(self))
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TableType {
    Help,
    Name(String),
    Query(String),
}

impl TableType {
    pub fn title(&self) -> String {
        match self {
            TableType::Help => "Help".to_owned(),
            TableType::Name(s) => s.clone(),
            TableType::Query(s) => s.clone(),
        }
    }
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
pub struct Pane {
    table: DataFrameTableState,
    modal: Modal,
    table_type: TableType,
}

impl Pane {
    /// Constructs a new instance of [`App`].
    pub fn new(data_frame: DataFrame, table_type: TableType) -> Self {
        Self {
            table: DataFrameTableState::new(data_frame.clone()),
            modal: Modal::None,
            table_type,
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
        self.modal = Modal::Sheet(Sheet::new(
            self.table
                .data_frame()
                .get_sheet_sections(self.table.selected()),
        ));
    }

    pub fn show_fuzzy_search(&mut self) {
        self.modal = Modal::SearchBar(SearchBar::fuzzy(self.table.data_frame().clone()));
    }

    pub fn show_exact_search(&mut self) {
        self.modal = Modal::SearchBar(SearchBar::exact(self.table.data_frame().clone()));
    }

    pub fn show_data_frame_info(&mut self) {
        match &self.table_type {
            TableType::Help => (),
            TableType::Name(name) => {
                if let Some(input) = sql().schema().get(&name).map(|info| info.source()).cloned() {
                    self.modal =
                        Modal::DataFrameInfo(DataFrameInfo::new(&self.table.data_frame(), input))
                }
            }
            TableType::Query(_) => {
                self.modal =
                    Modal::DataFrameInfo(DataFrameInfo::new(&self.table.data_frame(), Source::User))
            }
        }
    }

    pub fn show_scatter_plot(&mut self, scatter: ScatterPlot) {
        self.modal = Modal::ScatterPlot(scatter)
    }

    pub fn show_histogram_plot(&mut self, hist: HistogramPlot) {
        self.modal = Modal::HistogramPlot(hist)
    }

    pub fn show_inline_query(&mut self, query_type: InlineQueryType) {
        self.modal = Modal::InlineQuery(InlineQuery::new(query_type));
    }

    pub fn show_go_to_line(&mut self) {
        self.modal = Modal::GoToLine(GoToLine::new(self.table.selected()))
    }

    pub fn show_go_to_line_with_value(&mut self, value: usize) {
        self.modal = Modal::GoToLine(GoToLine::new(self.table.selected()).with_value(value))
    }

    pub fn show_export_data_frame(&mut self) {
        self.modal = Modal::ExportWizard(Default::default())
    }

    pub fn show_histogram_wizard(&mut self) {
        self.modal = Modal::HistogramWizard(HistogramWizard::new(self.table.data_frame()))
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

impl Component for Pane {
    fn render(
        &mut self,
        area: Rect,
        buf: &mut ratatui::prelude::Buffer,
        focus_state: super::component::FocusState,
    ) {
        // let (search_bar_area, table_area) = match &self.modal {
        //     Modal::SearchBar(_) => {
        //         let [a0, a1] =
        //             Layout::vertical([Constraint::Length(3), Constraint::Fill(1)]).areas(area);
        //         (a0, a1)
        //     }
        //     _ => (Rect::default(), area),
        // };
        // if let Modal::GoToLine(gtl) = &state.modal {
        //     state.table.select(gtl.value().saturating_sub(1));
        // }
        // DataFrameTable::new().render(table_area, buf, &mut state.table);

        match &mut self.modal {
            Modal::Sheet(sheet_state) => {
                DataFrameTable::new().render(area, buf, &mut self.table);
                let area = area.inner(Margin::new(13, 3));
                sheet_state.render(area, buf, focus_state);
                // let sections = self
                //     .table
                //     .data_frame()
                //     .get_sheet_sections(self.table.selected());
                // Sheet::new()
                //     .with_sections(sections)
                //     .render(area, buf, sheet_state);
            }
            Modal::SearchBar(search_bar_state) => {
                let [search_area, table_area] =
                    Layout::vertical([Constraint::Length(3), Constraint::Fill(1)]).areas(area);
                DataFrameTable::new().render(table_area, buf, &mut self.table);
                search_bar_state.render(search_area, buf, focus_state);
                // SearchBar::new().with_selection(true).render(
                //     search_bar_area,
                //     buf,
                //     search_bar_state,
                // );
            }
            Modal::DataFrameInfo(data_frame_info) => {
                //
                DataFrameTable::new().render(area, buf, &mut self.table);
                let [area] = Layout::horizontal([Constraint::Length(100)])
                    .flex(Flex::Center)
                    .areas(area);
                let [_, area] = Layout::vertical([Constraint::Length(3), Constraint::Length(25)])
                    // .flex(Flex::Center)
                    .areas(area);
                data_frame_info.render(area, buf, focus_state);
                // match &state.table_type {
                //     TableType::Help => (),
                //     TableType::Name(name) => {
                //         if let Some(tbl_info) = sql().schema().get(name) {
                //             let source = tbl_info.source().clone();
                //             DataFrameInfo::new(&TableInfo::new(
                //                 source,
                //                 state.table.data_frame_mut(),
                //             ))
                //             .render(area, buf, data_frame_info);
                //         }
                //     }
                //     TableType::Query(_) => {
                //         DataFrameInfo::new(&TableInfo::new(
                //             Source::User,
                //             state.table.data_frame_mut(),
                //         ))
                //         .render(area, buf, data_frame_info);
                //     }
                // };
            }
            Modal::ScatterPlot(state) => {
                DataFrameTable::new().render(area, buf, &mut self.table);
                state.render(area, buf, focus_state);
            }
            Modal::HistogramPlot(state) => {
                DataFrameTable::new().render(area, buf, &mut self.table);
                let [area] = Layout::horizontal([Constraint::Length(122)])
                    .flex(Flex::Center)
                    .areas(area);
                let [_, area] =
                    Layout::vertical([Constraint::Length(3), Constraint::Length(40)]).areas(area);
                state.render(area, buf, focus_state);
            }
            Modal::InlineQuery(state) => {
                DataFrameTable::new().render(area, buf, &mut self.table);
                state.render(area, buf, focus_state);
                // InlineQuery::default().render(area, buf, state);
            }
            Modal::GoToLine(state) => {
                DataFrameTable::new().render(area, buf, &mut self.table);
                state.render(area, buf, focus_state);
                // GoToLine::default().render(area, buf, state);
            }
            Modal::ExportWizard(state) => {
                DataFrameTable::new().render(area, buf, &mut self.table);
                state.render(area, buf, focus_state);
                // ExportWizard::default().render(area, buf, state);
            }
            Modal::HistogramWizard(state) => {
                DataFrameTable::new().render(area, buf, &mut self.table);
                state.render(area, buf, focus_state);
                // HistogramWizard::default().render(area, buf, state);
            }
            Modal::None => DataFrameTable::new().render(area, buf, &mut self.table),
        }
    }

    fn handle(&mut self, event: crossterm::event::KeyEvent) -> bool {
        self.modal
            .responder()
            .map(|r| r.handle(event))
            .unwrap_or(false)
            || match event.code {
                KeyCode::Esc | KeyCode::Char('q') => self.modal.take().is_some(),
                KeyCode::Char('I') => {
                    self.show_data_frame_info();
                    true
                }
                KeyCode::Up | KeyCode::Char('k') => {
                    //
                    self.table.select_up(1);
                    true
                }
                KeyCode::Down | KeyCode::Char('j') => {
                    self.table.select_down(1);
                    true
                }
                _ => false,
            }
    }

    fn tick(&mut self) {
        match &mut self.modal {
            Modal::SearchBar(search_bar) => {
                if let Some(df) = search_bar.searcher().latest() {
                    self.table.set_data_frame(df);
                }
            }
            Modal::Sheet(_) => (),
            Modal::DataFrameInfo(_) => (),
            Modal::None => (),
            Modal::ScatterPlot(_) => (),
            Modal::HistogramPlot(_) => (),
            Modal::InlineQuery(_) => (),
            Modal::GoToLine(_) => (),
            Modal::ExportWizard(_) => (),
            Modal::HistogramWizard(_) => (),
        }
    }
}

// #[derive(Debug, Default)]
// pub struct Pane;

// impl StatefulWidget for Pane {
//     type State = PaneState;

//     fn render(self, area: Rect, buf: &mut ratatui::prelude::Buffer, state: &mut Self::State) {
//         let (search_bar_area, table_area) = match state.modal {
//             Modal::SearchBar(_) => {
//                 let [a0, a1] =
//                     Layout::vertical([Constraint::Length(3), Constraint::Fill(1)]).areas(area);
//                 (a0, a1)
//             }
//             _ => (Rect::default(), area),
//         };
//         if let Modal::GoToLine(gtl) = &state.modal {
//             state.table.select(gtl.value().saturating_sub(1));
//         }
//         DataFrameTable::new().render(table_area, buf, &mut state.table);

//         match &mut state.modal {
//             Modal::Sheet(sheet_state) => {
//                 let area = area.inner(Margin::new(13, 3));
//                 let sections = state
//                     .table
//                     .data_frame()
//                     .get_sheet_sections(state.table.selected());
//                 Sheet::new()
//                     .with_sections(sections)
//                     .render(area, buf, sheet_state);
//             }
//             Modal::SearchBar(search_bar_state) => {
//                 search_bar_state.render(area, buf, focus_state);
//                 // SearchBar::new().with_selection(true).render(
//                 //     search_bar_area,
//                 //     buf,
//                 //     search_bar_state,
//                 // );
//             }
//             Modal::DataFrameInfo(data_frame_info) => {
//                 //
//                 let [area] = Layout::horizontal([Constraint::Length(100)])
//                     .flex(Flex::Center)
//                     .areas(area);
//                 let [_, area] = Layout::vertical([Constraint::Length(3), Constraint::Length(25)])
//                     // .flex(Flex::Center)
//                     .areas(area);
//                 data_frame_info.render(area, buf, focus_state);
//                 // match &state.table_type {
//                 //     TableType::Help => (),
//                 //     TableType::Name(name) => {
//                 //         if let Some(tbl_info) = sql().schema().get(name) {
//                 //             let source = tbl_info.source().clone();
//                 //             DataFrameInfo::new(&TableInfo::new(
//                 //                 source,
//                 //                 state.table.data_frame_mut(),
//                 //             ))
//                 //             .render(area, buf, data_frame_info);
//                 //         }
//                 //     }
//                 //     TableType::Query(_) => {
//                 //         DataFrameInfo::new(&TableInfo::new(
//                 //             Source::User,
//                 //             state.table.data_frame_mut(),
//                 //         ))
//                 //         .render(area, buf, data_frame_info);
//                 //     }
//                 // };
//             }
//             Modal::ScatterPlot(state) => {
//                 let [area] = Layout::horizontal([Constraint::Length(120)])
//                     .flex(Flex::Center)
//                     .areas(area);
//                 let [_, area] = Layout::vertical([Constraint::Length(3), Constraint::Length(40)])
//                     // .flex(Flex::Center)
//                     .areas(area);
//                 ScatterPlot::default().render(area, buf, state);
//             }
//             Modal::HistogramPlot(state) => {
//                 let [area] = Layout::horizontal([Constraint::Length(122)])
//                     .flex(Flex::Center)
//                     .areas(area);
//                 let [_, area] =
//                     Layout::vertical([Constraint::Length(3), Constraint::Length(40)]).areas(area);
//                 HistogramPlot.render(area, buf, state);
//             }
//             Modal::InlineQuery(state) => {
//                 InlineQuery::default().render(area, buf, state);
//             }
//             Modal::GoToLine(state) => {
//                 GoToLine::default().render(area, buf, state);
//             }
//             Modal::ExportWizard(state) => {
//                 ExportWizard::default().render(area, buf, state);
//             }
//             Modal::HistogramWizard(state) => {
//                 HistogramWizard::default().render(area, buf, state);
//             }
//             Modal::None => (),
//         }
//     }
// }
