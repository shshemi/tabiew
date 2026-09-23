use std::sync::Arc;

use crate::misc::config::config;
use crate::misc::remote_load::{self, RemoteLoad};
use crate::tui::Pane;
use crate::tui::popups::download_notif::DownloadNotification;
use crate::tui::popups::sql_query_picker::SqlQueryPicker;
use crate::tui::table::Table;
use crate::tui::toast::Toast;
use crate::tui::{error_popup::ErrorPopup, tabs::Tabs};
use crate::{
    handler::message::Message,
    tui::{
        component::{Component, FocusState},
        popups::{
            about::About,
            command_palette::CommandPalette,
            fp_precision_picker::FpPrecisionPicker,
            importer::Importer,
            importers::{
                arrow::ArrowImporter, avro::AvroImporter, csv::CsvImporter, excel::ExcelImporter,
                fwf::FwfImporter, html::HtmlImporter, json::JsonImporter, jsonl::JsonlImporter,
                logfmt::LogfmtImporter, markdown::MarkdownImporter, parquet::ParquetImporter,
                sqlite::SqliteImporter, tsv::TsvImporter,
            },
            theme_selector::ThemeSelector,
        },
        schema::schema::Schema,
    },
};
use crossterm::event::{KeyCode, KeyModifiers};
use itertools::Itertools;
use ratatui::layout::{Constraint, Direction, Flex, Layout, Rect};
use url::Url;

pub struct App {
    tabs: Tabs,
    overlay: Option<Overlay>,
    schema: Option<Schema>,
    toast: Option<Toast>,
    dls: Vec<DownloadNotification>,
    running: bool,
}

impl App {
    pub fn new(tabs: Tabs) -> Self {
        Self {
            tabs,
            overlay: None,
            schema: None,
            toast: None,
            running: true,
            dls: Vec::new(),
        }
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
        self.overlay = Some(Overlay::SqlQueryPicker(SqlQueryPicker::new(
            self.tabs
                .selected()
                .map(Pane::table)
                .map(Table::data_frame)
                .cloned(),
        )));
    }

    fn add_download(&mut self, url: &Url, reader: Arc<dyn remote_load::Reader>) {
        self.dls.push(DownloadNotification::new(
            url.as_str().to_owned(),
            RemoteLoad::new(url.to_owned(), reader),
        ));
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

        let areas = Layout::new(
            Direction::Vertical,
            self.dls.iter().map(|_| Constraint::Length(3)),
        )
        .flex(Flex::End)
        .split(right_notif_bar(area));

        for (dl, area) in self.dls.iter_mut().zip(areas.iter()) {
            dl.render(*area, buf, FocusState::NotFocused);
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
            Message::AppDownloadDataSource(url, reader) => self.add_download(url, reader.clone()),
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
        self.dls
            .iter()
            .enumerate()
            .filter_map(|(idx, dl)| (!dl.is_running()).then_some(idx))
            .collect_vec()
            .into_iter()
            .rev()
            .for_each(|idx| {
                let dl = self.dls.remove(idx).into_remote_load();
                match dl.join() {
                    Ok(nfs) => {
                        for (name, df) in nfs {
                            Message::TabsAddNamePane(df, name).enqueue();
                        }
                    }
                    Err(err) => self.show_error(err.to_string()),
                }
            });
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

fn right_notif_bar(area: Rect) -> Rect {
    Rect {
        x: area.width.saturating_sub(if config().show_table_borders() {
            41
        } else {
            40
        }),
        y: 1,
        width: 40,
        height: area.height.saturating_sub(2),
    }
}
