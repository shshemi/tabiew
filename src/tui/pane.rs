use crossterm::event::{KeyCode, KeyModifiers};
use polars::frame::DataFrame;
use ratatui::layout::{Constraint, Flex, Layout, Margin, Rect};

use super::{search_bar::SearchBar, sheet::Sheet};
use crate::{
    handler::action::Action,
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
        table::Table,
    },
};

#[derive(Debug)]
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
}

impl Modal {
    fn responder(&mut self) -> &mut dyn Component {
        match self {
            Modal::Sheet(sheet) => sheet,
            Modal::SearchBar(search_bar) => search_bar,
            Modal::DataFrameInfo(data_frame_info) => data_frame_info,
            Modal::ScatterPlot(scatter_plot_state) => scatter_plot_state,
            Modal::HistogramPlot(histogram_plot_state) => histogram_plot_state,
            Modal::InlineQuery(inline_query) => inline_query,
            Modal::GoToLine(go_to_line) => go_to_line,
            Modal::ExportWizard(export_wizard) => export_wizard,
            Modal::HistogramWizard(histogram_wizard) => histogram_wizard,
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
    table: Table,
    modal: Option<Modal>,
    table_type: TableType,
}

impl Pane {
    /// Constructs a new instance of [`App`].
    pub fn new(data_frame: DataFrame, table_type: TableType) -> Self {
        Self {
            table: Table::new(data_frame)
                .striped()
                .with_selected(0)
                .with_show_header(true)
                .with_show_gutter(true)
                .with_col_space(2)
                .with_extended_view_mode(),
            modal: None,
            table_type,
        }
    }

    pub fn table(&self) -> &Table {
        &self.table
    }

    // pub fn table_mut(&mut self) -> &mut DataFrameTableState {
    //     &mut self.table
    // }

    pub fn table_type(&self) -> &TableType {
        &self.table_type
    }

    pub fn show_sheet(&mut self) {
        if let Some(sections) = self.table.selected_sheet_section() {
            self.modal = Some(Modal::Sheet(Sheet::new(sections)));
        }
    }

    pub fn show_fuzzy_search(&mut self) {
        self.modal = Some(Modal::SearchBar(SearchBar::fuzzy(
            self.table.data_frame().clone(),
        )));
    }

    pub fn show_exact_search(&mut self) {
        self.modal = Some(Modal::SearchBar(SearchBar::exact(
            self.table.data_frame().clone(),
        )));
    }

    pub fn show_data_frame_info(&mut self) {
        match &self.table_type {
            TableType::Help => (),
            TableType::Name(name) => {
                if let Some(input) = sql().schema().get(name).map(|info| info.source()).cloned() {
                    self.modal = Some(Modal::DataFrameInfo(DataFrameInfo::new(
                        self.table.data_frame(),
                        input,
                    )))
                }
            }
            TableType::Query(_) => {
                self.modal = Some(Modal::DataFrameInfo(DataFrameInfo::new(
                    self.table.data_frame(),
                    Source::User,
                )))
            }
        }
    }

    pub fn show_scatter_plot(&mut self, scatter: ScatterPlot) {
        self.modal = Some(Modal::ScatterPlot(scatter))
    }

    pub fn show_histogram_plot(&mut self, hist: HistogramPlot) {
        self.modal = Some(Modal::HistogramPlot(hist))
    }

    pub fn show_inline_query(&mut self, query_type: InlineQueryType) {
        self.modal = Some(Modal::InlineQuery(InlineQuery::new(query_type)));
    }

    pub fn show_go_to_line(&mut self) {
        if let Some(selected) = self.table.selected() {
            self.modal = Some(Modal::GoToLine(GoToLine::new(selected)))
        }
    }

    pub fn show_go_to_line_with_value(&mut self, value: usize) {
        if let Some(selected) = self.table.selected() {
            self.modal = Some(Modal::GoToLine(GoToLine::new(selected).with_value(value)))
        }
    }

    pub fn show_export_data_frame(&mut self) {
        self.modal = Some(Modal::ExportWizard(Default::default()))
    }

    pub fn show_histogram_wizard(&mut self) {
        self.modal = Some(Modal::HistogramWizard(HistogramWizard::new(
            self.table.data_frame(),
        )))
    }
}

impl Component for Pane {
    fn render(
        &mut self,
        area: Rect,
        buf: &mut ratatui::prelude::Buffer,
        focus_state: super::component::FocusState,
    ) {
        match &mut self.modal {
            Some(Modal::Sheet(sheet_state)) => {
                // DataFrameTable::new().render(area, buf, &mut self.table);
                self.table.render(area, buf, focus_state);
                let area = area.inner(Margin::new(13, 3));
                sheet_state.render(area, buf, focus_state);
            }
            Some(Modal::SearchBar(search_bar_state)) => {
                let [search_area, table_area] =
                    Layout::vertical([Constraint::Length(3), Constraint::Fill(1)]).areas(area);
                // DataFrameTable::new().render(table_area, buf, &mut self.table);
                self.table.render(table_area, buf, focus_state);
                search_bar_state.render(search_area, buf, focus_state);
            }
            Some(Modal::DataFrameInfo(data_frame_info)) => {
                //
                // DataFrameTable::new().render(area, buf, &mut self.table);
                self.table.render(area, buf, focus_state);
                let [area] = Layout::horizontal([Constraint::Length(100)])
                    .flex(Flex::Center)
                    .areas(area);
                let [_, area] = Layout::vertical([Constraint::Length(3), Constraint::Length(25)])
                    // .flex(Flex::Center)
                    .areas(area);
                data_frame_info.render(area, buf, focus_state);
            }
            Some(Modal::ScatterPlot(state)) => {
                // DataFrameTable::new().render(area, buf, &mut self.table);
                self.table.render(area, buf, focus_state);
                state.render(area, buf, focus_state);
            }
            Some(Modal::HistogramPlot(state)) => {
                // DataFrameTable::new().render(area, buf, &mut self.table);
                self.table.render(area, buf, focus_state);
                let [area] = Layout::horizontal([Constraint::Length(122)])
                    .flex(Flex::Center)
                    .areas(area);
                let [_, area] =
                    Layout::vertical([Constraint::Length(3), Constraint::Length(40)]).areas(area);
                state.render(area, buf, focus_state);
            }
            Some(Modal::InlineQuery(state)) => {
                // DataFrameTable::new().render(area, buf, &mut self.table);
                self.table.render(area, buf, focus_state);
                state.render(area, buf, focus_state);
                // InlineQuery::default().render(area, buf, state);
            }
            Some(Modal::GoToLine(state)) => {
                // DataFrameTable::new().render(area, buf, &mut self.table);
                self.table.render(area, buf, focus_state);
                state.render(area, buf, focus_state);
                // GoToLine::default().render(area, buf, state);
            }
            Some(Modal::ExportWizard(state)) => {
                // DataFrameTable::new().render(area, buf, &mut self.table);
                self.table.render(area, buf, focus_state);
                state.render(area, buf, focus_state);
                // ExportWizard::default().render(area, buf, state);
            }
            Some(Modal::HistogramWizard(state)) => {
                // DataFrameTable::new().render(area, buf, &mut self.table);
                self.table.render(area, buf, focus_state);
                state.render(area, buf, focus_state);
                // HistogramWizard::default().render(area, buf, state);
            }
            None => self.table.render(area, buf, focus_state),
        }
    }

    fn handle(&mut self, event: crossterm::event::KeyEvent) -> bool {
        if let Some(modal) = self.modal.as_mut() {
            modal.responder().handle(event)
        } else {
            self.table.handle(event)
                | match (event.code, event.modifiers) {
                    (KeyCode::Enter, KeyModifiers::NONE) => {
                        self.show_sheet();
                        true
                    }
                    _ => false,
                }
        }
    }

    fn update(&mut self, action: &crate::handler::action::Action) {
        if let Some(modal) = self.modal.as_mut() {
            modal.responder().update(action);
        }
        self.table.update(action);
        #[allow(clippy::single_match)]
        match action {
            Action::PaneDismissModal => self.modal = None,
            Action::PaneTableSelectUp => {
                self.table.select_up();
                if let Some(Modal::Sheet(sheet)) = self.modal.as_mut()
                    && let Some(sections) = self.table.selected_sheet_section()
                {
                    sheet.set_sections(sections);
                }
            }
            Action::PaneTableSelectDown => {
                self.table.select_down();
                if let Some(Modal::Sheet(sheet)) = self.modal.as_mut()
                    && let Some(sections) = self.table.selected_sheet_section()
                {
                    sheet.set_sections(sections);
                }
            }
            _ => (),
        }
    }

    fn tick(&mut self) {
        match &mut self.modal {
            Some(Modal::SearchBar(search_bar)) => {
                if let Some(df) = search_bar.searcher().latest() {
                    self.table.set_data_frame(df);
                }
            }
            Some(Modal::Sheet(_)) => (),
            Some(Modal::DataFrameInfo(_)) => (),
            Some(Modal::ScatterPlot(_)) => (),
            Some(Modal::HistogramPlot(_)) => (),
            Some(Modal::InlineQuery(_)) => (),
            Some(Modal::GoToLine(_)) => (),
            Some(Modal::ExportWizard(_)) => (),
            Some(Modal::HistogramWizard(_)) => (),
            None => (),
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
