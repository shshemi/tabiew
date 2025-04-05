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
        tabs::{Tab, Tabs, TabsState},
        tabular::Modal,
    },
};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum AppContext {
    Empty,
    Table,
    Sheet,
    Command,
    Error,
    Search,
}

impl AppContext {
    pub fn parent(&self) -> Option<AppContext> {
        match self {
            AppContext::Empty => None,
            AppContext::Table => AppContext::Empty.into(),
            AppContext::Sheet => AppContext::Table.into(),
            AppContext::Command => AppContext::Empty.into(),
            AppContext::Error => AppContext::Empty.into(),
            AppContext::Search => AppContext::Table.into(),
        }
    }
}

pub struct App {
    tabs: TabsState,
    error: Option<String>,
    pallete: Option<CommandPalleteState>,
    history: History,
    borders: bool,
    running: bool,
}

impl App {
    pub fn new(tabs: TabsState, history: History) -> Self {
        Self {
            tabs,
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

    pub fn tabs_mut(&mut self) -> &mut TabsState {
        &mut self.tabs
    }

    pub fn pallete_mut(&mut self) -> Option<&mut CommandPalleteState> {
        self.pallete.as_mut()
    }

    pub fn history_mut(&mut self) -> &mut History {
        &mut self.history
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

    pub fn context(&self) -> AppContext {
        if self.error.is_some() {
            AppContext::Error
        } else if self.pallete.is_some() {
            AppContext::Command
        } else if let Some(tab) = self.tabs.selected() {
            match tab {
                Tab::Tabular(tabular) => match tabular.modal() {
                    Modal::SearchBar(_) => AppContext::Search,
                    Modal::Sheet(_) => AppContext::Sheet,
                    Modal::None => AppContext::Table,
                },
            }
        } else {
            AppContext::Empty
        }
    }

    pub fn draw(&mut self, frame: &mut Frame) -> AppResult<()> {
        // Draw table / item
        let state = self.context();
        frame.render_stateful_widget(
            Tabs::new()
                .with_borders(self.borders)
                .selection(matches!(state, AppContext::Table)),
            frame.area(),
            &mut self.tabs,
        );

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
