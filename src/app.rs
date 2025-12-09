use crate::{
    handler::message::Message,
    tui::{
        component::Component,
        popups::{command_picker::CommandPicker, help_modal::Help, theme_selector::ThemeSelector},
        schema::schema::Schema,
    },
};
use crate::{
    misc::history::History,
    tui::{error_popup::ErrorPopup, tabs::Tabs},
};
use crossterm::event::KeyCode;
use ratatui::layout::{Constraint, Flex, Layout};

#[derive(Debug)]
pub enum Overlay {
    Error(ErrorPopup),
    CommandPicker(CommandPicker),
    ThemeSelector(ThemeSelector),
    Help(Help),
}

impl Overlay {
    fn responder(&mut self) -> &mut dyn Component {
        match self {
            Overlay::Error(error) => error,
            Overlay::CommandPicker(command_palette) => command_palette,
            Overlay::ThemeSelector(theme_selector) => theme_selector,
            Overlay::Help(help) => help,
        }
    }
}

pub struct App {
    tabs: Tabs,
    overlay: Option<Overlay>,
    history: History,
    schema: Option<Schema>,
    running: bool,
}

impl App {
    pub fn new(tabs: Tabs, history: History) -> Self {
        Self {
            tabs,
            history,
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

    fn show_palette(&mut self, _cmd: impl ToString) {
        self.overlay = Some(Overlay::CommandPicker(CommandPicker::default()));
    }

    fn show_error(&mut self, message: impl Into<String>) {
        self.overlay = Some(Overlay::Error(ErrorPopup::new(message)));
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
        focus_state: crate::tui::component::FocusState,
    ) {
        if let Some(schema) = self.schema.as_mut() {
            schema.render(area, buf, focus_state);
        } else {
            self.tabs.render(area, buf, focus_state);
        }
        match self.overlay.as_mut() {
            Some(Overlay::Error(error)) => {
                error.render(area, buf, focus_state);
            }
            Some(Overlay::CommandPicker(cmd)) => {
                let upmid = {
                    let [mid_ver] = Layout::horizontal([Constraint::Max(80)])
                        .flex(Flex::Center)
                        .areas(area);
                    let [_, mid_hor] =
                        Layout::vertical([Constraint::Length(3), Constraint::Length(15)])
                            .areas(mid_ver);
                    mid_hor
                };
                cmd.render(upmid, buf, focus_state);
            }
            Some(Overlay::ThemeSelector(theme_selector)) => {
                theme_selector.render(area, buf, focus_state);
            }
            Some(Overlay::Help(help)) => {
                help.render(area, buf, focus_state);
            }
            None => {}
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
                    self.show_palette("");
                    true
                }
                KeyCode::Char('Q') => {
                    self.quit();
                    true
                }
                _ => false,
            }
    }

    fn update(&mut self, action: &Message) {
        match action {
            Message::Quit => self.quit(),
            Message::AppDismissOverlay => self.dismiss_overlay(),
            Message::AppShowError(message) => self.show_error(message),
            Message::AppShowCommandPicker => self.show_palette(""),
            Message::AppShowThemeSelector => self.show_theme_selector(),
            Message::AppShowSchema => self.show_schema(),
            Message::AppDismissSchema => self.dismiss_schema(),
            _ => (),
        };
        if let Some(overlay) = self.overlay.as_mut() {
            overlay.responder().update(action);
        }
        self.tabs.update(action);
    }

    fn tick(&mut self) {
        if let Some(overlay) = self.overlay.as_mut() {
            overlay.responder().tick();
        }
        self.tabs.tick();
    }
}
