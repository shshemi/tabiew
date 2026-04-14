use std::sync::mpsc::{Receiver, TryRecvError};
use std::time::Duration;

use polars::frame::DataFrame;

use crate::misc::sql::{Source as SqlSource, sql};
use crate::misc::upsert_index::UpsertIndex;
use crate::reader::StreamEvent;
use crate::tui::Pane;
use crate::tui::pane::StreamStatus;
use crate::tui::popups::sql_query_picker::SqlQueryPicker;
use crate::tui::table::{FlashKind, Table};
use ratatui::style::Color;
use crate::tui::toast::Toast;
use crate::tui::{error_popup::ErrorPopup, tabs::Tabs};
use crate::{
    handler::message::Message,
    tui::{
        component::{Component, FocusState},
        popups::{
            command_palette::CommandPalette, help_modal::Help, importer::Importer,
            theme_selector::ThemeSelector,
        },
        schema::schema::Schema,
    },
};
use crossterm::event::KeyCode;

/// State for a single streaming tab: receives `StreamEvent`s from a background
/// reader thread and knows which pane index they feed.
pub struct StreamSink {
    pub rx: Receiver<StreamEvent>,
    pub tab_index: usize,
    pub table_name: String,
    pub open: bool,
    pub rows_received: u64,
    pub rows_inserted: u64,
    pub rows_updated: u64,
    pub upsert: Option<UpsertIndex>,
    pub flash_duration: Duration,
    pub flash_update_color: Color,
}

impl StreamSink {
    pub fn new(
        rx: Receiver<StreamEvent>,
        tab_index: usize,
        table_name: String,
        upsert: Option<UpsertIndex>,
        flash_duration: Duration,
        flash_update_color: Color,
    ) -> Self {
        Self {
            rx,
            tab_index,
            table_name,
            open: true,
            rows_received: 0,
            rows_inserted: 0,
            rows_updated: 0,
            upsert,
            flash_duration,
            flash_update_color,
        }
    }
}

pub struct App {
    tabs: Tabs,
    overlay: Option<Overlay>,
    schema: Option<Schema>,
    toast: Option<Toast>,
    stream: Option<StreamSink>,
    running: bool,
}

impl App {
    pub fn new(tabs: Tabs) -> Self {
        Self {
            tabs,
            overlay: None,
            schema: None,
            toast: None,
            stream: None,
            running: true,
        }
    }

    pub fn with_stream(mut self, stream: StreamSink) -> Self {
        if let Some(pane) = self.tabs.pane_mut(stream.tab_index) {
            pane.set_stream_status(StreamStatus {
                open: stream.open,
                rows_received: stream.rows_received,
                rows_inserted: stream.rows_inserted,
                rows_updated: stream.rows_updated,
            });
        }
        self.stream = Some(stream);
        self
    }

    /// Drain any pending events from the stream channel and apply them to the
    /// live DataFrame backing the streaming pane. Called from `tick()`.
    fn drain_stream(&mut self) {
        let Some(stream) = self.stream.as_mut() else {
            return;
        };
        if !stream.open {
            return;
        }
        loop {
            match stream.rx.try_recv() {
                Ok(StreamEvent::Schema { schema, .. }) => {
                    if let Some(pane) = self.tabs.pane_mut(stream.tab_index) {
                        let current = pane.base_table_mut().data_frame();
                        if current.width() == 0 {
                            // Initial schema: replace the placeholder with an
                            // empty frame that has the right columns so the
                            // header row renders immediately.
                            let df = DataFrame::empty_with_schema(&schema);
                            sql().refresh_frame(&stream.table_name, df.clone(), SqlSource::Stdin);
                            pane.base_table_mut().set_data_frame(df);
                            pane.base_table_mut().set_flash_duration(stream.flash_duration);
                            pane.base_table_mut().set_flash_update_color(stream.flash_update_color);
                        }
                        Self::publish_stream_status(pane, stream);
                    }
                }
                Ok(StreamEvent::Batch { rows, .. }) => {
                    if let Some(pane) = self.tabs.pane_mut(stream.tab_index) {
                        let new_rows = rows.height() as u64;
                        let batch_result = if let Some(ref mut upsert) = stream.upsert {
                            // Upsert mode: deduplicate by key columns.
                            let df = pane.base_table_mut().data_frame_mut();
                            upsert.apply_batch(df, rows)
                                .map(|stats| (df.clone(), stats))
                        } else {
                            // Append-only mode: just vstack.
                            let df = pane.base_table_mut().data_frame_mut();
                            let res = if df.width() == 0 {
                                *df = rows;
                                Ok(())
                            } else {
                                df.vstack_mut_owned(rows).map(|_| ()).map_err(Into::into)
                            };
                            use crate::misc::upsert_index::UpsertStats;
                            res.map(|()| (df.clone(), UpsertStats {
                                inserted: new_rows as usize,
                                updated: 0,
                                inserted_rows: Vec::new(),
                                updated_cells: Vec::new(),
                            }))
                        };
                        match batch_result {
                            Ok((refreshed, stats)) => {
                                stream.rows_received += new_rows;
                                stream.rows_inserted += stats.inserted as u64;
                                stream.rows_updated += stats.updated as u64;

                                // Flash changed cells (only in upsert mode).
                                if stream.upsert.is_some() {
                                    let width = refreshed.width();
                                    if !stats.inserted_rows.is_empty() {
                                        let cells = stats.inserted_rows.iter()
                                            .flat_map(|&row| (0..width).map(move |col| (row, col)));
                                        pane.base_table_mut().flash_cells(FlashKind::Insert, cells);
                                    }
                                    if !stats.updated_cells.is_empty() {
                                        pane.base_table_mut().flash_cells(
                                            FlashKind::Update,
                                            stats.updated_cells.into_iter(),
                                        );
                                    }
                                }

                                pane.base_table_mut().refresh_layout();
                                sql().refresh_frame(
                                    &stream.table_name,
                                    refreshed,
                                    SqlSource::Stdin,
                                );
                                Self::publish_stream_status(pane, stream);
                            }
                            Err(err) => {
                                Message::AppShowError(format!("stream batch failed: {err}"))
                                    .enqueue();
                                stream.open = false;
                                Self::publish_stream_status(pane, stream);
                                return;
                            }
                        }
                    }
                }
                Ok(StreamEvent::Eof { .. }) => {
                    stream.open = false;
                    if let Some(pane) = self.tabs.pane_mut(stream.tab_index) {
                        Self::publish_stream_status(pane, stream);
                    }
                    Message::AppShowToast(format!("stream closed: {} rows", stream.rows_received))
                        .enqueue();
                    return;
                }
                Ok(StreamEvent::Error { error, .. }) => {
                    stream.open = false;
                    if let Some(pane) = self.tabs.pane_mut(stream.tab_index) {
                        Self::publish_stream_status(pane, stream);
                    }
                    Message::AppShowError(format!("stream error: {error}")).enqueue();
                    return;
                }
                Err(TryRecvError::Empty) => {
                    return;
                }
                Err(TryRecvError::Disconnected) => {
                    if stream.open {
                        stream.open = false;
                        if let Some(pane) = self.tabs.pane_mut(stream.tab_index) {
                            Self::publish_stream_status(pane, stream);
                        }
                        Message::AppShowToast(format!(
                            "stream closed: {} rows",
                            stream.rows_received
                        ))
                        .enqueue();
                    }
                    return;
                }
            }
        }
    }

    fn publish_stream_status(pane: &mut Pane, stream: &StreamSink) {
        pane.set_stream_status(StreamStatus {
            open: stream.open,
            rows_received: stream.rows_received,
            rows_inserted: stream.rows_inserted,
            rows_updated: stream.rows_updated,
        });
    }

    pub fn running(&self) -> bool {
        self.running
    }

    fn show_theme_selector(&mut self) {
        self.overlay = Some(Overlay::ThemeSelector(Default::default()));
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

    fn show_sql_query_picker(&mut self) {
        self.overlay = Some(Overlay::SqlQueryPicker(SqlQueryPicker::new(
            self.tabs
                .selected()
                .map(Pane::table)
                .map(Table::data_frame)
                .cloned(),
        )));
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

        if let Some(toast) = self.toast.as_mut() {
            toast.render(area, buf, FocusState::NotFocused);
        }
    }

    fn handle(&mut self, event: crossterm::event::KeyEvent) -> bool {
        (if let Some(overlay) = self.overlay.as_mut() {
            overlay.responder().handle(event)
        } else if let Some(schema) = self.schema.as_mut() {
            schema.handle(event)
        } else {
            self.tabs.handle(event)
        }) || match event.code {
            KeyCode::Char(':') => {
                self.show_palette();
                true
            }
            KeyCode::Char('Q') => {
                self.quit();
                true
            }
            _ => false,
        }
    }

    fn update(&mut self, action: &Message, _: FocusState) {
        match action {
            Message::Quit => self.quit(),
            Message::AppDismissOverlay => self.dismiss_overlay(),
            Message::AppShowError(message) => self.show_error(message),
            Message::AppShowToast(message) => self.show_toast(message),
            Message::AppShowCommandPicker => self.show_palette(),
            Message::AppShowThemeSelector => self.show_theme_selector(),
            Message::AppShowSchema => self.show_schema(),
            Message::AppShowImporter => self.show_importer(),
            Message::AppDismissSchema => self.dismiss_schema(),
            Message::AppShowSqlQuery => self.show_sql_query_picker(),
            _ => (),
        };
        match (self.overlay.as_mut(), self.schema.as_mut()) {
            (Some(overlay), Some(schema)) => {
                overlay.responder().update(action, FocusState::Focused);
                schema.update(action, FocusState::NotFocused);
                self.tabs.update(action, FocusState::NotFocused);
            }
            (Some(overlay), None) => {
                overlay.responder().update(action, FocusState::Focused);
                self.tabs.update(action, FocusState::NotFocused);
            }
            (None, Some(schema)) => {
                schema.update(action, FocusState::Focused);
                self.tabs.update(action, FocusState::NotFocused);
            }
            (None, None) => {
                self.tabs.update(action, FocusState::Focused);
            }
        }
    }

    fn tick(&mut self) {
        self.drain_stream();
        // Expire flash highlights on the streaming pane.
        if let Some(stream) = self.stream.as_ref()
            && let Some(pane) = self.tabs.pane_mut(stream.tab_index)
        {
            pane.base_table_mut().expire_flashes();
        }
        if let Some(overlay) = self.overlay.as_mut() {
            overlay.responder().tick();
        }
        if let Some(toast) = self.toast.as_mut()
            && toast.is_finished()
        {
            self.toast.take();
        }
        self.tabs.tick();
    }
}

#[derive(Debug)]
pub enum Overlay {
    Error(ErrorPopup),
    CommandPicker(CommandPalette),
    ThemeSelector(ThemeSelector),
    SqlQueryPicker(SqlQueryPicker),
    Import(Importer),
    Help(Help),
}

impl Overlay {
    fn responder(&mut self) -> &mut dyn Component {
        match self {
            Overlay::Error(error) => error,
            Overlay::CommandPicker(command_palette) => command_palette,
            Overlay::ThemeSelector(theme_selector) => theme_selector,
            Overlay::Help(help) => help,
            Overlay::Import(step_by_step) => step_by_step,
            Overlay::SqlQueryPicker(sql_query_picker) => sql_query_picker,
        }
    }
}
