use anyhow::Ok;
use ratatui::{
    Frame,
    layout::{Constraint, Flex, Layout},
};

use crate::{
    AppResult,
    misc::history::History,
    tui::{
        command_pallete::{CommandPallete, CommandPalleteState},
        error_popup::ErrorPopup,
        schema::{Schema, SchemaState},
        tab::{Tab, TabState},
        tabular::Modal,
    },
};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Context {
    Empty,
    Table,
    Sheet,
    Command,
    Error,
    Search,
    Schema,
}

impl Context {
    pub fn parent(&self) -> Option<Context> {
        match self {
            Context::Empty => None,
            Context::Table => Context::Empty.into(),
            Context::Sheet => Context::Table.into(),
            Context::Command => Context::Empty.into(),
            Context::Error => Context::Empty.into(),
            Context::Search => Context::Table.into(),
            Context::Schema => Context::Empty.into(),
        }
    }
}

#[derive(Debug, Default)]
pub enum Overlay {
    Schema(SchemaState),
    #[default]
    Empty,
}

pub struct App {
    tabs: TabState,
    overlay: Overlay,
    error: Option<String>,
    pallete: Option<CommandPalleteState>,
    history: History,
    borders: bool,
    running: bool,
}

impl App {
    pub fn new(tabs: TabState, history: History) -> Self {
        Self {
            tabs,
            overlay: Default::default(),
            error: None,
            pallete: None,
            history,
            borders: true,
            running: true,
        }
    }

    pub fn running(&self) -> bool {
        self.running
    }

    pub fn tabs_mut(&mut self) -> &mut TabState {
        &mut self.tabs
    }

    pub fn pallete_mut(&mut self) -> Option<&mut CommandPalleteState> {
        self.pallete.as_mut()
    }

    pub fn history_mut(&mut self) -> &mut History {
        &mut self.history
    }

    pub fn overlay_mut(&mut self) -> &mut Overlay {
        &mut self.overlay
    }

    pub fn show_pallete(&mut self, cmd: impl ToString) {
        self.pallete = Some(CommandPalleteState::new(cmd.to_string()));
    }

    pub fn hide_pallete(&mut self) -> Option<String> {
        self.pallete
            .take()
            .map(|mut pallete| pallete.input().value().to_owned())
    }

    pub fn error(&mut self, error: impl ToString) {
        self.error = Some(error.to_string());
    }

    pub fn dismiss_error(&mut self) {
        self.error = None;
    }

    pub fn show_schema(&mut self) {
        self.overlay = Overlay::Schema(Default::default());
    }

    pub fn hide_schema(&mut self) {
        self.overlay = Overlay::Empty;
    }

    pub fn toggle_borders(&mut self) {
        self.borders = !self.borders;
    }

    pub fn tick(&mut self) -> AppResult<()> {
        for tab in self.tabs.iter_mut() {
            tab.tick();
        }
        Ok(())
    }

    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn context(&self) -> Context {
        if self.error.is_some() {
            Context::Error
        } else if self.pallete.is_some() {
            Context::Command
        } else if let Overlay::Schema(_) = self.overlay {
            Context::Schema
        } else if let Some(tabular) = self.tabs.selected() {
            match tabular.modal() {
                Modal::SearchBar(_) => Context::Search,
                Modal::Sheet(_) => Context::Sheet,
                Modal::None => Context::Table,
            }
        } else {
            Context::Empty
        }
    }

    pub fn draw(&mut self, frame: &mut Frame) -> AppResult<()> {
        // Draw table / item
        let state = self.context();
        match &mut self.overlay {
            Overlay::Schema(schema) => {
                frame.render_stateful_widget(Schema::default(), frame.area(), schema);
            }
            Overlay::Empty => {
                frame.render_stateful_widget(
                    Tab::new()
                        .with_borders(self.borders)
                        .selection(matches!(state, Context::Table)),
                    frame.area(),
                    &mut self.tabs,
                );
            }
        }

        if let Some(msg) = self.error.as_ref() {
            let error = ErrorPopup::new().with_message(msg);
            let mid = {
                let [mid_ver] = Layout::vertical([Constraint::Length(error.line_count(50))])
                    .flex(Flex::Center)
                    .areas(frame.area());
                let [mid_hor] = Layout::horizontal([Constraint::Length(50)])
                    .flex(Flex::Center)
                    .areas(mid_ver);
                mid_hor
            };
            frame.render_widget(error, mid);
        }

        if let Some(cmd) = self.pallete.as_mut() {
            let upmid = {
                let [mid_ver] = Layout::horizontal([Constraint::Max(80)])
                    .flex(Flex::Center)
                    .areas(frame.area());
                let [_, mid_hor] =
                    Layout::vertical([Constraint::Length(3), Constraint::Length(15)])
                        .areas(mid_ver);
                mid_hor
            };
            frame.render_stateful_widget(
                CommandPallete::new(self.history.iter().take(100)),
                upmid,
                cmd,
            );
        }

        Ok(())
    }
}
