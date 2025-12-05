use crossterm::event::{KeyCode, KeyModifiers};

use polars::frame::DataFrame;
use ratatui::layout::{Constraint, Flex, Layout, Margin, Rect};

use super::{search_bar::SearchBar, sheet::Sheet};
use crate::{
    AppResult,
    handler::message::Message,
    misc::{
        globals::sql, non_empty_stack::NonEmptyStack, polars_ext::PlotData, sql::Source,
        type_ext::UnwrapOrEnqueueError,
    },
    tui::{
        component::Component,
        plots::{histogram_plot::HistogramPlot, scatter_plot::ScatterPlot},
        popups::{
            data_frame_info::DataFrameInfo,
            export_wizard::ExportWizard,
            go_to_line::GoToLine,
            histogram_wizard::HistogramWizard,
            inline_query::{InlineQuery, InlineQueryType},
            scatter_plot_wizard::{self, ScatterPlotWizard},
            wizard::Wizard,
        },
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
    ScatterPlotWizard(ScatterPlotWizard),
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
            Modal::ScatterPlotWizard(wizard) => wizard,
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
    tstack: NonEmptyStack<Table>,
    modal: Option<Modal>,
    table_type: TableType,
}

impl Pane {
    /// Constructs a new instance of [`App`].
    pub fn new(data_frame: DataFrame, table_type: TableType) -> Self {
        Self {
            tstack: NonEmptyStack::new(
                Table::new(data_frame.clone())
                    .striped()
                    .with_selected(0)
                    .with_show_header(true)
                    .with_show_gutter(true)
                    .with_col_space(2)
                    .with_extended_view_mode(),
            ),
            modal: None,
            table_type,
        }
    }

    pub fn table(&self) -> &Table {
        self.tstack.last()
    }

    pub fn table_type(&self) -> &TableType {
        &self.table_type
    }

    pub fn show_sheet(&mut self) {
        if let Some(sections) = self.tstack.last().selected_sheet_sections() {
            self.modal = Some(Modal::Sheet(Sheet::new(sections)));
        }
    }

    fn show_fuzzy_search(&mut self) {
        let tbl = self.tstack.last().to_owned();
        self.tstack.push(tbl);
        self.modal = Some(Modal::SearchBar(SearchBar::fuzzy(
            self.tstack.last().data_frame().clone(),
        )));
    }

    fn show_exact_search(&mut self) {
        let tbl = self.tstack.last().to_owned();
        self.tstack.push(tbl);
        self.modal = Some(Modal::SearchBar(SearchBar::exact(
            self.tstack.last().data_frame().clone(),
        )));
    }

    fn show_data_frame_info(&mut self) {
        match &self.table_type {
            TableType::Help => (),
            TableType::Name(name) => {
                if let Some(input) = sql().schema().get(name).map(|info| info.source()).cloned() {
                    self.modal = Some(Modal::DataFrameInfo(DataFrameInfo::new(
                        self.tstack.last().data_frame(),
                        input,
                    )))
                }
            }
            TableType::Query(_) => {
                self.modal = Some(Modal::DataFrameInfo(DataFrameInfo::new(
                    self.tstack.last().data_frame(),
                    Source::User,
                )))
            }
        }
    }

    fn show_scatter_plot(
        &mut self,
        x_label: String,
        y_label: String,
        group_by: Option<&str>,
    ) -> AppResult<()> {
        let df = self.tstack.last().data_frame();
        let plot = if let Some(group_by) = group_by {
            let (data, groups) = df.scatter_plot_data_grouped(&x_label, &y_label, group_by)?;
            ScatterPlot::new(x_label, y_label, data)?.with_groups(groups)
        } else {
            let data = df.scatter_plot_data(&x_label, &y_label)?;
            ScatterPlot::new(x_label, y_label, data)?
        };
        self.modal = Some(Modal::ScatterPlot(plot));
        Ok(())
    }

    fn show_inline_query(&mut self, query_type: InlineQueryType) {
        self.modal = Some(Modal::InlineQuery(InlineQuery::new(
            self.tstack.last().data_frame().clone(),
            query_type,
        )));
    }

    fn show_go_to_line(&mut self) {
        if let Some(selected) = self.tstack.last().selected() {
            self.modal = Some(Modal::GoToLine(GoToLine::new(selected)))
        }
    }

    fn show_go_to_line_with_value(&mut self, value: usize) {
        if let Some(selected) = self.tstack.last().selected() {
            self.modal = Some(Modal::GoToLine(GoToLine::new(selected).with_value(value)))
        }
    }

    fn show_export_wizard(&mut self) {
        self.modal = Some(Modal::ExportWizard(ExportWizard::new(
            self.tstack.last().data_frame().clone(),
        )))
    }

    fn show_histogram_wizard(&mut self) {
        self.modal = Some(Modal::HistogramWizard(HistogramWizard::new(
            self.tstack.last().data_frame(),
        )))
    }

    fn show_scatter_plot_wizard(&mut self) {
        self.modal = Some(Modal::ScatterPlotWizard(Wizard::new(
            scatter_plot_wizard::State::new(self.tstack.last().data_frame().clone()),
        )))
    }

    fn dismiss_model(&mut self) {
        self.modal = None
    }

    fn select_up(&mut self) {
        self.tstack.last_mut().select_up();
        self.update_sheet();
    }

    fn select_down(&mut self) {
        self.tstack.last_mut().select_down();
        self.update_sheet();
    }

    fn select(&mut self, idx: usize) {
        self.tstack.last_mut().select(idx);
        self.update_sheet();
    }

    fn update_sheet(&mut self) {
        if let Some(Modal::Sheet(sheet)) = self.modal.as_mut()
            && let Some(sections) = self.tstack.last_mut().selected_sheet_sections()
        {
            sheet.set_sections(sections);
        }
    }

    fn push_data_frame(&mut self, df: DataFrame) {
        self.tstack.push(
            Table::new(df)
                .striped()
                .with_selected(0)
                .with_show_header(true)
                .with_show_gutter(true)
                .with_col_space(2)
                .with_extended_view_mode(),
        );
    }

    fn pop_data_frame(&mut self) {
        self.tstack.pop();
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
                self.tstack.last_mut().render(area, buf, focus_state);
                let area = area.inner(Margin::new(13, 3));
                sheet_state.render(area, buf, focus_state);
            }
            Some(Modal::SearchBar(search_bar_state)) => {
                let [search_area, table_area] =
                    Layout::vertical([Constraint::Length(3), Constraint::Fill(1)]).areas(area);
                // DataFrameTable::new().render(table_area, buf, &mut self.table);
                self.tstack.last_mut().render(table_area, buf, focus_state);
                search_bar_state.render(search_area, buf, focus_state);
            }
            Some(Modal::DataFrameInfo(data_frame_info)) => {
                //
                // DataFrameTable::new().render(area, buf, &mut self.table);
                self.tstack.last_mut().render(area, buf, focus_state);
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
                self.tstack.last_mut().render(area, buf, focus_state);
                state.render(area, buf, focus_state);
            }
            Some(Modal::HistogramPlot(state)) => {
                // DataFrameTable::new().render(area, buf, &mut self.table);
                self.tstack.last_mut().render(area, buf, focus_state);
                let [area] = Layout::horizontal([Constraint::Length(122)])
                    .flex(Flex::Center)
                    .areas(area);
                let [_, area] =
                    Layout::vertical([Constraint::Length(3), Constraint::Length(40)]).areas(area);
                state.render(area, buf, focus_state);
            }
            Some(Modal::InlineQuery(state)) => {
                // DataFrameTable::new().render(area, buf, &mut self.table);
                self.tstack.last_mut().render(area, buf, focus_state);
                state.render(area, buf, focus_state);
                // InlineQuery::default().render(area, buf, state);
            }
            Some(Modal::GoToLine(state)) => {
                // DataFrameTable::new().render(area, buf, &mut self.table);
                self.tstack.last_mut().render(area, buf, focus_state);
                state.render(area, buf, focus_state);
                // GoToLine::default().render(area, buf, state);
            }
            Some(Modal::ExportWizard(state)) => {
                // DataFrameTable::new().render(area, buf, &mut self.table);
                self.tstack.last_mut().render(area, buf, focus_state);
                state.render(area, buf, focus_state);
                // ExportWizard::default().render(area, buf, state);
            }
            Some(Modal::HistogramWizard(state)) => {
                // DataFrameTable::new().render(area, buf, &mut self.table);
                self.tstack.last_mut().render(area, buf, focus_state);
                state.render(area, buf, focus_state);
                // HistogramWizard::default().render(area, buf, state);
            }
            Some(Modal::ScatterPlotWizard(state)) => {
                self.tstack.last_mut().render(area, buf, focus_state);
                state.render(area, buf, focus_state);
            }
            None => self.tstack.last_mut().render(area, buf, focus_state),
        }
    }

    fn handle(&mut self, event: crossterm::event::KeyEvent) -> bool {
        if let Some(modal) = self.modal.as_mut() {
            modal.responder().handle(event)
        } else {
            self.tstack.last_mut().handle(event)
                | match (event.code, event.modifiers) {
                    (KeyCode::Enter, KeyModifiers::NONE) => {
                        self.show_sheet();
                        true
                    }
                    (KeyCode::Char('e'), KeyModifiers::NONE) => {
                        self.tstack.last_mut().toggle_view_mode();
                        true
                    }
                    (KeyCode::Char('1'), KeyModifiers::NONE) => {
                        self.show_go_to_line_with_value(1);
                        true
                    }
                    (KeyCode::Char('2'), KeyModifiers::NONE) => {
                        self.show_go_to_line_with_value(2);
                        true
                    }
                    (KeyCode::Char('3'), KeyModifiers::NONE) => {
                        self.show_go_to_line_with_value(3);
                        true
                    }
                    (KeyCode::Char('4'), KeyModifiers::NONE) => {
                        self.show_go_to_line_with_value(4);
                        true
                    }
                    (KeyCode::Char('5'), KeyModifiers::NONE) => {
                        self.show_go_to_line_with_value(5);
                        true
                    }
                    (KeyCode::Char('6'), KeyModifiers::NONE) => {
                        self.show_go_to_line_with_value(6);
                        true
                    }
                    (KeyCode::Char('7'), KeyModifiers::NONE) => {
                        self.show_go_to_line_with_value(7);
                        true
                    }
                    (KeyCode::Char('8'), KeyModifiers::NONE) => {
                        self.show_go_to_line_with_value(8);
                        true
                    }
                    (KeyCode::Char('9'), KeyModifiers::NONE) => {
                        self.show_go_to_line_with_value(9);
                        true
                    }
                    (KeyCode::Char('I'), KeyModifiers::SHIFT) => {
                        self.show_data_frame_info();
                        true
                    }
                    (KeyCode::Char('/'), KeyModifiers::NONE) => {
                        self.show_fuzzy_search();
                        true
                    }
                    (KeyCode::Char('?'), KeyModifiers::NONE)
                    | (KeyCode::Char('?'), KeyModifiers::SHIFT) => {
                        self.show_exact_search();
                        true
                    }
                    (KeyCode::Char('q'), KeyModifiers::NONE) => {
                        if self.tstack.len_without_base() > 0 {
                            self.pop_data_frame();
                            true
                        } else {
                            false
                        }
                    }
                    _ => false,
                }
        }
    }

    fn update(&mut self, action: &crate::handler::message::Message) {
        if let Some(modal) = self.modal.as_mut() {
            modal.responder().update(action);
        }
        self.tstack.last_mut().update(action);
        match action {
            Message::PaneShowInlineSelect => self.show_inline_query(InlineQueryType::Select),
            Message::PaneShowInlineFilter => self.show_inline_query(InlineQueryType::Filter),
            Message::PaneShowInlineOrder => self.show_inline_query(InlineQueryType::Order),
            Message::PaneShowExportWizard => self.show_export_wizard(),
            Message::PaneShowHistogramWizard => self.show_histogram_wizard(),
            Message::PaneShowScatterPlot(x, y, grp) => self
                .show_scatter_plot(x.to_owned(), y.to_owned(), grp.as_deref())
                .unwrap_or_enqueue_error(),
            Message::PaneShowScatterPlotWizard => self.show_scatter_plot_wizard(),
            Message::PaneDismissModal => self.dismiss_model(),
            Message::PaneTableSelectUp => self.select_up(),
            Message::PaneTableSelectDown => self.select_down(),
            Message::PanePushDataFrame(df) => self.push_data_frame(df.clone()),
            Message::PanePopDataFrame => self.pop_data_frame(),
            Message::PaneTableSelect(idx) => self.select(*idx),
            _ => (),
        }
    }

    fn tick(&mut self) {
        match &mut self.modal {
            Some(Modal::SearchBar(search_bar)) => {
                if let Some(df) = search_bar.searcher().latest() {
                    self.tstack.last_mut().set_data_frame(df);
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
            Some(Modal::ScatterPlotWizard(_)) => (),
            None => (),
        }
    }
}
