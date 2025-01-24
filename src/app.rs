use anyhow::Ok;
use ratatui::{
    layout::{Constraint, Flex, Layout},
    Frame,
};

use crate::{
    tui::{
        command_pallete::{CommandPallete, CommandPalleteState},
        error_popup::ErrorPopup,
        tab_content::Modal,
        tabs::{Tabs, TabsState},
        Styler, TabContentState,
    },
    AppResult,
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
            AppContext::Command => AppContext::Command.into(),
            AppContext::Error => AppContext::Empty.into(),
            AppContext::Search => AppContext::Table.into(),
        }
    }
}

pub struct App {
    tabs: TabsState,
    error: Option<String>,
    pallete: Option<CommandPalleteState>,
    running: bool,
}

impl App {
    pub fn new(tabs: TabsState) -> Self {
        Self {
            tabs,
            error: None,
            pallete: None,
            running: true,
        }
    }

    pub fn running(&self) -> bool {
        self.running
    }

    pub fn tabs(&mut self) -> &mut TabsState {
        &mut self.tabs
    }

    pub fn pallete(&mut self) -> Option<&mut CommandPalleteState> {
        self.pallete.as_mut()
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
        } else {
            match self.tabs.selected().map(TabContentState::modal) {
                Some(Some(Modal::Search(_, _))) => AppContext::Search,
                Some(Some(Modal::Sheet(_))) => AppContext::Sheet,
                Some(None) => AppContext::Table,
                None => AppContext::Empty,
            }
        }
        // match (
        //     self.error.as_ref(),
        //     self.status_bar.view(),
        //     self.tabs.selected().map(TabContentState::modal),
        // ) {
        //     (Some(_), _, _) => AppContext::Error,
        //     (None, StatusBarView::Info, None) => AppContext::Empty,
        //     (None, StatusBarView::Info, Some(None)) => AppContext::Table,
        //     (None, StatusBarView::Info, Some(Some(Modal::Sheet(_)))) => AppContext::Sheet,
        //     (None, StatusBarView::Info, Some(Some(Modal::Search(_, _)))) => AppContext::Search,
        //     (None, StatusBarView::Error(_), _) => AppContext::Error,
        //     (None, StatusBarView::Prompt(_), _) => AppContext::Command,
        //     (None, StatusBarView::Search(_), _) => AppContext::Search,
        // }
    }

    pub fn draw<Theme: Styler>(&mut self, frame: &mut Frame) -> AppResult<()> {
        // Draw table / item
        let state = self.context();
        frame.render_stateful_widget(
            Tabs::<Theme>::new().selection(matches!(state, AppContext::Table)),
            frame.area(),
            &mut self.tabs,
        );

        if let Some(msg) = self.error.as_ref() {
            let error = ErrorPopup::<Theme>::new().with_message(msg);
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
                    Layout::vertical([Constraint::Length(3), Constraint::Length(3)])
                        .areas(mid_ver);
                mid_hor
            };
            frame.render_stateful_widget(CommandPallete::<Theme>::new(), upmid, cmd);
        }

        Ok(())
    }
}
