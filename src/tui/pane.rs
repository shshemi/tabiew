use crossterm::event::{KeyCode, KeyModifiers};
use itertools::{FoldWhile, Itertools};
use polars::frame::DataFrame;
use rand::RngExt;
use ratatui::{
    layout::{Constraint, Layout, Rect},
    widgets::{Block, Borders, Widget},
};
use unicode_width::UnicodeWidthStr;

use super::{search_bar::SearchBar, sheet::Sheet};
use crate::{
    AppResult,
    handler::message::Message,
    misc::{
        config::config,
        external_editor::edit_in_external_editor,
        non_empty_stack::NonEmptyStack,
        polars_ext::DataFrameExt,
        sql::{TableSource, sql},
        type_ext::UnwrapOrEnqueueError,
    },
    tui::{
        app_default::AppDefault,
        component::{Component, FocusState},
        icons,
        plots::{histogram_plot::HistogramPlot, scatter_plot::ScatterPlot},
        popups::{
            column_caster::ColumnCaster,
            data_frame_info::DataFrameInfo,
            exporter::Exporter,
            go_to_line::GoToLine,
            histogram_builder::{self, HistogramBuilder},
            inline_query_picker::{InlineQueryPicker, QueryType},
            multi_step_overlay::MultiStepOverlay,
            scatter_plot_builder::{self, ScatterPlotBuilder},
            table_registerer::TableRegisterer,
        },
        search_bar::Searcher,
        table::Table,
        widgets::status_bar::StatusBar,
    },
};

#[derive(Debug)]
pub struct Pane {
    tstack: NonEmptyStack<Table>,
    dstack: NonEmptyStack<TableDescription>,
    sheet: Option<Sheet>,
    modal: Option<Modal>,
}

impl Pane {
    /// Constructs a new instance of [`App`].
    pub fn new(data_frame: DataFrame, description: TableDescription) -> Self {
        Self {
            tstack: NonEmptyStack::new(
                Table::new(data_frame)
                    .striped()
                    .with_selected(0)
                    .with_show_header(true)
                    .with_col_space(2)
                    .with_extended_column(),
            ),
            dstack: NonEmptyStack::new(description),
            sheet: None,
            modal: None,
        }
    }

    pub fn table(&self) -> &Table {
        self.tstack.last()
    }

    pub fn description(&self) -> &TableDescription {
        self.dstack.last()
    }

    pub fn description_mut(&mut self) -> &mut TableDescription {
        self.dstack.last_mut()
    }

    pub fn iter_descriptions(&self) -> impl DoubleEndedIterator<Item = &TableDescription> {
        self.dstack.iter()
    }

    pub fn show_sheet(&mut self) {
        if self.sheet.is_none()
            && let Some(row) = self.tstack.last().selected()
        {
            let values = self.tstack.last().data_frame().get_sheet_values(row);
            self.sheet = Some(Sheet::new(row, values));
        }
    }

    fn dismiss_sheet(&mut self) {
        self.sheet.take();
    }

    fn sync_sheet(&mut self) {
        if let Some(sheet) = self.sheet.as_mut()
            && let Some(row) = self.tstack.last().selected()
            && row != sheet.row()
        {
            let sections = self.tstack.last().data_frame().get_sheet_values(row);
            sheet.set(row, sections);
        }
    }

    fn force_sync_sheet(&mut self) {
        if let Some(sheet) = self.sheet.as_mut()
            && let Some(row) = self.tstack.last().selected()
        {
            let sections = self.tstack.last().data_frame().get_sheet_values(row);
            sheet.set(row, sections);
        }
    }

    fn show_fuzzy_search(&mut self) {
        let tbl = self.tstack.last().to_owned();
        self.tstack.push(tbl);
        self.dstack
            .push(TableDescription::Search(Default::default()));
        self.modal = Some(Modal::SearchBar(SearchBar::fuzzy(
            self.tstack.last().data_frame().clone(),
        )));
    }

    fn show_exact_search(&mut self) {
        let tbl = self.tstack.last().to_owned();
        self.tstack.push(tbl);
        self.dstack
            .push(TableDescription::Search(Default::default()));
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
            | TableDescription::Cast(_)
            | TableDescription::Search(_)
            | TableDescription::FuzzySearch(_) => {
                self.modal = Some(Modal::DataFrameInfo(DataFrameInfo::new(
                    self.tstack.last().data_frame(),
                    TableSource::User,
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

    fn show_inline_query_picker(&mut self, query_type: QueryType) {
        self.modal = Some(Modal::InlineQueryPicker(InlineQueryPicker::new(
            self.tstack.last().data_frame().clone(),
            query_type,
        )));
    }

    fn show_go_to_line_with_value(&mut self, value: usize) {
        if let Some(selected) = self.tstack.last().selected() {
            self.modal = Some(Modal::GoToLine(GoToLine::new(selected).with_value(value)));
            self.select(value.saturating_sub(1));
        }
    }

    fn show_exporter(&mut self) {
        self.modal = Some(Modal::Exporter(Exporter::new(
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

    fn show_histogram_builder(&mut self) {
        self.modal = Some(Modal::HistogramBuilder(HistogramBuilder::new(
            histogram_builder::State::new(self.tstack.last().data_frame()),
        )))
    }

    fn show_scatter_plot_builder(&mut self) {
        self.modal = Some(Modal::ScatterPlotBuilder(MultiStepOverlay::new(
            scatter_plot_builder::State::new(self.tstack.last().data_frame().clone()),
        )))
    }

    fn show_table_registerer(&mut self) {
        self.modal = Some(Modal::TableRegisterer(TableRegisterer::new(
            self.tstack.last().data_frame().clone(),
        )));
    }

    fn show_column_caster(&mut self) {
        self.modal = Some(Modal::ColumnCaster(ColumnCaster::new(
            self.tstack.last().data_frame().clone().into(),
        )))
    }

    fn push_data_frame(&mut self, df: DataFrame, description: TableDescription) {
        self.tstack
            .push(self.tstack.last().clone_with_data_frame(df));
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
        let height = self.tstack.last().data_frame().height();
        if height > 0 {
            self.select(rand::rng().random_range(0..height));
        }
    }

    fn cancel_modal(&mut self) {
        self.modal.take();
    }

    pub fn title(&self) -> &str {
        self.dstack.base().description()
    }

    pub fn history(&self, mut width: usize) -> String {
        self.dstack
            .iter()
            .rev()
            .fold_while(String::new(), |mut s, td| {
                if s.is_empty() {
                    let tag = format!("{} {}", td.variant(), td.description());
                    width = width.saturating_sub(tag.width());
                    s.extend(tag.chars().rev());
                    FoldWhile::Continue(s)
                } else {
                    //
                    let tag = format!("{} {} > ", td.variant(), td.description());
                    let w = tag.width();
                    if w <= width {
                        width -= w;
                        s.extend(tag.chars().rev());
                        FoldWhile::Continue(s)
                    } else {
                        FoldWhile::Done(s)
                    }
                }
            })
            .into_inner()
            .chars()
            .rev()
            .collect()
    }
}

impl Component for Pane {
    fn render(
        &mut self,
        area: Rect,
        buf: &mut ratatui::prelude::Buffer,
        focus_state: super::component::FocusState,
    ) {
        let bordered = config().show_table_borders();
        let [mut table_area, status_bar_area, sheet_area] =
            table_status_bar_areas(area, bordered, self.sheet.is_some());

        // settings
        self.sync_sheet();
        self.tstack
            .last_mut()
            .set_gutter_visibility(config().show_table_row_numbers());

        // render table borders
        if bordered {
            let block = Block::app_default().borders(Borders::all());
            let inner = block.inner(table_area);
            block.render(table_area, buf);
            table_area = inner;
        }

        // render sheet
        if let Some(sheet) = self.sheet.as_mut() {
            sheet.render(sheet_area, buf, focus_state);
        }

        // render status bar
        StatusBar::new(self).render(status_bar_area, buf);

        match &mut self.modal {
            Some(Modal::SearchBar(search_bar_state)) => {
                let [search_area, table_area] =
                    Layout::vertical([Constraint::Length(3), Constraint::Fill(1)])
                        .areas(table_area);
                self.tstack.last_mut().render(table_area, buf, focus_state);
                search_bar_state.render(search_area, buf, focus_state);
            }
            Some(Modal::GoToLine(go_to_line)) => {
                self.tstack.last_mut().render(table_area, buf, focus_state);
                go_to_line.render(table_area, buf, focus_state);
            }
            Some(Modal::DataFrameInfo(data_frame_info)) => {
                self.tstack
                    .last_mut()
                    .render(table_area, buf, FocusState::NotFocused);
                data_frame_info.render(table_area, buf, focus_state);
            }
            Some(Modal::ScatterPlot(scatter_plot)) => {
                self.tstack
                    .last_mut()
                    .render(table_area, buf, FocusState::NotFocused);
                scatter_plot.render(table_area, buf, focus_state);
            }
            Some(Modal::HistogramPlot(histogram_plot)) => {
                self.tstack
                    .last_mut()
                    .render(table_area, buf, FocusState::NotFocused);
                histogram_plot.render(table_area, buf, focus_state);
            }
            Some(Modal::InlineQueryPicker(inline_query_picker)) => {
                self.tstack
                    .last_mut()
                    .render(table_area, buf, FocusState::NotFocused);
                inline_query_picker.render(table_area, buf, focus_state);
            }
            Some(Modal::Exporter(exporter)) => {
                self.tstack
                    .last_mut()
                    .render(table_area, buf, FocusState::NotFocused);
                exporter.render(table_area, buf, focus_state);
            }
            Some(Modal::HistogramBuilder(histogram_builder)) => {
                self.tstack
                    .last_mut()
                    .render(table_area, buf, FocusState::NotFocused);
                histogram_builder.render(table_area, buf, focus_state);
            }
            Some(Modal::ScatterPlotBuilder(scatter_plot_builder)) => {
                self.tstack
                    .last_mut()
                    .render(table_area, buf, FocusState::NotFocused);
                scatter_plot_builder.render(table_area, buf, focus_state);
            }
            Some(Modal::TableRegisterer(table_registerer)) => {
                self.tstack
                    .last_mut()
                    .render(table_area, buf, FocusState::NotFocused);
                table_registerer.render(table_area, buf, focus_state);
            }
            Some(Modal::ColumnCaster(column_caster)) => {
                self.tstack
                    .last_mut()
                    .render(table_area, buf, FocusState::NotFocused);
                column_caster.render(table_area, buf, focus_state);
            }
            None => self.tstack.last_mut().render(table_area, buf, focus_state),
        }
    }

    fn handle(&mut self, event: crossterm::event::KeyEvent) -> bool {
        if let Some(model) = &mut self.modal {
            match model {
                Modal::SearchBar(search_bar) => {
                    search_bar.handle(event) || self.tstack.last_mut().handle(event)
                }
                Modal::GoToLine(go_to_line) => go_to_line.handle(event),
                Modal::DataFrameInfo(data_frame_info) => data_frame_info.handle(event),
                Modal::Exporter(exporter) => exporter.handle(event),
                Modal::HistogramPlot(histogram_plot) => histogram_plot.handle(event),
                Modal::HistogramBuilder(histogram_builder) => histogram_builder.handle(event),
                Modal::InlineQueryPicker(query_picker) => query_picker.handle(event),
                Modal::ScatterPlot(scatter_plot) => scatter_plot.handle(event),
                Modal::TableRegisterer(table_registerer) => table_registerer.handle(event),
                Modal::ScatterPlotBuilder(scatter_plot_builder) => {
                    scatter_plot_builder.handle(event)
                }
                Modal::ColumnCaster(column_caster) => column_caster.handle(event),
            };
            true
        } else {
            self.sheet
                .as_mut()
                .map(|sheet| sheet.handle(event))
                .unwrap_or_default()
                || self.tstack.last_mut().handle(event)
                || (match (event.code, event.modifiers) {
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
                    (KeyCode::Char('i'), KeyModifiers::NONE) => {
                        self.show_data_frame_info();
                        true
                    }
                    (KeyCode::Char('/'), KeyModifiers::NONE) => {
                        self.show_fuzzy_search();
                        true
                    }
                    (KeyCode::Char('R'), KeyModifiers::SHIFT) => {
                        self.select_random();
                        true
                    }
                    (KeyCode::Char('?'), KeyModifiers::NONE)
                    | (KeyCode::Char('?'), KeyModifiers::SHIFT) => {
                        self.show_exact_search();
                        true
                    }
                    (KeyCode::Char('q'), KeyModifiers::NONE)
                        if self.tstack.len_without_base() > 0 =>
                    {
                        self.pop_data_frame();
                        true
                    }
                    _ => false,
                })
        }
    }

    fn update(&mut self, action: &crate::handler::message::Message) {
        if let Some(modal) = self.modal.as_mut() {
            modal.responder().update(action);
        }
        if let Some(sheet) = self.sheet.as_mut() {
            sheet.update(action);
        }
        self.tstack.last_mut().update(action);
        match action {
            Message::PaneShowInlineSelect => self.show_inline_query_picker(QueryType::Select),
            Message::PaneShowInlineFilter => self.show_inline_query_picker(QueryType::Filter),
            Message::PaneShowInlineOrder => self.show_inline_query_picker(QueryType::Order),
            Message::PaneShowExporter => self.show_exporter(),
            Message::PaneShowScatterPlotBuilder => self.show_scatter_plot_builder(),
            Message::PaneShowHistogramBuilder => self.show_histogram_builder(),
            Message::PaneShowHistogram(col, buckets) => {
                self.show_histogram(col, *buckets).unwrap_or_enqueue_error();
            }
            Message::PaneShowScatterPlot(x, y, grp) => {
                self.show_scatter_plot(x.to_owned(), y.to_owned(), grp.as_deref())
                    .unwrap_or_enqueue_error();
            }
            Message::PaneShowTableRegisterer => self.show_table_registerer(),
            Message::PaneDismissModal => self.cancel_modal(),
            Message::PaneDismissSheet => self.dismiss_sheet(),
            Message::PanePushDataFrame(df, desc) => self.push_data_frame(df.clone(), desc.clone()),
            Message::PanePopDataFrame => self.pop_data_frame(),
            Message::PaneTableSelect(idx) => self.select(*idx),
            Message::PaneShowTableInfo => self.show_data_frame_info(),
            Message::PaneShowColumnCaster => self.show_column_caster(),
            Message::PaneShowSearch => {
                self.show_exact_search();
            }
            Message::PaneShowFuzzySearch => {
                self.show_fuzzy_search();
            }
            Message::PaneEditInExternalEditor => {
                match edit_in_external_editor(self.tstack.last().data_frame().clone()) {
                    Ok(df) => self.push_data_frame(
                        df,
                        TableDescription::Table("Manual edit using $EDITOR".to_owned()),
                    ),
                    Err(err) => Message::AppShowError(err.to_string()).enqueue(),
                }
            }
            _ => (),
        }
    }

    fn tick(&mut self) {
        match &mut self.modal {
            Some(Modal::SearchBar(search_bar)) => {
                if let Some(df) = search_bar.searcher().latest() {
                    self.tstack.last_mut().set_data_frame(df);
                    *self.description_mut() = match search_bar.searcher() {
                        Searcher::Fuzzy(_) => {
                            TableDescription::FuzzySearch(search_bar.value().to_owned())
                        }
                        Searcher::Exact(_) => {
                            TableDescription::Search(search_bar.value().to_owned())
                        }
                    };
                    self.force_sync_sheet();
                }
            }
            Some(Modal::DataFrameInfo(_)) => (),
            Some(Modal::ScatterPlot(_)) => (),
            Some(Modal::HistogramPlot(_)) => (),
            Some(Modal::InlineQueryPicker(_)) => (),
            Some(Modal::GoToLine(_)) => (),
            Some(Modal::Exporter(_)) => (),
            Some(Modal::HistogramBuilder(_)) => (),
            Some(Modal::ScatterPlotBuilder(_)) => (),
            Some(Modal::TableRegisterer(_)) => (),
            Some(Modal::ColumnCaster(_)) => (),
            None => (),
        }
    }
}

fn table_status_bar_areas(area: Rect, bordered: bool, sheet: bool) -> [Rect; 3] {
    let [table_area, sheet_area] = if sheet {
        Layout::horizontal([Constraint::Percentage(70), Constraint::Min(48)]).areas(area)
    } else {
        [area, area]
    };
    let [table_area, status_bar_area] = if bordered {
        [
            table_area,
            Rect {
                x: table_area.x + 1,
                y: table_area.y + table_area.height.saturating_sub(1),
                width: table_area.width.saturating_sub(2),
                height: 1,
            },
        ]
    } else {
        Layout::vertical([Constraint::Fill(1), Constraint::Length(1)]).areas(table_area)
    };
    [table_area, status_bar_area, sheet_area]
}

#[derive(Debug)]
pub enum Modal {
    SearchBar(SearchBar),
    DataFrameInfo(DataFrameInfo),
    ScatterPlot(ScatterPlot),
    HistogramPlot(HistogramPlot),
    InlineQueryPicker(InlineQueryPicker),
    GoToLine(GoToLine),
    Exporter(Exporter),
    HistogramBuilder(HistogramBuilder),
    ScatterPlotBuilder(ScatterPlotBuilder),
    TableRegisterer(TableRegisterer),
    ColumnCaster(ColumnCaster),
}

impl Modal {
    fn responder(&mut self) -> &mut dyn Component {
        match self {
            Modal::SearchBar(search_bar) => search_bar,
            Modal::DataFrameInfo(data_frame_info) => data_frame_info,
            Modal::ScatterPlot(scatter_plot_state) => scatter_plot_state,
            Modal::HistogramPlot(histogram_plot_state) => histogram_plot_state,
            Modal::InlineQueryPicker(query_picker) => query_picker,
            Modal::GoToLine(go_to_line) => go_to_line,
            Modal::Exporter(exporter) => exporter,
            Modal::HistogramBuilder(histogram_builder) => histogram_builder,
            Modal::ScatterPlotBuilder(scatter_plot_builder) => scatter_plot_builder,
            Modal::TableRegisterer(table_registerer) => table_registerer,
            Modal::ColumnCaster(column_caster) => column_caster,
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
    Search(String),
    FuzzySearch(String),
}
impl TableDescription {
    pub fn variant(&self) -> &'static str {
        match self {
            TableDescription::Table(_) => "Table",
            TableDescription::Query(_) => "Query",
            TableDescription::Filter(_) => "Filter",
            TableDescription::Order(_) => "Order",
            TableDescription::Select(_) => "Select",
            TableDescription::Cast(_) => "Cast",
            TableDescription::Search(_) => "Search",
            TableDescription::FuzzySearch(_) => "Fuzzy Search",
        }
    }
    pub fn icon(&self) -> icons::Icon {
        match self {
            TableDescription::Table(_) => icons::TABLE,
            TableDescription::Query(_) => icons::DATABASE,
            TableDescription::Filter(_) => icons::FILTER,
            TableDescription::Order(_) => icons::SORT,
            TableDescription::Select(_) => icons::COLUMN,
            TableDescription::Cast(_) => icons::CAST,
            TableDescription::Search(_) => icons::SEARCH,
            TableDescription::FuzzySearch(_) => icons::FUZZY_SEARCH,
        }
    }

    pub fn description(&self) -> &str {
        match self {
            TableDescription::Table(desc)
            | TableDescription::Query(desc)
            | TableDescription::Filter(desc)
            | TableDescription::Order(desc)
            | TableDescription::Select(desc)
            | TableDescription::Cast(desc)
            | TableDescription::Search(desc)
            | TableDescription::FuzzySearch(desc) => desc,
        }
    }
}
