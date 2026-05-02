use crate::misc::config::config;
use crate::misc::download::BackgroundDownloader;
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
            command_palette::CommandPalette, help_modal::Help, importer::Importer,
            theme_selector::ThemeSelector,
        },
        schema::schema::Schema,
    },
};
use crossterm::event::KeyCode;
use itertools::Itertools;
use ratatui::layout::{Constraint, Direction, Flex, Layout, Rect};

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

    fn add_download(&mut self, url: &str) {
        self.dls.push(DownloadNotification::new(
            url.to_owned(),
            BackgroundDownloader::new(url.to_owned()),
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
            Message::AppReloadConfig => self.reload_app_config(),
            Message::AppDownloadDataSource(url) => self.add_download(url),
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
        if let Some(toast) = self.toast.as_mut()
            && toast.is_finished()
        {
            self.toast.take();
        }
        self.dls
            .iter()
            .enumerate()
            .filter_map(|(idx, dl)| dl.is_done().then_some(idx))
            .collect_vec()
            .into_iter()
            .rev()
            .for_each(|idx| {
                self.dls.remove(idx);
            });
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
