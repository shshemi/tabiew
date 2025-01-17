use ratatui::{
    layout::{Constraint, Layout},
    Frame,
};

use crate::{
    tui::{
        pallete::{Pallete, PalleteState},
        status_bar::{StatusBar, StatusBarState, StatusBarTag, StatusBarView},
        tabs::{Tabs, TabsState},
        tabular::TabularView,
        Styler, TabularState, TabularType,
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
    pallete: Option<PalleteState>,
}

impl App {
    pub fn new(tabs: TabsState) -> Self {
        Self {
            tabs,
            status_bar: StatusBarState::new(),
            running: true,
            pallete: Default::default(),
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

    pub fn pallete(&mut self) -> &mut Option<PalleteState> {
        &mut self.pallete
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
            self.pallete.as_ref(),
            self.status_bar.view(),
            self.tabs.selected().map(TabularState::view),
        ) {
            (Some(_), _, _) => AppContext::Pallete,
            (_, StatusBarView::Error(_), _) => AppContext::Error,
            (_, StatusBarView::Prompt(_), _) => AppContext::Command,
            (_, StatusBarView::Search(_), _) => AppContext::Search,
            (_, _, Some(TabularView::Sheet(_))) => AppContext::Sheet,
            (_, _, Some(TabularView::Table)) => AppContext::Table,
            (_, _, None) => AppContext::Empty,
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
                        match tab.tabular_type() {
                            TabularType::Help | TabularType::Schema | TabularType::Name(_) => {
                                "Table"
                            }
                            TabularType::Query(_) => "SQL",
                        },
                        match tab.tabular_type() {
                            TabularType::Help => "Help",
                            TabularType::Schema => "Schema",
                            TabularType::Name(name) => name.as_str(),
                            TabularType::Query(query) => query.as_str(),
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

        if let Some(state) = &mut self.pallete {
            frame.render_stateful_widget(
                Pallete::<Theme>::new()
                    .with_horizontal_pad(5)
                    .with_items((0..10).map(|idx| format!("Item {}", idx))),
                frame.area(),
                state,
            );
        }

        Ok(())
    }
}
