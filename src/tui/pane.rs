use crossterm::event::{KeyCode, KeyModifiers};
use polars::frame::DataFrame;
use rand::RngExt;
use ratatui::{
    layout::{Constraint, Flex, Layout, Rect},
    widgets::{Block, Borders, Widget},
};
use std::{
    path::{Path, PathBuf},
    sync::Arc,
    time::{Duration, Instant},
};

use super::{
    search_bar::SearchBar,
    sheet::{Data as SheetData, Sheet},
};
use crate::{
    AppResult,
    collections::non_empty_stack::NonEmptyStack,
    duckdb::IngestProgress,
    handler::message::{Message, UnwrapOrEnqueueError},
    misc::{
        config::config,
        external_editor::edit_in_external_editor,
        sql::{TableSource, sql},
    },
    reader::Rows,
    tui::{
        app_default::AppDefault,
        component::{Component, FocusState},
        icons,
        layouts::status_bar::StatusBarLayout,
        plots::{
            histogram_plot::{self, HistogramPlot},
            scatter_plot::{self, ScatterPlot},
        },
        popups::{
            column_caster::{ColumnCaster, Source},
            data_frame_info::DataFrameInfo,
            download_notif::{idle, render_progress},
            exporter::Exporter,
            exporters::{
                arrow::ArrowExporter, avro::AvroExporter, csv::CsvExporter, json::JsonExporter,
                jsonl::JsonlExporter, markdown::MarkdownExporter, parquet::ParquetExporter,
                tsv::TsvExporter,
            },
            go_to_line::GoToLine,
            histogram_builder::{self, HistogramBuilder},
            pickers::inline_query_picker::{InlineQueryPicker, QueryType},
            scatter_plot_builder::{self, ScatterPlotBuilder},
            table_registerer::TableRegisterer,
            wizard::Wizard,
        },
        search_bar::SearchType,
        table::Table,
        widgets::status_bar::StatusBar,
    },
    writers::ExportData,
};

#[derive(Debug)]
pub struct Pane {
    tstack: NonEmptyStack<Table>,
    dstack: NonEmptyStack<TableDescription>,
    sheet: Option<Sheet>,
    modal: Option<Modal>,
    loading: Option<Loading>,
    search: Option<Search>,
    pending: Option<Pending>,
}

struct Pending {
    view: String,
    replaces: bool,
    install: Box<dyn FnOnce(&mut Pane)>,
}

impl std::fmt::Debug for Pending {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.view)
    }
}

#[derive(Debug)]
struct Search {
    base: String,
    fuzzy: bool,
    sent: String,
    typed: String,
    since: Instant,
}

struct Loading {
    source: PathBuf,
    title: String,
    verb: &'static str,
    progress: Arc<dyn IngestProgress>,
    done: Option<String>,
}

impl std::fmt::Debug for Loading {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.title)
    }
}

impl Pane {
    /// Constructs a new instance of [`App`].
    pub fn new(data_frame: DataFrame, description: TableDescription) -> Self {
        Self::with_table(Table::new(data_frame), description)
    }

    pub fn reader(rows: impl Rows + 'static, description: TableDescription) -> Self {
        Self::with_table(Table::reader(rows), description)
    }

    fn with_table(table: Table, description: TableDescription) -> Self {
        Self {
            tstack: NonEmptyStack::new(
                table
                    .striped()
                    .with_selected(0)
                    .with_show_header(true)
                    .with_col_space(2)
                    .with_extended_column(),
            ),
            dstack: NonEmptyStack::new(description),
            sheet: None,
            modal: None,
            loading: None,
            search: None,
            pending: None,
        }
    }

    pub fn loading_from(
        self,
        source: PathBuf,
        title: String,
        progress: Arc<dyn IngestProgress>,
    ) -> Self {
        Self {
            loading: Some(Loading {
                source,
                title,
                verb: "Ingesting",
                progress,
                done: None,
            }),
            ..self
        }
    }

    pub fn loading(&self) -> Option<&Path> {
        self.loading
            .as_ref()
            .map(|loading| loading.source.as_path())
    }

    pub fn progress(&self) -> Option<(String, Option<u16>)> {
        self.loading.as_ref().map(|loading| {
            (
                format!("{} {}", loading.verb, loading.title),
                loading
                    .progress
                    .fraction()
                    .map(|fraction| (fraction * 100.0) as u16),
            )
        })
    }

    pub fn start_search(&mut self, rows: impl Rows + 'static, base: String, fuzzy: bool) {
        self.push_reader(rows, TableDescription::Search(String::new()));
        self.modal = Some(Modal::SearchBar(SearchBar::duckdb(match fuzzy {
            true => SearchType::Fuzzy,
            false => SearchType::Exact,
        })));
        self.search = Some(Search {
            base,
            fuzzy,
            sent: String::new(),
            typed: String::new(),
            since: Instant::now(),
        });
    }

    fn send_search(&mut self, wait: Duration) {
        let Some(Modal::SearchBar(search_bar)) = &self.modal else {
            return;
        };
        let Some(search) = self.search.as_mut() else {
            return;
        };
        if search.typed != search_bar.value() {
            search.typed = search_bar.value().to_owned();
            search.since = Instant::now();
        }
        if search.sent != search.typed && search.since.elapsed() >= wait {
            search.sent = search.typed.clone();
            Message::AppSearch(search.base.clone(), search.sent.clone(), search.fuzzy).enqueue();
            let description = match search.fuzzy {
                true => TableDescription::FuzzySearch(search.sent.clone()),
                false => TableDescription::Search(search.sent.clone()),
            };
            *self.description_mut() = description;
        }
    }

    pub fn loaded(&mut self, rows: impl Rows + 'static) {
        let below = self.tstack.iter().rev().nth(1).and_then(Table::view);
        if let Some(old) = self.table().view()
            && Some(&old) != below.as_ref()
            && Some(&old) != rows.view().as_ref()
        {
            Message::AppDropQuery(old).enqueue();
        }
        self.loading = None;
        self.tstack.last_mut().set_reader(rows);
        self.sync_sheet();
    }

    pub fn reset_view(&mut self, view: &str) {
        for table in self.tstack.iter() {
            if table.view().as_deref() == Some(view)
                && let Some(rows) = table.rows()
                && let Ok(mut rows) = rows.lock()
            {
                rows.reset();
            }
        }
        self.sync_sheet();
    }

    pub fn set_pending(
        &mut self,
        view: String,
        replaces: bool,
        install: impl FnOnce(&mut Pane) + 'static,
    ) {
        self.cancel_pending();
        self.pending = Some(Pending {
            view,
            replaces,
            install: Box::new(install),
        });
    }

    pub fn cancel_pending(&mut self) -> Option<bool> {
        let pending = self.pending.take()?;
        Message::AppDropQuery(pending.view).enqueue();
        Some(pending.replaces)
    }

    pub fn install_pending(&mut self, view: &str) {
        if self
            .pending
            .as_ref()
            .is_some_and(|pending| pending.view == view)
            && let Some(pending) = self.pending.take()
        {
            (pending.install)(self);
        }
    }

    pub fn fail_pending(&mut self, view: &str) {
        if self
            .pending
            .as_ref()
            .is_some_and(|pending| pending.view == view)
        {
            self.cancel_pending();
        }
    }

    pub fn stop_loading(&mut self) {
        self.loading = None;
    }

    pub fn set_running(
        &mut self,
        view: String,
        title: String,
        progress: Arc<dyn IngestProgress>,
        done: Option<String>,
    ) {
        self.loading = Some(Loading {
            title,
            source: PathBuf::from(view),
            verb: "Running",
            progress,
            done,
        });
    }

    pub fn done(&mut self) -> Option<String> {
        self.loading
            .as_mut()
            .and_then(|loading| loading.done.take())
    }

    pub fn failed(&mut self) {
        self.loading = None;
        if self.tstack.len_without_base() > 0 {
            self.tstack.pop();
            self.dstack.pop();
            self.sync_sheet();
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

    pub fn iter_tables(&self) -> impl DoubleEndedIterator<Item = &Table> {
        self.tstack.iter()
    }

    fn sheet_data(&self) -> Option<SheetData> {
        match (self.table().data_frame(), self.table().rows()) {
            (Some(df), _) => Some(SheetData::Frame(df.clone())),
            (None, Some(rows)) => Some(SheetData::Reader(rows)),
            (None, None) => None,
        }
    }

    fn frame(&self) -> Option<DataFrame> {
        let df = self.tstack.last().data_frame().cloned();
        if df.is_none() {
            Message::AppShowError("Not available on DuckDB tables yet".to_string()).enqueue();
        }
        df
    }

    fn export_data(&self) -> Option<ExportData> {
        match (self.table().data_frame(), self.table().view()) {
            (Some(df), _) => Some(df.clone().into()),
            (None, Some(view)) => Some(ExportData::View(view)),
            (None, None) => {
                Message::AppShowError("Not available until the table is ingested".to_string())
                    .enqueue();
                None
            }
        }
    }

    pub fn show_sheet(&mut self) {
        if self.sheet.is_none()
            && let Some(data) = self.sheet_data()
        {
            self.sheet = Some(Sheet::new(data, self.table().selected()));
        }
    }

    fn dismiss_sheet(&mut self) {
        self.sheet.take();
    }

    fn sync_sheet(&mut self) {
        let data = self.sheet_data();
        if let Some(sheet) = self.sheet.as_mut()
            && let Some(data) = data
        {
            sheet.set_data(data);
            sheet.set_row(self.tstack.last().selected());
        }
    }

    fn show_fuzzy_search(&mut self) {
        if self.table().rows().is_some() {
            Message::AppSearchStart(true).enqueue();
            return;
        }
        let Some(df) = self.frame() else {
            return;
        };
        let tbl = self.tstack.last().to_owned();
        self.tstack.push(tbl);
        self.dstack
            .push(TableDescription::Search(Default::default()));
        self.modal = Some(Modal::SearchBar(SearchBar::fuzzy(df)));
    }

    fn show_exact_search(&mut self) {
        if self.table().rows().is_some() {
            Message::AppSearchStart(false).enqueue();
            return;
        }
        let Some(df) = self.frame() else {
            return;
        };
        let tbl = self.tstack.last().to_owned();
        self.tstack.push(tbl);
        self.dstack
            .push(TableDescription::Search(Default::default()));
        self.modal = Some(Modal::SearchBar(SearchBar::exact(df)));
    }

    fn show_data_frame_info(&mut self) {
        let Some(df) = self.frame() else {
            return;
        };
        match &self.dstack.last() {
            TableDescription::Table(desc) => {
                if let Some(input) = sql().schema().get(desc).map(|info| info.source()).cloned() {
                    self.modal = Some(Modal::DataFrameInfo(DataFrameInfo::new(&df, input)))
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
                    &df,
                    TableSource::User,
                )))
            }
        }
    }

    fn show_scatter_plot(
        &mut self,
        x_label: &str,
        y_label: &str,
        group_by: Option<&str>,
    ) -> AppResult<()> {
        let Some(df) = self.frame() else {
            return Ok(());
        };
        let df = &df;
        let plot = if let Some(group_by) = group_by {
            scatter_plot::ScatterPlotBuilder::new(df, x_label, y_label)
                .with_group(group_by)
                .build()?
        } else {
            scatter_plot::ScatterPlotBuilder::new(df, x_label, y_label).build()?
        };
        self.modal = Some(Modal::ScatterPlot(plot));
        Ok(())
    }

    fn show_inline_query_picker(&mut self, query_type: QueryType) {
        let picker = match (self.table().data_frame(), self.table().rows()) {
            (Some(df), _) => InlineQueryPicker::new(df.clone(), query_type),
            (None, Some(rows)) => InlineQueryPicker::duckdb(
                rows.lock()
                    .map(|rows| {
                        rows.columns()
                            .iter()
                            .map(|(name, _)| name.clone())
                            .collect()
                    })
                    .unwrap_or_default(),
                query_type,
            ),
            (None, None) => return,
        };
        self.modal = Some(Modal::InlineQueryPicker(picker));
    }

    fn show_go_to_line_with_value(&mut self, value: usize) {
        if let Some(selected) = self.tstack.last().selected() {
            self.modal = Some(Modal::GoToLine(GoToLine::new(selected).with_value(value)));
            self.select(value.saturating_sub(1));
        }
    }

    fn show_exporter(&mut self) {
        self.modal = Some(Modal::Exporter(Exporter::default()))
    }

    fn show_arrow_exporter(&mut self) {
        let Some(df) = self.export_data() else {
            return;
        };
        self.modal = Some(Modal::ExportArrow(ArrowExporter::new(df.into())))
    }

    fn show_avro_exporter(&mut self) {
        let Some(df) = self.export_data() else {
            return;
        };
        self.modal = Some(Modal::ExportAvro(AvroExporter::new(df.into())))
    }

    fn show_csv_exporter(&mut self) {
        let Some(df) = self.export_data() else {
            return;
        };
        self.modal = Some(Modal::ExportCsv(CsvExporter::new(df.into())))
    }

    fn show_json_exporter(&mut self) {
        let Some(df) = self.export_data() else {
            return;
        };
        self.modal = Some(Modal::ExportJson(JsonExporter::new(df.into())))
    }

    fn show_jsonl_exporter(&mut self) {
        let Some(df) = self.export_data() else {
            return;
        };
        self.modal = Some(Modal::ExportJsonl(JsonlExporter::new(df.into())))
    }

    fn show_markdown_exporter(&mut self) {
        let Some(df) = self.export_data() else {
            return;
        };
        self.modal = Some(Modal::ExportMarkdown(MarkdownExporter::new(df.into())))
    }

    fn show_parquet_exporter(&mut self) {
        let Some(df) = self.export_data() else {
            return;
        };
        self.modal = Some(Modal::ExportParquet(ParquetExporter::new(df.into())))
    }

    fn show_tsv_exporter(&mut self) {
        let Some(df) = self.export_data() else {
            return;
        };
        self.modal = Some(Modal::ExportTsv(TsvExporter::new(df.into())))
    }

    fn show_histogram(&mut self, col: &str, buckets: usize) -> AppResult<()> {
        let Some(df) = self.frame() else {
            return Ok(());
        };
        let df = &df;
        self.modal = Some(Modal::HistogramPlot(
            histogram_plot::HistogramPlotBuilder::new(df, col, buckets).build()?,
        ));
        Ok(())
    }

    fn show_histogram_builder(&mut self) {
        let Some(df) = self.frame() else {
            return;
        };
        self.modal = Some(Modal::HistogramBuilder(HistogramBuilder::new(
            histogram_builder::State::new(&df),
        )))
    }

    fn show_scatter_plot_builder(&mut self) {
        let Some(df) = self.frame() else {
            return;
        };
        self.modal = Some(Modal::ScatterPlotBuilder(Wizard::new(
            scatter_plot_builder::State::new(df),
        )))
    }

    fn show_table_registerer(&mut self) {
        let Some(df) = self.frame() else {
            return;
        };
        self.modal = Some(Modal::TableRegisterer(TableRegisterer::new(df)));
    }

    fn show_column_caster(&mut self) {
        let source = match (self.table().data_frame(), self.table().rows()) {
            (Some(df), _) => Source::Frame(df.clone()),
            (None, Some(rows)) if self.table().view().is_some() => Source::View(
                rows.lock()
                    .map(|rows| rows.columns().to_vec())
                    .unwrap_or_default(),
            ),
            _ => {
                Message::AppShowError("Not available until the table is ingested".to_string())
                    .enqueue();
                return;
            }
        };
        self.modal = Some(Modal::ColumnCaster(ColumnCaster::new(source.into())))
    }

    fn push_data_frame(&mut self, df: DataFrame, description: TableDescription) {
        self.tstack
            .push(self.tstack.last().clone_with_data_frame(df));
        self.dstack.push(description);
    }

    pub fn push_reader(&mut self, rows: impl Rows + 'static, description: TableDescription) {
        self.tstack.push(self.tstack.last().clone_with_reader(rows));
        self.dstack.push(description);
        self.sync_sheet();
    }

    pub fn views(&self) -> impl Iterator<Item = String> {
        self.tstack.iter().filter_map(Table::view).chain(
            self.loading
                .as_ref()
                .map(|loading| loading.source.to_string_lossy().into_owned()),
        )
    }

    fn pop_data_frame(&mut self) {
        if self.cancel_pending() == Some(false) {
            return;
        }
        let stacked = self.tstack.len_without_base() > 0;
        let below = self.tstack.iter().rev().nth(1).and_then(Table::view);
        if let Some(view) = self.tstack.pop().and_then(|table| table.view())
            && Some(&view) != below.as_ref()
        {
            Message::AppDropQuery(view).enqueue();
        }
        if stacked {
            self.search = None;
        }
        if stacked
            && self
                .loading
                .as_ref()
                .is_some_and(|loading| loading.verb == "Running")
            && let Some(loading) = self.loading.take()
        {
            Message::AppDropQuery(loading.source.to_string_lossy().into_owned()).enqueue();
        }
        self.dstack.pop();
        self.sync_sheet();
    }

    fn select(&mut self, idx: usize) {
        self.tstack.last_mut().select(idx);
        self.sync_sheet();
    }

    fn select_random(&mut self) {
        let height = self.tstack.last().row_count();
        if height > 0 {
            self.select(rand::rng().random_range(0..height));
        }
    }

    fn dismiss_modal(&mut self) {
        self.send_search(Duration::ZERO);
        self.search = None;
        self.modal.take();
    }

    pub fn title(&self) -> &str {
        self.dstack.base().description()
    }

    fn render_table(
        &mut self,
        buf: &mut ratatui::prelude::Buffer,
        area: Rect,
        focus_status: FocusState,
    ) {
        // render table borders
        let block = Block::app_default().borders(Borders::all());
        let table_area = block.inner(area);
        let status_bar_area = StatusBarLayout::new().area(area);
        let focus_state = if matches!(
            self.modal,
            Some(Modal::SearchBar(_)) | Some(Modal::GoToLine(_)) | None
        ) {
            focus_status
        } else {
            FocusState::NotFocused
        };
        block.render(area, buf);
        match self.progress() {
            Some((title, percent)) if self.table().col_count() == 0 => {
                let [gauge_area] = Layout::vertical([Constraint::Length(3)])
                    .flex(Flex::Center)
                    .areas(table_area);
                let [gauge_area] = Layout::horizontal([Constraint::Max(60)])
                    .flex(Flex::Center)
                    .horizontal_margin(2)
                    .areas(gauge_area);
                render_progress(&title, percent, &idle(&title), gauge_area, buf);
            }
            _ => self.tstack.last_mut().render(table_area, buf, focus_state),
        }
        StatusBar::new(self).render(status_bar_area, buf);
    }

    fn render_sheet(&mut self, buf: &mut ratatui::prelude::Buffer, area: Rect) {
        if let Some(sheet) = self.sheet.as_mut() {
            sheet.render(area, buf, FocusState::Focused);
        }
    }
}

impl Component for Pane {
    fn render(
        &mut self,
        area: Rect,
        buf: &mut ratatui::prelude::Buffer,
        focus_state: super::component::FocusState,
    ) {
        // settings
        self.sync_sheet();
        self.tstack
            .last_mut()
            .set_gutter_visibility(config().show_table_row_numbers());

        let areas = Areas::new(self, area);

        // render table
        self.render_table(buf, areas.table, focus_state);

        // render sheet
        self.render_sheet(buf, areas.sheet);

        // render modal
        match &mut self.modal {
            Some(Modal::SearchBar(search_bar_state)) => {
                search_bar_state.render(areas.search_bar, buf, focus_state);
            }
            Some(modal) => {
                modal.responder().render(areas.table, buf, focus_state);
            }
            None => (),
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
                Modal::ExportArrow(exporter) => exporter.handle(event),
                Modal::ExportAvro(exporter) => exporter.handle(event),
                Modal::ExportCsv(exporter) => exporter.handle(event),
                Modal::ExportJson(exporter) => exporter.handle(event),
                Modal::ExportJsonl(exporter) => exporter.handle(event),
                Modal::ExportMarkdown(exporter) => exporter.handle(event),
                Modal::ExportParquet(exporter) => exporter.handle(event),
                Modal::ExportTsv(exporter) => exporter.handle(event),
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
                        if self.tstack.len_without_base() > 0 || self.pending.is_some() =>
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
            Message::PaneShowArrowExporter => self.show_arrow_exporter(),
            Message::PaneShowAvroExporter => self.show_avro_exporter(),
            Message::PaneShowCsvExporter => self.show_csv_exporter(),
            Message::PaneShowJsonExporter => self.show_json_exporter(),
            Message::PaneShowJsonlExporter => self.show_jsonl_exporter(),
            Message::PaneShowMarkdownExporter => self.show_markdown_exporter(),
            Message::PaneShowParquetExporter => self.show_parquet_exporter(),
            Message::PaneShowTsvExporter => self.show_tsv_exporter(),
            Message::PaneShowScatterPlotBuilder => self.show_scatter_plot_builder(),
            Message::PaneShowHistogramBuilder => self.show_histogram_builder(),
            Message::PaneShowHistogram(col, buckets) => {
                self.show_histogram(col, *buckets).unwrap_or_enqueue_error();
            }
            Message::PaneShowScatterPlot(x, y, grp) => {
                self.show_scatter_plot(x, y, grp.as_deref())
                    .unwrap_or_enqueue_error();
            }
            Message::PaneShowTableRegisterer => self.show_table_registerer(),
            Message::PaneDismissModal => self.dismiss_modal(),
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
                let Some(df) = self.frame() else {
                    return;
                };
                match edit_in_external_editor(df) {
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
        self.send_search(Duration::from_millis(200));
        match &mut self.modal {
            Some(Modal::SearchBar(search_bar)) => {
                if let Some(df) = search_bar.searcher().and_then(|searcher| searcher.latest()) {
                    self.tstack.last_mut().set_data_frame(df);
                    *self.description_mut() = match search_bar.search_type() {
                        SearchType::Fuzzy => {
                            TableDescription::FuzzySearch(search_bar.value().to_owned())
                        }
                        SearchType::Exact => {
                            TableDescription::Search(search_bar.value().to_owned())
                        }
                    };
                    self.sync_sheet();
                }
            }
            Some(Modal::DataFrameInfo(_)) => (),
            Some(Modal::ScatterPlot(_)) => (),
            Some(Modal::HistogramPlot(_)) => (),
            Some(Modal::InlineQueryPicker(_)) => (),
            Some(Modal::GoToLine(_)) => (),
            Some(Modal::Exporter(_)) => (),
            Some(Modal::ExportArrow(_)) => (),
            Some(Modal::ExportAvro(_)) => (),
            Some(Modal::ExportCsv(_)) => (),
            Some(Modal::ExportJson(_)) => (),
            Some(Modal::ExportJsonl(_)) => (),
            Some(Modal::ExportMarkdown(_)) => (),
            Some(Modal::ExportParquet(_)) => (),
            Some(Modal::ExportTsv(_)) => (),
            Some(Modal::HistogramBuilder(_)) => (),
            Some(Modal::ScatterPlotBuilder(_)) => (),
            Some(Modal::TableRegisterer(_)) => (),
            Some(Modal::ColumnCaster(_)) => (),
            None => (),
        }
    }
}

pub struct Areas {
    table: Rect,
    sheet: Rect,
    search_bar: Rect,
}

impl Areas {
    fn new(pane: &Pane, area: Rect) -> Self {
        let [left, sheet] = if pane.sheet.is_some() {
            Layout::horizontal([Constraint::Percentage(70), Constraint::Min(58)]).areas(area)
        } else {
            [area, Default::default()]
        };

        let [search_bar, table] = if matches!(pane.modal, Some(Modal::SearchBar(_))) {
            Layout::vertical([Constraint::Length(3), Constraint::Fill(1)]).areas(left)
        } else {
            [Default::default(), left]
        };

        Areas {
            table,
            sheet,
            search_bar,
        }
    }
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
    ExportArrow(ArrowExporter),
    ExportAvro(AvroExporter),
    ExportCsv(CsvExporter),
    ExportJson(JsonExporter),
    ExportJsonl(JsonlExporter),
    ExportMarkdown(MarkdownExporter),
    ExportParquet(ParquetExporter),
    ExportTsv(TsvExporter),
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
            Modal::ExportArrow(exporter) => exporter,
            Modal::ExportAvro(exporter) => exporter,
            Modal::ExportCsv(exporter) => exporter,
            Modal::ExportJson(exporter) => exporter,
            Modal::ExportJsonl(exporter) => exporter,
            Modal::ExportMarkdown(exporter) => exporter,
            Modal::ExportParquet(exporter) => exporter,
            Modal::ExportTsv(exporter) => exporter,
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
