use crate::tui::Pane;
use crate::tui::popups::sql_query_picker::SqlQueryPicker;
use crate::tui::table::Table;
use crate::tui::{error_popup::ErrorPopup, tabs::Tabs};
use crate::{
    handler::message::Message,
    tui::{
        component::{Component, FocusState},
        popups::{
            command_picker::CommandPicker, help_modal::Help, import_wizard::ImportWizard,
            theme_selector::ThemeSelector,
        },
        schema::schema::Schema,
    },
};
use crossterm::event::KeyCode;

pub struct App {
    tabs: Tabs,
    overlay: Option<Overlay>,
    schema: Option<Schema>,
    running: bool,
}

impl App {
    pub fn new(tabs: Tabs) -> Self {
        Self {
            tabs,
            overlay: None,
            schema: None,
            running: true,
        }
    }

    pub fn running(&self) -> bool {
        self.running
    }

    fn show_theme_selector(&mut self) {
        self.overlay = Some(Overlay::ThemeSelector(Default::default()));
    }

    fn show_palette(&mut self) {
        self.overlay = Some(Overlay::CommandPicker(CommandPicker::default()));
    }

    fn show_error(&mut self, message: impl Into<String>) {
        self.overlay = Some(Overlay::Error(ErrorPopup::new(message)));
    }

    fn show_import_wizard(&mut self) {
        self.overlay = Some(Overlay::Import(ImportWizard::default()))
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
    }

    fn handle(&mut self, event: crossterm::event::KeyEvent) -> bool {
        self.overlay
            .as_mut()
            .map(|overlay| overlay.responder().handle(event))
            .unwrap_or_else(|| {
                if let Some(schema) = self.schema.as_mut() {
                    schema.handle(event)
                } else {
                    self.tabs.handle(event)
                }
            })
            || match event.code {
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
            Message::AppShowCommandPicker => self.show_palette(),
            Message::AppShowThemeSelector => self.show_theme_selector(),
            Message::AppShowSchema => self.show_schema(),
            Message::AppShowImportWizard => self.show_import_wizard(),
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
        if let Some(overlay) = self.overlay.as_mut() {
            overlay.responder().tick();
        }
        self.tabs.tick();
    }
}

#[derive(Debug)]
pub enum Overlay {
    Error(ErrorPopup),
    CommandPicker(CommandPicker),
    ThemeSelector(ThemeSelector),
    SqlQueryPicker(SqlQueryPicker),
    Import(ImportWizard),
    Help(Help),
}

impl Overlay {
    fn responder(&mut self) -> &mut dyn Component {
        match self {
            Overlay::Error(error) => error,
            Overlay::CommandPicker(command_palette) => command_palette,
            Overlay::ThemeSelector(theme_selector) => theme_selector,
            Overlay::Help(help) => help,
            Overlay::Import(wizard) => wizard,
            Overlay::SqlQueryPicker(sql_query_picker) => sql_query_picker,
        }
    }
}
