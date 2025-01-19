use ratatui::{
    layout::{Constraint, Layout},
    Frame,
};

use crate::{
    tui::{
        status_bar::{StatusBar, StatusBarState, StatusBarTag, StatusBarView}, tabs::{Tabs, TabsState}, tab_content::TabularMode, Styler, TabularSource, TabContentState
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
    running: bool,
}

impl App {
    pub fn new(tabs: TabsState) -> Self {
        Self {
            tabs,
            status_bar: StatusBarState::new(),
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
        self.status_bar().switch_error(error);
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
            self.status_bar.view(),
            self.tabs.selected().map(TabContentState::mode),
        ) {
            (StatusBarView::Info, None) => AppContext::Empty,
            (StatusBarView::Info, Some(TabularMode::Table)) => AppContext::Table,
            (StatusBarView::Info, Some(TabularMode::Sheet(_))) => AppContext::Sheet,
            (StatusBarView::Info, Some(TabularMode::Search(_, _))) => AppContext::Search,
            (StatusBarView::Error(_), _) => AppContext::Error,
            (StatusBarView::Prompt(_), _) => AppContext::Command,
            (StatusBarView::Search(_), _) => AppContext::Search,
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

        if let Some(tab) = self.tabs.selected() {
            frame.render_stateful_widget(
                StatusBar::<Theme>::new(&[
                    StatusBarTag::new(
                        match tab.tabular_source() {
                            TabularSource::Help | TabularSource::Schema | TabularSource::Name(_) => {
                                "Table"
                            }
                            TabularSource::Query(_) => "SQL",
                        },
                        match tab.tabular_source() {
                            TabularSource::Help => "Help",
                            TabularSource::Schema => "Schema",
                            TabularSource::Name(name) => name.as_str(),
                            TabularSource::Query(query) => query.as_str(),
                        },
                    ),
                    StatusBarTag::new(
                        "Tab",
                        &format!(
                            "{:>width$} / {}",
                            self.tabs.idx() + 1,
                            self.tabs.len(),
                            width = self.tabs.len().to_string().len()
                        ),
                    ),
                    StatusBarTag::new(
                        "Row",
                        &format!(
                            "{:>width$}",
                            tab.selected() + 1,
                            width = tab.data_frame().height().to_string().len()
                        ),
                    ),
                    StatusBarTag::new(
                        "Shape",
                        &format!(
                            "{} x {}",
                            tab.data_frame().height(),
                            tab.data_frame().width()
                        ),
                    ),
                ]),
                status_bar_area,
                &mut self.status_bar,
            );
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
