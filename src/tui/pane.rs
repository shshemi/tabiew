use crossterm::event::{KeyCode, KeyModifiers};

use polars::frame::DataFrame;
use rand::Rng;
use ratatui::layout::{Constraint, Layout, Margin, Rect};

use super::{search_bar::SearchBar, sheet::Sheet};
use crate::{
    AppResult,
    handler::message::Message,
    misc::{
        config::config, globals::sql, non_empty_stack::NonEmptyStack, polars_ext::DataFrameExt,
        sql::Source, type_ext::UnwrapOrEnqueueError,
    },
    tui::{
        component::{Component, FocusState},
        plots::{histogram_plot::HistogramPlot, scatter_plot::ScatterPlot},
        popups::{
            column_caster_wizard::ColumnCastWizard,
            data_frame_info::DataFrameInfo,
            export_wizard::ExportWizard,
            go_to_line::GoToLine,
            histogram_wizard::{self, HistogramWizard},
            query_picker::{QueryPicker, QueryType},
            scatter_plot_wizard::{self, ScatterPlotWizard},
            table_registerer::TableRegisterer,
            wizard::Wizard,
        },
        table::Table,
    },
};

#[derive(Debug)]
pub struct Pane {
    tstack: NonEmptyStack<Table>,
    dstack: NonEmptyStack<TableDescription>,
    modal: Option<Modal>,
}

impl Pane {
    /// Constructs a new instance of [`App`].
    pub fn new(data_frame: DataFrame, description: TableDescription) -> Self {
        Self {
            tstack: NonEmptyStack::new(table(data_frame)),
            dstack: NonEmptyStack::new(description),
            modal: None,
        }
    }

    pub fn table(&self) -> &Table {
        self.tstack.last()
    }

    pub fn description(&self) -> &TableDescription {
        self.dstack.last()
    }

    pub fn iter_descriptions(&self) -> impl Iterator<Item = &TableDescription> {
        self.dstack.iter()
    }

    pub fn show_sheet(&mut self) {
        if let Some(row) = self.tstack.last().selected() {
            let sections = self.tstack.last().data_frame().get_sheet_sections(row);
            self.modal = Some(Modal::Sheet(Sheet::new(row, sections)));
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
        match &self.dstack.last() {
            TableDescription::Table(desc) => {
                if let Some(input) = sql().schema().get(desc).map(|info| info.source()).cloned() {
                    self.modal = Some(Modal::DataFrameInfo(DataFrameInfo::new(
                        self.tstack.last().data_frame(),
                        input,
                    )))
                }
            }
            TableDescription::Query(_)
            | TableDescription::Filter(_)
            | TableDescription::Order(_)
            | TableDescription::Select(_)
            | TableDescription::Cast(_) => {
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

    fn show_query_picker(&mut self, query_type: QueryType) {
        self.modal = Some(Modal::QueryPicker(QueryPicker::new(
            self.tstack.last().data_frame().clone(),
            query_type,
        )));
    }

    fn show_go_to_line_with_value(&mut self, value: usize) {
        if let Some(selected) = self.tstack.last().selected() {
            self.modal = Some(Modal::GoToLine(GoToLine::new(selected).with_value(value)))
        }
    }

    fn show_export_wizard(&mut self) {
        self.modal = Some(Modal::ExportWizard(ExportWizard::new(
            self.tstack.last().data_frame().clone().into(),
        )))
    }

    fn show_histogram(&mut self, col: &str, buckets: usize) -> AppResult<()> {
        self.modal = Some(Modal::HistogramPlot(HistogramPlot::new(
            self.tstack
                .last()
                .data_frame()
                .histogram_plot_data(col, buckets)?,
        )));
        Ok(())
    }

    fn show_histogram_wizard(&mut self) {
        self.modal = Some(Modal::HistogramWizard(HistogramWizard::new(
            histogram_wizard::State::new(self.tstack.last().data_frame()),
        )))
    }

    fn show_scatter_plot_wizard(&mut self) {
        self.modal = Some(Modal::ScatterPlotWizard(Wizard::new(
            scatter_plot_wizard::State::new(self.tstack.last().data_frame().clone()),
        )))
    }

    fn show_table_registerer(&mut self) {
        self.modal = Some(Modal::TableRegisterer(TableRegisterer::new(
            self.tstack.last().data_frame().clone(),
        )));
    }

    fn show_column_caster_wizard(&mut self) {
        self.modal = Some(Modal::ColumnCasterWizard(ColumnCastWizard::new(
            self.tstack.last().data_frame().clone().into(),
        )))
    }

    fn push_data_frame(&mut self, df: DataFrame, description: TableDescription) {
        self.tstack.push(table(df));
        self.dstack.push(description);
    }

    fn pop_data_frame(&mut self) {
        self.tstack.pop();
        self.dstack.pop();
    }

    fn select(&mut self, idx: usize) {
        self.tstack.last_mut().select(idx);
    }

    fn select_random(&mut self) {
        self.select(rand::rng().random_range(0..self.tstack.last().data_frame().height()));
    }

    fn cancel_modal(&mut self) {
        self.modal.take();
    }

    pub fn title(&self) -> &str {
        self.dstack.base().title()
    }
}

impl Component for Pane {
    fn render(
        &mut self,
        area: Rect,
        buf: &mut ratatui::prelude::Buffer,
        focus_state: super::component::FocusState,
    ) {
        self.tstack
            .last_mut()
            .set_gutter_visibility(config().show_table_row_numbers());
        match &mut self.modal {
            Some(Modal::Sheet(sheet_state)) => {
                if let Some(row) = self.tstack.last().selected()
                    && row != sheet_state.row()
                {
                    let sections = self.tstack.last().data_frame().get_sheet_sections(row);
                    sheet_state.set(row, sections);
                }
                self.tstack.last_mut().render(area, buf, focus_state);
                let area = area.inner(Margin::new(13, 3));
                sheet_state.render(area, buf, focus_state);
            }
            Some(Modal::SearchBar(search_bar_state)) => {
                let [search_area, table_area] =
                    Layout::vertical([Constraint::Length(3), Constraint::Fill(1)]).areas(area);
                self.tstack.last_mut().render(table_area, buf, focus_state);
                search_bar_state.render(search_area, buf, focus_state);
            }
            Some(Modal::GoToLine(state)) => {
                self.tstack.last_mut().render(area, buf, focus_state);
                state.render(area, buf, focus_state);
            }
            Some(Modal::DataFrameInfo(data_frame_info)) => {
                self.tstack
                    .last_mut()
                    .render(area, buf, FocusState::NotFocused);
                data_frame_info.render(area, buf, focus_state);
            }
            Some(Modal::ScatterPlot(state)) => {
                self.tstack
                    .last_mut()
                    .render(area, buf, FocusState::NotFocused);
                state.render(area, buf, focus_state);
            }
            Some(Modal::HistogramPlot(state)) => {
                self.tstack
                    .last_mut()
                    .render(area, buf, FocusState::NotFocused);
                state.render(area, buf, focus_state);
            }
            Some(Modal::QueryPicker(state)) => {
                self.tstack
                    .last_mut()
                    .render(area, buf, FocusState::NotFocused);
                state.render(area, buf, focus_state);
            }
            Some(Modal::ExportWizard(state)) => {
                self.tstack
                    .last_mut()
                    .render(area, buf, FocusState::NotFocused);
                state.render(area, buf, focus_state);
            }
            Some(Modal::HistogramWizard(state)) => {
                self.tstack
                    .last_mut()
                    .render(area, buf, FocusState::NotFocused);
                state.render(area, buf, focus_state);
            }
            Some(Modal::ScatterPlotWizard(state)) => {
                self.tstack
                    .last_mut()
                    .render(area, buf, FocusState::NotFocused);
                state.render(area, buf, focus_state);
            }
            Some(Modal::TableRegisterer(state)) => {
                self.tstack
                    .last_mut()
                    .render(area, buf, FocusState::NotFocused);
                state.render(area, buf, focus_state);
            }
            Some(Modal::ColumnCasterWizard(state)) => {
                self.tstack
                    .last_mut()
                    .render(area, buf, FocusState::NotFocused);
                state.render(area, buf, focus_state);
            }
            None => self.tstack.last_mut().render(area, buf, focus_state),
        }
    }

    fn handle(&mut self, event: crossterm::event::KeyEvent) -> bool {
        (match &mut self.modal {
            Some(Modal::GoToLine(go_to_line)) => {
                go_to_line.handle(event) || self.tstack.last_mut().handle(event)
            }
            Some(Modal::SearchBar(search_bar)) => {
                search_bar.handle(event) || self.tstack.last_mut().handle(event)
            }
            Some(Modal::Sheet(sheet)) => {
                sheet.handle(event) || self.tstack.last_mut().handle(event)
            }
            Some(Modal::DataFrameInfo(data_frame_info)) => data_frame_info.handle(event),
            Some(Modal::ExportWizard(export_wizard)) => export_wizard.handle(event),
            Some(Modal::HistogramPlot(histogram_plot)) => histogram_plot.handle(event),
            Some(Modal::HistogramWizard(histogram_wizard)) => histogram_wizard.handle(event),
            Some(Modal::QueryPicker(query_picker)) => query_picker.handle(event),
            Some(Modal::ScatterPlot(scatter_plot)) => scatter_plot.handle(event),
            Some(Modal::TableRegisterer(table_registerer)) => table_registerer.handle(event),
            Some(Modal::ScatterPlotWizard(scatter_plot_wizard)) => {
                scatter_plot_wizard.handle(event)
            }
            Some(Modal::ColumnCasterWizard(wizard)) => wizard.handle(event),

            None => self.tstack.last_mut().handle(event),
        }) || (match (event.code, event.modifiers) {
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
            (KeyCode::Char('R'), KeyModifiers::SHIFT)
                if !matches!(self.modal, Some(Modal::GoToLine(_))) =>
            {
                self.select_random();
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
        })
    }

    fn update(&mut self, action: &crate::handler::message::Message, focus_state: FocusState) {
        if let Some(modal) = self.modal.as_mut() {
            modal.responder().update(action, focus_state);
        }
        self.tstack.last_mut().update(action, focus_state);
        match action {
            Message::PaneShowInlineSelect if focus_state.is_focused() => {
                self.show_query_picker(QueryType::Select)
            }
            Message::PaneShowInlineFilter if focus_state.is_focused() => {
                self.show_query_picker(QueryType::Filter)
            }
            Message::PaneShowInlineOrder if focus_state.is_focused() => {
                self.show_query_picker(QueryType::Order)
            }
            Message::PaneShowSqlQuery if focus_state.is_focused() => {
                self.show_query_picker(QueryType::Sql)
            }
            Message::PaneShowExportWizard if focus_state.is_focused() => self.show_export_wizard(),
            Message::PaneShowScatterPlotWizard if focus_state.is_focused() => {
                self.show_scatter_plot_wizard()
            }
            Message::PaneShowHistogramWizard if focus_state.is_focused() => {
                self.show_histogram_wizard()
            }
            Message::PaneShowHistogram(col, buckets) if focus_state.is_focused() => {
                self.show_histogram(col, *buckets).unwrap_or_enqueue_error()
            }
            Message::PaneShowScatterPlot(x, y, grp) if focus_state.is_focused() => self
                .show_scatter_plot(x.to_owned(), y.to_owned(), grp.as_deref())
                .unwrap_or_enqueue_error(),
            Message::PaneShowTableRegisterer if focus_state.is_focused() => {
                self.show_table_registerer()
            }
            Message::PaneDismissModal if focus_state.is_focused() => self.cancel_modal(),
            Message::PanePushDataFrame(df, desc) if focus_state.is_focused() => {
                self.push_data_frame(df.clone(), desc.clone())
            }
            Message::PanePopDataFrame if focus_state.is_focused() => self.pop_data_frame(),
            Message::PaneTableSelect(idx) if focus_state.is_focused() => self.select(*idx),
            Message::PaneShowTableInfo if focus_state.is_focused() => self.show_data_frame_info(),
            Message::PaneShowColumnCasterWizard if focus_state.is_focused() => {
                self.show_column_caster_wizard()
            }
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
            Some(Modal::QueryPicker(_)) => (),
            Some(Modal::GoToLine(_)) => (),
            Some(Modal::ExportWizard(_)) => (),
            Some(Modal::HistogramWizard(_)) => (),
            Some(Modal::ScatterPlotWizard(_)) => (),
            Some(Modal::TableRegisterer(_)) => (),
            Some(Modal::ColumnCasterWizard(_)) => (),
            None => (),
        }
    }
}

#[derive(Debug)]
pub enum Modal {
    Sheet(Sheet),
    SearchBar(SearchBar),
    DataFrameInfo(DataFrameInfo),
    ScatterPlot(ScatterPlot),
    HistogramPlot(HistogramPlot),
    QueryPicker(QueryPicker),
    GoToLine(GoToLine),
    ExportWizard(ExportWizard),
    HistogramWizard(HistogramWizard),
    ScatterPlotWizard(ScatterPlotWizard),
    TableRegisterer(TableRegisterer),
    ColumnCasterWizard(ColumnCastWizard),
}

impl Modal {
    fn responder(&mut self) -> &mut dyn Component {
        match self {
            Modal::Sheet(sheet) => sheet,
            Modal::SearchBar(search_bar) => search_bar,
            Modal::DataFrameInfo(data_frame_info) => data_frame_info,
            Modal::ScatterPlot(scatter_plot_state) => scatter_plot_state,
            Modal::HistogramPlot(histogram_plot_state) => histogram_plot_state,
            Modal::QueryPicker(query_picker) => query_picker,
            Modal::GoToLine(go_to_line) => go_to_line,
            Modal::ExportWizard(export_wizard) => export_wizard,
            Modal::HistogramWizard(histogram_wizard) => histogram_wizard,
            Modal::ScatterPlotWizard(wizard) => wizard,
            Modal::TableRegisterer(table_registerer) => table_registerer,
            Modal::ColumnCasterWizard(wizard) => wizard,
        }
    }
}

#[derive(Debug, Clone)]
pub enum TableDescription {
    Table(String),
    Query(String),
    Filter(String),
    Order(String),
    Select(String),
    Cast(String),
}
impl TableDescription {
    pub(crate) fn title(&self) -> &str {
        match self {
            TableDescription::Table(desc)
            | TableDescription::Query(desc)
            | TableDescription::Filter(desc)
            | TableDescription::Order(desc)
            | TableDescription::Select(desc)
            | TableDescription::Cast(desc) => desc,
        }
    }
}

fn table(df: DataFrame) -> Table {
    Table::new(df)
        .striped()
        .with_selected(0)
        .with_show_header(true)
        .with_col_space(2)
        .with_extended_column()
}
