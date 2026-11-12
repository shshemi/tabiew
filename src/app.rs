use std::{
    path::{Path, PathBuf},
    sync::Arc,
    time::{Duration, Instant},
};

use crate::AppResult;
use crate::ctl::{
    self,
    job::{Done, Job, Kind, Wait},
};
use crate::duckdb::{
    DataSource, DuckDB, Import, IngestOptions, IngestProgress, Load, Opened, Store,
};
use crate::indexer::Indexer;
use crate::misc::config::config;
use crate::misc::external_editor::run_editor;
use crate::misc::sql::{BackendSchema, TableInfo, sql};
use crate::reader::{Empty, Reader, RowSource};
use crate::tui::Pane;
use crate::tui::layouts::notification::NotificationLayout;
use crate::tui::pane::TableDescription;
use crate::tui::popups::column_caster::TargetType;
use crate::tui::popups::download_notif::{idle, render_progress};
use crate::tui::popups::pickers::inline_query_picker::QueryType;
use crate::tui::popups::pickers::sql_query_picker::{self, SqlQueryPicker};
use crate::tui::table::Table;
use crate::tui::toast::Toast;
use crate::tui::{error_popup::ErrorPopup, tabs::Tabs};
use crate::writers::{Destination, ExportFormat};
use crate::{
    handler::message::Message,
    tui::{
        component::{Component, FocusState},
        popups::{
            about::About,
            command_palette::CommandPalette,
            importer::Importer,
            importers::{
                arrow::ArrowImporter, avro::AvroImporter, csv::CsvImporter, excel::ExcelImporter,
                fwf::FwfImporter, html::HtmlImporter, json::JsonImporter, jsonl::JsonlImporter,
                logfmt::LogfmtImporter, markdown::MarkdownImporter, parquet::ParquetImporter,
                sqlite::SqliteImporter, tsv::TsvImporter,
            },
            pickers::fp_precision_picker::FpPrecisionPicker,
            theme_selector::ThemeSelector,
        },
        schema::schema::Schema,
    },
};
use crossterm::event::{KeyCode, KeyModifiers};
use ratatui::layout::{Constraint, Direction, Flex, Layout};

pub struct App {
    tabs: Tabs,
    overlay: Option<Overlay>,
    schema: Option<Schema>,
    toast: Option<Toast>,
    running: bool,
    duckdb: Option<DuckDB>,
    options: IngestOptions,
    ctl: Vec<Job>,
}

impl App {
    pub fn tabs(&self) -> &Tabs {
        &self.tabs
    }

    pub fn duckdb(&self) -> Option<&DuckDB> {
        self.duckdb.as_ref()
    }

    pub fn new(tabs: Tabs) -> Self {
        Self {
            tabs,
            overlay: None,
            schema: None,
            toast: None,
            running: true,
            duckdb: None,
            options: IngestOptions::default(),
            ctl: Vec::new(),
        }
    }

    pub fn with_duckdb(self, duckdb: DuckDB, options: IngestOptions) -> Self {
        Self {
            duckdb: Some(duckdb),
            options,
            ..self
        }
    }

    pub fn load(
        &mut self,
        source: &DataSource,
        options: &IngestOptions,
        max_rows: Option<usize>,
        select: bool,
    ) {
        let Some(duckdb) = self.duckdb.as_mut() else {
            return;
        };
        let loaded = match source {
            DataSource::Stdin => duckdb.load_stdin(None, options, max_rows),
            DataSource::File(path) => duckdb.load(path, None, options, max_rows),
            DataSource::Url(url) => duckdb.load_url(url, None, options, max_rows),
        };
        self.open(&label(source), loaded, options, select);
    }

    pub fn load_parts(
        &mut self,
        parts: &[DataSource],
        options: &IngestOptions,
        max_rows: Option<usize>,
    ) {
        let Some(duckdb) = self.duckdb.as_mut() else {
            return;
        };
        let loaded = duckdb.load_parts(parts, options, max_rows);
        let source = parts.first().map(label).unwrap_or_default();
        self.open(&source, loaded, options, false);
    }

    fn open(
        &mut self,
        source: &str,
        loaded: AppResult<Load>,
        options: &IngestOptions,
        select: bool,
    ) {
        let Some(duckdb) = self.duckdb.as_ref() else {
            return;
        };
        let mut panes = Vec::new();
        let mut errors = Vec::new();
        let ready = matches!(loaded, Ok(Load::Ready(_)));
        match loaded {
            Ok(Load::Ready(names)) => {
                for name in names {
                    match duckdb.reader(&name) {
                        Ok(reader) => {
                            panes.push(Pane::reader(reader, TableDescription::Table(name)))
                        }
                        Err(err) => errors.push(err.to_string()),
                    }
                }
            }
            Ok(Load::Ingesting(ingest)) => {
                let (name, title) = match source {
                    "-" => ("Stdin".to_string(), "Stdin".to_string()),
                    _ => {
                        let path = Path::new(source);
                        let part = |part: Option<&std::ffi::OsStr>| {
                            part.map(|part| part.to_string_lossy().into_owned())
                                .unwrap_or(source.to_string())
                        };
                        (part(path.file_stem()), part(path.file_name()))
                    }
                };
                let indexer = ingest
                    .file
                    .and_then(|file| Indexer::new(file, options, ingest.complete).ok());
                let description = TableDescription::Table(name);
                let config = duckdb.reader_config();
                let pane = match indexer {
                    Some(indexer) => Pane::reader(Reader::new(indexer, config), description),
                    None => Pane::reader(Reader::new(Empty, config), description),
                };
                panes.push(pane.loading_from(ingest.source, title, ingest.progress));
            }
            Err(err) => errors.push(format!("{source}: {err:#}")),
        }
        if let Some(error) = errors.pop() {
            self.show_error(error);
        }
        for pane in panes {
            match select {
                true => self.tabs.add(pane),
                false => self.tabs.push(pane),
            }
        }
        if ready {
            self.sync_schema();
        }
    }

    fn import(&mut self, source: &DataSource, import: &Import) {
        let options = import.options(self.options.clone());
        self.load(source, &options, None, true);
    }

    fn poll_duckdb(&mut self) {
        let Some(duckdb) = self.duckdb.as_mut() else {
            return;
        };
        let mut errors = Vec::new();
        let mut toasts = Vec::new();
        let mut trim = false;
        let mut changed = false;
        let mut ctl_failed = Vec::new();
        let mut ctl_exports = Vec::new();
        let mut ctl_tables = Vec::new();
        let (tables, queries) = match duckdb.poll() {
            Ok(finished) => {
                changed =
                    finished.stats || !finished.tables.is_empty() || !finished.queries.is_empty();
                for view in finished.results {
                    if let Some(pane) = self
                        .tabs
                        .panes_mut()
                        .find(|pane| pane.loading() == Some(Path::new(&view)))
                    {
                        match duckdb.reader(&view) {
                            Ok(reader) => {
                                toasts.extend(pane.done());
                                pane.loaded(reader)
                            }
                            Err(err) => {
                                pane.failed();
                                errors.push(err.to_string());
                            }
                        }
                    }
                }
                for view in finished.swapped {
                    for pane in self.tabs.panes_mut() {
                        pane.reset_view(&view);
                        pane.install_pending(&view);
                    }
                }
                if !finished.plots.is_empty() || !finished.exports.is_empty() {
                    trim = true;
                }
                for (view, plot) in finished.plots {
                    match plot.and_then(|data| {
                        self.tabs
                            .panes_mut()
                            .find(|pane| pane.table().view().as_deref() == Some(view.as_str()))
                            .map(|pane| pane.show_plot(data))
                            .unwrap_or(Ok(()))
                    }) {
                        Ok(()) => (),
                        Err(err) => errors.push(err.to_string()),
                    }
                }
                for (id, export) in finished.exports {
                    ctl_exports.push((id, export.clone()));
                    match export {
                        Ok(done) => toasts.push(done),
                        Err(error) => errors.push(error),
                    }
                }
                for (view, error) in &finished.failed {
                    ctl_failed.push((view.clone(), error.clone()));
                    for pane in self.tabs.panes_mut() {
                        pane.fail_pending(view);
                    }
                }
                for (view, error) in finished.failed {
                    if let Some(pane) = self
                        .tabs
                        .panes_mut()
                        .find(|pane| pane.loading() == Some(Path::new(&view)))
                    {
                        pane.failed();
                    }
                    errors.push(error);
                }
                (finished.tables, finished.queries)
            }
            Err(err) => {
                errors.push(err.to_string());
                (Vec::new(), Vec::new())
            }
        };
        for (source, result) in tables {
            ctl_tables.push((
                source.clone(),
                result
                    .as_ref()
                    .map(Clone::clone)
                    .map_err(|err| format!("{}: {err}", source.display())),
            ));
            let Some(pane) = self
                .tabs
                .panes_mut()
                .find(|pane| pane.loading() == Some(source.as_path()))
            else {
                continue;
            };
            let mut readers = Vec::new();
            match result {
                Ok(names) => {
                    for name in names {
                        match duckdb.reader(&name) {
                            Ok(reader) => readers.push((name, reader)),
                            Err(err) => errors.push(err.to_string()),
                        }
                    }
                }
                Err(err) => errors.push(format!("{}: {err}", source.display())),
            }
            let mut readers = readers.into_iter();
            match readers.next() {
                Some((_, reader)) => pane.loaded(reader),
                None => pane.stop_loading(),
            }
            for (name, reader) in readers {
                self.tabs
                    .push(Pane::reader(reader, TableDescription::Table(name)));
            }
        }
        for query in queries {
            match query {
                Ok((name, progress)) => {
                    let mut pane = Pane::reader(
                        Reader::new(Empty, duckdb.reader_config()),
                        TableDescription::Table(name.clone()),
                    );
                    pane.set_running(name.clone(), name, progress, None);
                    self.tabs.add(pane);
                }
                Err(err) => errors.push(err.to_string()),
            }
        }
        for toast in toasts {
            self.show_toast(toast);
        }
        if trim {
            crate::misc::workers::release_memory();
        }
        if changed {
            self.sync_schema();
            for pane in self.tabs.panes_mut() {
                if let Some(view) = pane.info_view()
                    && let Some(entry) = self.duckdb.as_mut().and_then(|duckdb| duckdb.entry(&view))
                {
                    pane.show_info(TableInfo::duckdb(&entry), view);
                }
            }
        }
        if let Some(error) = errors.pop() {
            self.show_error(error);
        }
        self.poll_ctl(ctl_failed, ctl_exports, ctl_tables);
    }

    fn poll_ctl(
        &mut self,
        failed: Vec<(String, String)>,
        exports: Vec<(usize, Result<String, String>)>,
        tables: Vec<(PathBuf, Result<Vec<String>, String>)>,
    ) {
        let Some(duckdb) = self.duckdb.as_ref() else {
            return;
        };
        let mut i = 0;
        while i < self.ctl.len() {
            let job = &mut self.ctl[i];
            let result = match &job.wait {
                Wait::View(view) => match failed.iter().find(|(failed, _)| failed == view) {
                    Some((_, error)) => Some(Err(error.clone())),
                    None if duckdb.result_progress(view).is_some() => None,
                    None => Some(
                        duckdb
                            .query_shape("SELECT * FROM _", Some(view))
                            .map(|(height, width)| Done::Shape(height, width))
                            .map_err(|_| "Cancelled in tw".to_owned()),
                    ),
                },
                Wait::Export(id) => exports
                    .iter()
                    .find(|(export, _)| export == id)
                    .map(|(_, result)| result.clone().map(|_| Done::Written)),
                Wait::Source(source) => tables
                    .iter()
                    .find(|(table, _)| table == source)
                    .map(|(_, result)| result.clone().map(Done::Names)),
            };
            match result {
                Some(result) => {
                    let job = self.ctl.swap_remove(i);
                    job.kind.reply(job.to, result).send();
                }
                None => {
                    if job.sent.elapsed() >= Duration::from_millis(90) {
                        ctl::Message::progress(
                            job.to,
                            job.progress
                                .as_ref()
                                .and_then(|progress| progress.fraction()),
                        )
                        .send();
                        job.sent = Instant::now();
                    }
                    i += 1;
                }
            }
        }
    }

    fn track(
        &mut self,
        to: u32,
        kind: Kind,
        started: AppResult<(Wait, Option<Arc<dyn IngestProgress>>)>,
    ) -> Option<ctl::Message> {
        match started {
            Ok((wait, progress)) => {
                self.ctl.push(Job {
                    to,
                    kind,
                    wait,
                    progress,
                    sent: Instant::now(),
                });
                None
            }
            Err(err) => Some(kind.reply(to, Err(format!("{err:#}")))),
        }
    }

    pub fn ctl_inline(
        &mut self,
        to: u32,
        query_type: QueryType,
        value: String,
    ) -> Option<ctl::Message> {
        let started = self.try_inline_query(query_type, &value);
        let kind = match query_type {
            QueryType::Select => Kind::Select(value),
            QueryType::Filter => Kind::Filter(value),
            QueryType::Order => Kind::Order(value),
        };
        let started = started.map(|view| self.view_job(view));
        self.track(to, kind, started)
    }

    pub fn ctl_query(&mut self, to: u32, sql: String) -> Option<ctl::Message> {
        let started = self.try_query(&sql).map(|view| self.view_job(view));
        self.track(to, Kind::Sql(sql), started)
    }

    fn view_job(&self, view: String) -> (Wait, Option<Arc<dyn IngestProgress>>) {
        let progress = self
            .duckdb
            .as_ref()
            .and_then(|duckdb| duckdb.result_progress(&view));
        (Wait::View(view), progress)
    }

    pub fn ctl_export(
        &mut self,
        to: u32,
        format: ExportFormat,
        path: PathBuf,
    ) -> Option<ctl::Message> {
        let view = self.tabs.selected().and_then(|pane| pane.table().view());
        let started = match (self.duckdb.as_mut(), view) {
            (Some(duckdb), Some(view)) => duckdb
                .export(&view, format, Destination::File(path.clone()))
                .map(|(id, progress)| (Wait::Export(id), Some(progress))),
            _ => Err(anyhow::anyhow!("not available until the table is ingested")),
        };
        self.track(to, Kind::Export(path), started)
    }

    pub fn ctl_import(
        &mut self,
        to: u32,
        source: DataSource,
        import: Import,
        max_rows: Option<usize>,
    ) -> Option<ctl::Message> {
        let duckdb = self.duckdb.as_mut()?;
        let options = import.options(self.options.clone());
        let loaded = match &source {
            DataSource::Stdin => duckdb.load_stdin(None, &options, max_rows),
            DataSource::File(path) => duckdb.load(path, None, &options, max_rows),
            DataSource::Url(url) => duckdb.load_url(url, None, &options, max_rows),
        };
        match loaded {
            Ok(Load::Ready(names)) => {
                self.sync_schema();
                Some(Kind::Import.reply(to, Ok(Done::Names(names))))
            }
            Ok(Load::Ingesting(ingest)) => self.track(
                to,
                Kind::Import,
                Ok((Wait::Source(ingest.source), Some(ingest.progress))),
            ),
            Err(err) => Some(Kind::Import.reply(to, Err(format!("{}: {err:#}", label(&source))))),
        }
    }

    pub fn ctl_cancel(&mut self, to: u32) -> Option<ctl::Message> {
        let job = self
            .ctl
            .swap_remove(self.ctl.iter().position(|job| job.to == to)?);
        match &job.wait {
            Wait::View(view) if !self.tabs.panes_mut().any(|pane| pane.cancel_view(view)) => {
                match self
                    .tabs
                    .iter()
                    .position(|pane| pane.loading() == Some(Path::new(view)))
                {
                    Some(idx) => Message::TabsRemove(idx).enqueue(),
                    None => Message::AppDropQuery(view.clone()).enqueue(),
                }
            }
            Wait::View(_) => (),
            Wait::Export(id) => {
                if let Some(duckdb) = self.duckdb.as_mut() {
                    duckdb.cancel_export(*id);
                }
            }
            Wait::Source(source) => {
                if let Some(duckdb) = self.duckdb.as_mut() {
                    duckdb.cancel_load(source);
                }
            }
        }
        Some(job.kind.reply(to, Err("Cancelled".to_owned())))
    }

    pub fn running(&self) -> bool {
        self.running
    }

    fn show_about(&mut self) {
        self.overlay = Some(Overlay::About(Default::default()));
    }

    fn show_theme_selector(&mut self) {
        self.overlay = Some(Overlay::ThemeSelector(Default::default()));
    }

    fn show_fp_precision_picker(&mut self) {
        self.overlay = Some(Overlay::FpPrecisionPicker(Default::default()));
    }

    fn show_palette(&mut self) {
        self.overlay = Some(Overlay::CommandPicker(CommandPalette::default()));
    }

    fn show_error(&mut self, message: impl Into<String>) {
        self.overlay = Some(Overlay::Error(ErrorPopup::new(message)));
    }

    fn show_toast(&mut self, message: impl Into<String>) {
        self.toast = Some(Toast::new(message));
    }

    fn show_importer(&mut self) {
        self.overlay = Some(Overlay::Import(Importer::default()))
    }

    fn show_arrow_importer(&mut self) {
        self.overlay = Some(Overlay::ImportArrow(ArrowImporter::default()))
    }

    fn show_avro_importer(&mut self) {
        self.overlay = Some(Overlay::ImportAvro(AvroImporter::default()))
    }

    fn show_csv_importer(&mut self) {
        self.overlay = Some(Overlay::ImportCsv(CsvImporter::default()))
    }

    fn show_excel_importer(&mut self) {
        self.overlay = Some(Overlay::ImportExcel(ExcelImporter::default()))
    }

    fn show_fwf_importer(&mut self) {
        self.overlay = Some(Overlay::ImportFwf(FwfImporter::default()))
    }

    fn show_html_importer(&mut self) {
        self.overlay = Some(Overlay::ImportHtml(HtmlImporter::default()))
    }

    fn show_json_importer(&mut self) {
        self.overlay = Some(Overlay::ImportJson(JsonImporter::default()))
    }

    fn show_jsonl_importer(&mut self) {
        self.overlay = Some(Overlay::ImportJsonl(JsonlImporter::default()))
    }

    fn show_logfmt_importer(&mut self) {
        self.overlay = Some(Overlay::ImportLogfmt(LogfmtImporter::default()))
    }

    fn show_markdown_importer(&mut self) {
        self.overlay = Some(Overlay::ImportMarkdown(MarkdownImporter::default()))
    }

    fn show_parquet_importer(&mut self) {
        self.overlay = Some(Overlay::ImportParquet(ParquetImporter::default()))
    }

    fn show_sqlite_importer(&mut self) {
        self.overlay = Some(Overlay::ImportSqlite(SqliteImporter::default()))
    }

    fn show_tsv_importer(&mut self) {
        self.overlay = Some(Overlay::ImportTsv(TsvImporter::default()))
    }

    fn show_sql_query_picker(&mut self) {
        self.sync_schema();
        let table = self.tabs.selected().map(Pane::table);
        self.overlay = Some(Overlay::SqlQueryPicker(SqlQueryPicker::duckdb(
            table
                .filter(|table| table.view().is_some())
                .and_then(Table::rows)
                .and_then(|rows| rows.lock().ok().map(|rows| rows.columns()))
                .map(|columns| columns.iter().map(|(name, _)| name.clone()).collect()),
        )));
    }

    fn sync_schema(&mut self) {
        if let Some(duckdb) = self.duckdb.as_mut() {
            let mut schema = BackendSchema::default();
            for entry in duckdb.tables() {
                schema.insert(entry.name.clone(), TableInfo::duckdb(&entry));
            }
            sql().set_schema(schema);
        }
    }

    fn open_table(&mut self, name: &str) {
        let Some(duckdb) = self.duckdb.as_mut() else {
            return;
        };
        match duckdb.counted_reader(name) {
            Ok(reader) => {
                self.schema = None;
                self.tabs.add(Pane::reader(
                    reader,
                    TableDescription::Table(name.to_owned()),
                ));
            }
            Err(err) => self.show_error(err.to_string()),
        }
    }

    fn drop_table(&mut self, name: &str) {
        let Some(duckdb) = self.duckdb.as_mut() else {
            return;
        };
        let detached = duckdb.detach(name);
        duckdb.cancel_jobs(name, true);
        duckdb.forget_stats(name);
        self.sync_schema();
        if let Err(err) = detached {
            self.show_error(err.to_string());
        }
    }

    fn search_start(&mut self, fuzzy: bool) {
        let idx = self.tabs.idx();
        let (Some(duckdb), Some(pane)) = (self.duckdb.as_mut(), self.tabs.panes_mut().nth(idx))
        else {
            return;
        };
        let Some(view) = pane.table().view() else {
            self.show_error("Not available until the table is ingested");
            return;
        };
        match duckdb.reader(&view) {
            Ok(reader) => pane.start_search(reader, view, fuzzy),
            Err(err) => self.show_error(err.to_string()),
        }
    }

    fn search(&mut self, base: &str, pattern: &str, fuzzy: bool) {
        let idx = self.tabs.idx();
        let (Some(duckdb), Some(pane)) = (self.duckdb.as_mut(), self.tabs.panes_mut().nth(idx))
        else {
            return;
        };
        let opened = match pattern.is_empty() {
            true => duckdb
                .reader(base)
                .map(|reader| Opened::Ready(Box::new(reader))),
            false => duckdb.open_search(base, pattern, fuzzy),
        };
        pane.cancel_pending();
        let mut opened = opened;
        let slow = first_page_slow(&mut opened);
        match opened {
            Ok(Opened::Ready(reader)) if slow => {
                let view = reader.source().view().unwrap_or_default().to_string();
                pane.set_pending(view, true, move |pane| pane.loaded(*reader));
            }
            Ok(Opened::Ready(reader)) => pane.loaded(*reader),
            Ok(Opened::Pending(view, progress)) => {
                pane.set_running(view, format!("Searching {pattern}"), progress, None)
            }
            Err(err) => self.show_error(err.to_string()),
        }
    }

    fn query(&mut self, sql: &str) {
        if let Err(err) = self.try_query(sql) {
            self.show_toast(err.to_string());
        }
    }

    fn try_query(&mut self, sql: &str) -> AppResult<String> {
        let duckdb = self
            .duckdb
            .as_mut()
            .ok_or_else(|| anyhow::anyhow!("DuckDB is not running"))?;
        let current = self.tabs.selected().and_then(|pane| pane.table().view());
        match duckdb.open_query(sql, current.as_deref(), Store::Wait)? {
            Opened::Pending(view, progress) => {
                self.overlay = None;
                sql_query_picker::remember(sql);
                let mut pane = Pane::reader(
                    Reader::new(Empty, duckdb.reader_config()),
                    TableDescription::Query(sql.to_owned()),
                );
                pane.set_running(view.clone(), sql.to_owned(), progress, None);
                self.tabs.add(pane);
                Ok(view)
            }
            Opened::Ready(reader) => Ok(reader.source().view().unwrap_or_default().to_string()),
        }
    }

    fn register(&mut self, name: &str) {
        let (Some(duckdb), Some(view)) = (
            self.duckdb.as_mut(),
            self.tabs.selected().and_then(|pane| pane.table().view()),
        ) else {
            return;
        };
        match duckdb.register(name, &view) {
            Ok(name) => {
                self.sync_schema();
                self.show_toast(format!("Data frame registered with name '{name}'"));
            }
            Err(err) => self.show_error(err.to_string()),
        }
    }

    fn edit_in_editor(&mut self, view: &str) {
        let idx = self.tabs.idx();
        let (Some(duckdb), Some(pane)) = (self.duckdb.as_mut(), self.tabs.panes_mut().nth(idx))
        else {
            return;
        };
        let path = duckdb.scratch_path(&format!("edit-{view}.csv"));
        let edited = duckdb
            .export_now(
                view,
                ExportFormat::Csv {
                    separator: ',',
                    quote: '"',
                    header: true,
                },
                &path,
            )
            .and_then(|()| run_editor(&path))
            .and_then(|()| {
                duckdb.open_query(
                    &format!(
                        "SELECT * FROM read_csv('{}')",
                        path.to_string_lossy().replace('\'', "''")
                    ),
                    None,
                    Store::Wait,
                )
            });
        match edited {
            Ok(Opened::Pending(result, progress)) => {
                pane.push_reader(
                    Reader::new(Empty, duckdb.reader_config()),
                    TableDescription::Table("Manual edit using $EDITOR".to_owned()),
                );
                pane.set_running(result, "$EDITOR changes".to_owned(), progress, None);
            }
            Ok(Opened::Ready(_)) => (),
            Err(err) => self.show_error(err.to_string()),
        }
    }

    fn show_info(&mut self, view: &str) {
        let idx = self.tabs.idx();
        let (Some(duckdb), Some(pane)) = (self.duckdb.as_mut(), self.tabs.panes_mut().nth(idx))
        else {
            return;
        };
        let Some(entry) = duckdb.entry(view) else {
            return;
        };
        if entry.stats.is_none() {
            let _ = duckdb.request_stats(view);
        }
        pane.show_info(TableInfo::duckdb(&entry), view.to_owned());
    }

    fn cast(&mut self, column: &str, target: TargetType) {
        let idx = self.tabs.idx();
        let (Some(duckdb), Some(pane)) = (self.duckdb.as_mut(), self.tabs.panes_mut().nth(idx))
        else {
            return;
        };
        let Some(current) = pane.table().view() else {
            return;
        };
        let quoted = format!("\"{}\"", column.replace('"', "\"\""));
        let sql = format!(
            "SELECT * REPLACE (CAST({quoted} AS {}) AS {quoted}) FROM _",
            target.duckdb()
        );
        match duckdb.open_query(&sql, Some(&current), Store::Wait) {
            Ok(Opened::Pending(view, progress)) => {
                pane.push_reader(
                    Reader::new(Empty, duckdb.reader_config()),
                    TableDescription::Cast(format!("'{column}' as {target}")),
                );
                pane.set_running(
                    view,
                    sql,
                    progress,
                    Some(format!("Column '{column}' were casted to '{target}'")),
                );
            }
            Ok(Opened::Ready(_)) => (),
            Err(err) => self.show_error(err.to_string()),
        }
    }

    fn inline_query(&mut self, query_type: QueryType, value: &str) {
        if let Err(err) = self.try_inline_query(query_type, value) {
            self.show_error(err.to_string());
        }
    }

    fn try_inline_query(&mut self, query_type: QueryType, value: &str) -> AppResult<String> {
        let idx = self.tabs.idx();
        let (Some(duckdb), Some(pane)) = (self.duckdb.as_mut(), self.tabs.panes_mut().nth(idx))
        else {
            return Err(anyhow::anyhow!("No tab is open"));
        };
        let Some(current) = pane.table().view() else {
            return Err(anyhow::anyhow!("Not available until the table is ingested"));
        };
        let (sql, description, toast) = match query_type {
            QueryType::Select => (
                format!("SELECT {value} FROM _"),
                TableDescription::Select(value.to_owned()),
                format!("Column selection '{value}' occurred"),
            ),
            QueryType::Filter => (
                format!("SELECT * FROM _ WHERE {value}"),
                TableDescription::Filter(value.to_owned()),
                format!("Filter '{value}' applied"),
            ),
            QueryType::Order => (
                format!("SELECT * FROM _ ORDER BY {value}"),
                TableDescription::Order(value.to_owned()),
                format!("Data frame ordered by '{value}'"),
            ),
        };
        let store = match query_type {
            QueryType::Select | QueryType::Filter => Store::Background,
            QueryType::Order => Store::Wait,
        };
        let mut opened = duckdb.open_query(&sql, Some(&current), store);
        let slow = first_page_slow(&mut opened);
        match opened? {
            Opened::Ready(reader) if slow => {
                let view = reader.source().view().unwrap_or_default().to_string();
                pane.set_pending(view.clone(), false, move |pane| {
                    pane.push_reader(*reader, description);
                    Message::AppShowToast(toast).enqueue();
                });
                Ok(view)
            }
            Opened::Ready(reader) => {
                let view = reader.source().view().unwrap_or_default().to_string();
                pane.push_reader(*reader, description);
                self.show_toast(toast);
                Ok(view)
            }
            Opened::Pending(view, progress) => {
                pane.push_reader(Reader::new(Empty, duckdb.reader_config()), description);
                pane.set_running(view.clone(), sql, progress, Some(toast));
                Ok(view)
            }
        }
    }

    fn reload_app_config(&mut self) {
        if let Err(err) = config().reload() {
            self.show_error(err.to_string());
        }
    }

    fn dismiss_overlay(&mut self) {
        self.overlay = None;
    }

    fn show_schema(&mut self) {
        self.sync_schema();
        self.schema = Some(Default::default());
    }

    fn dismiss_schema(&mut self) {
        self.schema = None;
    }

    fn quit(&mut self) {
        self.running = false;
    }
}

impl Component for App {
    fn render(
        &mut self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        _: crate::tui::component::FocusState,
    ) {
        match (self.overlay.as_mut(), self.schema.as_mut()) {
            (Some(overlay), Some(schema)) => {
                schema.render(area, buf, FocusState::NotFocused);
                overlay.responder().render(area, buf, FocusState::Focused);
            }
            (Some(overlay), None) => {
                self.tabs.render(area, buf, FocusState::NotFocused);
                overlay.responder().render(area, buf, FocusState::Focused);
            }
            (None, Some(schema)) => {
                schema.render(area, buf, FocusState::Focused);
            }
            (None, None) => {
                self.tabs.render(area, buf, FocusState::Focused);
            }
        }

        let ingests =
            self.tabs
                .iter()
                .enumerate()
                .filter(|(idx, pane)| *idx != self.tabs.idx() || pane.table().col_count() > 0)
                .filter_map(|(_, pane)| pane.progress())
                .map(|(title, percent)| (title.to_string(), percent))
                .chain(self.duckdb.iter().flat_map(DuckDB::materializing).map(
                    |(title, fraction)| {
                        (
                            title.to_string(),
                            fraction.map(|fraction| (fraction * 100.0) as u16),
                        )
                    },
                ))
                .collect::<Vec<_>>();
        let areas = Layout::new(
            Direction::Vertical,
            (0..ingests.len()).map(|_| Constraint::Length(3)),
        )
        .flex(Flex::End)
        .split(NotificationLayout::default().area(area));

        for ((title, percent), area) in ingests.iter().zip(areas.iter()) {
            render_progress(title, *percent, &idle(title), *area, buf);
        }

        if let Some(toast) = self.toast.as_mut() {
            toast.render(area, buf, FocusState::NotFocused);
        }
    }

    fn handle(&mut self, event: crossterm::event::KeyEvent) -> bool {
        if let Some(overlay) = self.overlay.as_mut() {
            overlay.responder().handle(event);
            true
        } else {
            (if let Some(schema) = self.schema.as_mut() {
                schema.handle(event)
            } else {
                self.tabs.handle(event)
            }) || match (event.modifiers, event.code) {
                (KeyModifiers::SHIFT, KeyCode::Char(':'))
                | (KeyModifiers::NONE, KeyCode::Char(':')) => {
                    self.show_palette();
                    true
                }
                (KeyModifiers::SHIFT, KeyCode::Char('Q'))
                | (KeyModifiers::NONE, KeyCode::Char('Q')) => {
                    self.quit();
                    true
                }
                _ => false,
            }
        }
    }

    fn update(&mut self, action: &Message) {
        match action {
            Message::Quit => self.quit(),
            Message::AppDismissOverlay => self.dismiss_overlay(),
            Message::AppShowError(message) => self.show_error(message),
            Message::AppShowToast(message) => self.show_toast(message),
            Message::AppShowAbout => self.show_about(),
            Message::AppShowCommandPicker => self.show_palette(),
            Message::AppShowThemeSelector => self.show_theme_selector(),
            Message::AppShowFpPrecisionPicker => self.show_fp_precision_picker(),
            Message::AppShowSchema => self.show_schema(),
            Message::AppShowImporter => self.show_importer(),
            Message::AppShowArrowImporter => self.show_arrow_importer(),
            Message::AppShowAvroImporter => self.show_avro_importer(),
            Message::AppShowCsvImporter => self.show_csv_importer(),
            Message::AppShowExcelImporter => self.show_excel_importer(),
            Message::AppShowFwfImporter => self.show_fwf_importer(),
            Message::AppShowHtmlImporter => self.show_html_importer(),
            Message::AppShowJsonImporter => self.show_json_importer(),
            Message::AppShowJsonlImporter => self.show_jsonl_importer(),
            Message::AppShowLogfmtImporter => self.show_logfmt_importer(),
            Message::AppShowMarkdownImporter => self.show_markdown_importer(),
            Message::AppShowParquetImporter => self.show_parquet_importer(),
            Message::AppShowSqliteImporter => self.show_sqlite_importer(),
            Message::AppShowTsvImporter => self.show_tsv_importer(),
            Message::AppDismissSchema => self.dismiss_schema(),
            Message::AppShowSqlQuery => self.show_sql_query_picker(),
            Message::AppReloadConfig => self.reload_app_config(),
            Message::AppImport(source, import) => self.import(source, import),
            Message::AppQuery(sql) => self.query(sql),
            Message::AppSearchStart(fuzzy) => self.search_start(*fuzzy),
            Message::AppCast(column, target) => self.cast(column, *target),
            Message::AppRegister(name) => self.register(name),
            Message::AppEditInEditor(view) => self.edit_in_editor(view),
            Message::AppPlot(view, request) => {
                if let Some(duckdb) = self.duckdb.as_mut()
                    && let Err(err) = duckdb.plot(view, request.clone())
                {
                    self.show_error(err.to_string());
                }
            }
            Message::AppShowInfo(view) => self.show_info(view),
            Message::AppExport(view, format, dest) => {
                if let Some(duckdb) = self.duckdb.as_mut()
                    && let Err(err) = duckdb.export(view, *format, dest.clone())
                {
                    self.show_error(err.to_string());
                }
            }
            Message::AppSearch(base, pattern, fuzzy) => self.search(base, pattern, *fuzzy),
            Message::AppInlineQuery(query_type, value) => self.inline_query(*query_type, value),
            Message::AppOpenTable(name) => self.open_table(name),
            Message::AppDropTable(name) => self.drop_table(name),
            Message::AppTableStats(name) => {
                if let Some(duckdb) = self.duckdb.as_mut() {
                    let _ = duckdb.request_stats(name);
                }
            }
            Message::AppDropQuery(view) => {
                let loading = self
                    .tabs
                    .iter()
                    .any(|pane| pane.loading() == Some(Path::new(view)));
                let shown = self
                    .tabs
                    .iter()
                    .any(|pane| pane.views().any(|v| v == *view));
                if let Some(duckdb) = self.duckdb.as_mut() {
                    duckdb.drop_result(view);
                    if !loading {
                        duckdb.cancel_load(Path::new(view));
                    }
                    if !shown {
                        duckdb.cancel_jobs(view, false);
                    }
                }
            }
            _ => (),
        };
        if let Some(overlay) = self.overlay.as_mut() {
            overlay.responder().update(action);
        }
        if let Some(schema) = self.schema.as_mut() {
            schema.update(action);
        }
        self.tabs.update(action);
    }

    fn tick(&mut self) {
        if let Some(overlay) = self.overlay.as_mut() {
            overlay.responder().tick();
        }
        if let Some(toast) = self.toast.as_mut()
            && toast.is_finished()
        {
            self.toast.take();
        }
        self.poll_duckdb();
        self.tabs.tick();
    }
}

#[derive(Debug)]
pub enum Overlay {
    About(About),
    Error(ErrorPopup),
    CommandPicker(CommandPalette),
    ThemeSelector(ThemeSelector),
    FpPrecisionPicker(FpPrecisionPicker),
    SqlQueryPicker(SqlQueryPicker),
    Import(Importer),
    ImportArrow(ArrowImporter),
    ImportAvro(AvroImporter),
    ImportCsv(CsvImporter),
    ImportExcel(ExcelImporter),
    ImportFwf(FwfImporter),
    ImportHtml(HtmlImporter),
    ImportJson(JsonImporter),
    ImportJsonl(JsonlImporter),
    ImportLogfmt(LogfmtImporter),
    ImportMarkdown(MarkdownImporter),
    ImportParquet(ParquetImporter),
    ImportSqlite(SqliteImporter),
    ImportTsv(TsvImporter),
}

impl Overlay {
    fn responder(&mut self) -> &mut dyn Component {
        match self {
            Overlay::About(about) => about,
            Overlay::Error(error) => error,
            Overlay::CommandPicker(command_palette) => command_palette,
            Overlay::ThemeSelector(theme_selector) => theme_selector,
            Overlay::FpPrecisionPicker(fp_precision_picker) => fp_precision_picker,
            Overlay::Import(step_by_step) => step_by_step,
            Overlay::SqlQueryPicker(sql_query_picker) => sql_query_picker,
            Overlay::ImportArrow(importer) => importer,
            Overlay::ImportAvro(importer) => importer,
            Overlay::ImportCsv(importer) => importer,
            Overlay::ImportExcel(importer) => importer,
            Overlay::ImportFwf(importer) => importer,
            Overlay::ImportHtml(importer) => importer,
            Overlay::ImportJson(importer) => importer,
            Overlay::ImportJsonl(importer) => importer,
            Overlay::ImportLogfmt(importer) => importer,
            Overlay::ImportMarkdown(importer) => importer,
            Overlay::ImportParquet(importer) => importer,
            Overlay::ImportSqlite(importer) => importer,
            Overlay::ImportTsv(importer) => importer,
        }
    }
}

fn label(source: &DataSource) -> String {
    match source {
        DataSource::Stdin => "-".to_string(),
        DataSource::File(path) => path.to_string_lossy().into_owned(),
        DataSource::Url(url) => url.to_string(),
    }
}

fn first_page_slow(opened: &mut AppResult<Opened>) -> bool {
    let Ok(Opened::Ready(reader)) = opened else {
        return false;
    };
    let _ = reader.fill(0..1);
    reader.source().deferred()
}
