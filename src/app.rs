use std::sync::mpsc::{Receiver, TryRecvError};

use polars::frame::DataFrame;

use crate::misc::sql::{Source as SqlSource, sql};
use crate::reader::StreamEvent;
use crate::tui::Pane;
use crate::tui::popups::sql_query_picker::SqlQueryPicker;
use crate::tui::table::Table;
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
}

impl StreamSink {
    pub fn new(rx: Receiver<StreamEvent>, tab_index: usize, table_name: String) -> Self {
        Self {
            rx,
            tab_index,
            table_name,
            open: true,
            rows_received: 0,
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
                        let current = pane.table().data_frame();
                        if current.width() == 0 {
                            // Initial schema: replace the placeholder with an
                            // empty frame that has the right columns so the
                            // header row renders immediately.
                            let df = DataFrame::empty_with_schema(&schema);
                            sql().refresh_frame(
                                &stream.table_name,
                                df.clone(),
                                SqlSource::Stdin,
                            );
                            pane.table_mut().set_data_frame(df);
                        }
                    }
                }
                Ok(StreamEvent::Batch { rows, .. }) => {
                    if let Some(pane) = self.tabs.pane_mut(stream.tab_index) {
                        let df = pane.table_mut().data_frame_mut();
                        let new_rows = rows.height() as u64;
                        if df.width() == 0 {
                            *df = rows;
                        } else if let Err(err) = df.vstack_mut_owned(rows) {
                            Message::AppShowError(format!("stream append failed: {err}"))
                                .enqueue();
                            stream.open = false;
                            return;
                        }
                        stream.rows_received += new_rows;
                        let refreshed = df.clone();
                        sql().refresh_frame(&stream.table_name, refreshed, SqlSource::Stdin);
                    }
                }
                Ok(StreamEvent::Eof { .. }) => {
                    stream.open = false;
                    Message::AppShowToast(format!(
                        "stream closed: {} rows",
                        stream.rows_received
                    ))
                    .enqueue();
                    return;
                }
                Ok(StreamEvent::Error { error, .. }) => {
                    stream.open = false;
                    Message::AppShowError(format!("stream error: {error}")).enqueue();
                    return;
                }
                Err(TryRecvError::Empty) => return,
                Err(TryRecvError::Disconnected) => {
                    if stream.open {
                        stream.open = false;
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
