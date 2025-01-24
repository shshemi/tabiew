use ratatui::{
    layout::{Constraint, Flex, Layout},
    Frame,
};

use crate::{
    tui::{
        error_popup::ErrorPopup,
        status_bar::{StatusBar, StatusBarState, StatusBarView},
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
    Pallete,
}

pub struct App {
    tabs: TabsState,
    status_bar: StatusBarState,
    error: Option<String>,
    running: bool,
}

impl App {
    pub fn new(tabs: TabsState) -> Self {
        Self {
            tabs,
            status_bar: StatusBarState::new(),
            error: None,
            running: true,
        }
    }

    pub fn running(&self) -> bool {
        self.running
    }

    pub fn tabs(&mut self) -> &mut TabsState {
        &mut self.tabs
    }

    pub fn status_bar(&mut self) -> &mut StatusBarState {
        &mut self.status_bar
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
        self.status_bar.tick()
    }

    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn context(&self) -> AppContext {
        match (
            self.error.as_ref(),
            self.status_bar.view(),
            self.tabs.selected().map(TabContentState::modal),
        ) {
            (Some(_), _, _) => AppContext::Error,
            (None, StatusBarView::Info, None) => AppContext::Empty,
            (None, StatusBarView::Info, Some(None)) => AppContext::Table,
            (None, StatusBarView::Info, Some(Some(Modal::Sheet(_)))) => AppContext::Sheet,
            (None, StatusBarView::Info, Some(Some(Modal::Search(_, _)))) => AppContext::Search,
            (None, StatusBarView::Error(_), _) => AppContext::Error,
            (None, StatusBarView::Prompt(_), _) => AppContext::Command,
            (None, StatusBarView::Search(_), _) => AppContext::Search,
        }
    }

    pub fn draw<Theme: Styler>(&mut self, frame: &mut Frame) -> AppResult<()> {
        let [table_area, status_bar_area] =
            Layout::vertical([Constraint::Fill(1), Constraint::Length(1)]).areas(frame.area());

        // Draw table / item
        let state = self.context();
        frame.render_stateful_widget(
            Tabs::<Theme>::new().selection(matches!(state, AppContext::Table)),
            table_area,
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
        if let Some(_tab) = self.tabs.selected() {
        } else {
            frame.render_stateful_widget(
                StatusBar::<Theme>::new(&[]),
                status_bar_area,
                &mut self.status_bar,
            );
        }
        Ok(())
    }
}
