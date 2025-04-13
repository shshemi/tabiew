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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Content {
    Schema,
    Tabulars,
}

pub struct App {
    tabs: TabState,
    schema: SchemaState,
    content: Content,
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
            history,
            schema: SchemaState::default(),
            content: Content::Tabulars,
            error: None,
            pallete: None,
            borders: true,
            running: true,
        }
    }

    pub fn running(&self) -> bool {
        self.running
    }

    pub fn tabs(&self) -> &TabState {
        &self.tabs
    }

    pub fn tabs_mut(&mut self) -> &mut TabState {
        &mut self.tabs
    }

    pub fn schema(&self) -> &SchemaState {
        &self.schema
    }

    pub fn schema_mut(&mut self) -> &mut SchemaState {
        &mut self.schema
    }

    pub fn pallete_mut(&mut self) -> Option<&mut CommandPalleteState> {
        self.pallete.as_mut()
    }

    pub fn history_mut(&mut self) -> &mut History {
        &mut self.history
    }

    pub fn content(&self) -> &Content {
        &self.content
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

    pub fn switch_schema(&mut self) {
        self.content = Content::Schema;
    }

    pub fn switch_tabular(&mut self) {
        self.content = Content::Tabulars;
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
        } else if let Content::Schema = self.content {
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
        match &mut self.content {
            Content::Schema => {
                frame.render_stateful_widget(Schema::default(), frame.area(), &mut self.schema);
            }
            Content::Tabulars => {
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
