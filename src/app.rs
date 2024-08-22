use std::error;
use std::ops::Div;

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::layout::{Constraint, Layout};
use ratatui::Frame;
use status_bar::{StatusBar, StatusBarState};
use tabular::{Tabular, TabularType};

use crate::command::{CommandRegistery, Commands};
use crate::keybind::{Action, Keybind};
use crate::sql::SqlBackend;
use crate::theme::Styler;

pub mod status_bar;
pub mod tabular;

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug, Default)]
pub struct Tabs {
    tabulars: Vec<Tabular>,
    idx: usize,
}

pub struct App {
    tabs: Tabs,
    status_bar: StatusBar,
    sql: SqlBackend,
    exec_table: CommandRegistery,
    keybindings: Keybind,
    running: bool,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum AppState {
    Empty,
    Table,
    Sheet,
    Chart,
    Command,
    Error,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum ChartNav {
    Init,
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum AppAction {
    StatusBarStats,
    StatusBarCommand(String),
    StatausBarError(String),
    TabularTableView,
    TabularSheetView,
    TabularChartView(ChartNav),
    TabularSwitchView,
    SqlQuery(String),
    SqlSchema,
    TabularGoto(usize),
    TabularGotoFirst,
    TabularGotoLast,
    TabularGotoRandom,
    TabularGoUp(usize),
    TabularGoUpHalfPage,
    TabularGoUpFullPage,
    TabularGoDown(usize),
    TabularGoDownHalfPage,
    TabularGoDownFullPage,
    SheetScrollUp,
    SheetScrollDown,
    TabularReset,
    TabularSelect(String),
    TabularOrder(String),
    TabularFilter(String),
    TabNew(String),
    TabSelect(usize),
    TabRemove(usize),
    TabRemoveSelected,
    TabSelectedPrev,
    TabSelectedNext,
    TabRemoveOrQuit,
    TabRename(usize, String),
    Help,
    Quit,
}

impl App {
    pub fn new(
        tabs: Tabs,
        status_bar: StatusBar,
        sql: SqlBackend,
        exec_table: CommandRegistery,
        key_bind: Keybind,
    ) -> Self {
        Self {
            tabs,
            status_bar,
            sql,
            exec_table,
            keybindings: key_bind,
            running: true,
        }
    }

    pub fn running(&self) -> bool {
        self.running
    }

    pub fn tick(&mut self) -> AppResult<()> {
        self.tabs.selected_mut().map(|tab| tab.tick());
        self.status_bar.tick()
    }

    pub fn quit(&mut self) -> AppResult<()> {
        self.running = false;
        Ok(())
    }

    pub fn infer_state(&self) -> AppState {
        match (
            self.tabs.selected().map(Tabular::state),
            self.status_bar.state(),
        ) {
            (Some(tabular::TabularState::Table), StatusBarState::Info) => AppState::Table,
            (Some(tabular::TabularState::Table), StatusBarState::Error(_)) => AppState::Error,
            (Some(tabular::TabularState::Table), StatusBarState::Prompt(_)) => AppState::Command,
            (Some(tabular::TabularState::Sheet(_)), StatusBarState::Info) => AppState::Sheet,
            (Some(tabular::TabularState::Sheet(_)), StatusBarState::Error(_)) => AppState::Error,
            (Some(tabular::TabularState::Sheet(_)), StatusBarState::Prompt(_)) => AppState::Command,
            (Some(tabular::TabularState::Chart(_)), StatusBarState::Info) => AppState::Table,
            (Some(tabular::TabularState::Chart(_)), StatusBarState::Error(_)) => AppState::Error,
            (Some(tabular::TabularState::Chart(_)), StatusBarState::Prompt(_)) => AppState::Command,
            (None, StatusBarState::Info) => AppState::Empty,
            (None, StatusBarState::Error(_)) => AppState::Error,
            (None, StatusBarState::Prompt(_)) => AppState::Command,
        }
    }

    pub fn draw<Theme: Styler>(&mut self, frame: &mut Frame) -> AppResult<()> {
        let layout =
            Layout::vertical([Constraint::Fill(1), Constraint::Length(1)]).split(frame.area());

        // Draw table / item
        let state = self.infer_state();
        if let Some(tab) = self.tabs.selected_mut() {
            tab.render::<Theme>(frame, layout[0], matches!(state, AppState::Table))?;
        }
        if let Some(tab) = self.tabs.selected() {
            self.status_bar.render::<Theme>(
                frame,
                layout[1],
                &[
                    (
                        match tab.tabular_type() {
                            TabularType::Help => "Table",
                            TabularType::Schema => "Table",
                            TabularType::Name(_) => "Table",
                            TabularType::Query(_) => "SQL",
                        },
                        match tab.tabular_type() {
                            TabularType::Help => "Help",
                            TabularType::Schema => "Schema",
                            TabularType::Name(name) => name,
                            TabularType::Query(query) => query,
                        },
                    ),
                    (
                        "Tab",
                        &format!(
                            "{:>width$} / {}",
                            self.tabs.idx() + 1,
                            self.tabs.len(),
                            width = tab.page_len().to_string().len()
                        ),
                    ),
                    (
                        "Row",
                        &format!(
                            "{:>width$}",
                            tab.selected() + 1,
                            width = tab.table_values().height().to_string().len()
                        ),
                    ),
                    (
                        "Shape",
                        &format!(
                            "{} x {}",
                            tab.table_values().height(),
                            tab.table_values().width()
                        ),
                    ),
                ],
            )
        } else {
            self.status_bar.render::<Theme>(frame, layout[1], &[])
        }
    }

    pub fn handle_key_event(&mut self, key_event: KeyEvent) -> AppResult<()> {
        let state = self.infer_state();
        let key_code = key_event.code;
        match (state, key_code) {
            (AppState::Command | AppState::Error, KeyCode::Esc) => self.status_bar.show_info(),

            (AppState::Command, KeyCode::Enter) => {
                if let Some(command) = self.status_bar.commit_prompt() {
                    let (s1, s2) = command.split_once(' ').unwrap_or((command.as_ref(), ""));
                    if let Some(parse_fn) = self.exec_table.get(s1) {
                        match parse_fn(s2).and_then(|action| self.invoke(action)) {
                            Ok(_) => self.status_bar.show_info(),
                            Err(error) => self.status_bar.show_error(error),
                        }
                    } else {
                        self.status_bar.show_error("Command not found")
                    }
                } else {
                    self.status_bar
                        .show_error("Invalid state; consider restarting Tabiew")
                }
            }

            (AppState::Command, _) => self.status_bar.input(key_event),

            (_, KeyCode::Char(':')) => self.status_bar.show_prompt(""),

            _ => {
                match self
                    .keybindings
                    .get_action(state, key_event)
                    .cloned()
                    .map(|action| self.invoke(action))
                {
                    Some(Err(error)) => self.status_bar.show_error(error),
                    _ => Ok(()),
                }
            }
        }
    }
    fn invoke(&mut self, action: Action) -> AppResult<()> {
        match action {
            AppAction::StatusBarStats => self.status_bar.show_info(),

            AppAction::StatusBarCommand(prefix) => self.status_bar.show_prompt(prefix),

            AppAction::StatausBarError(msg) => self.status_bar.show_error(msg),

            AppAction::TabularTableView => {
                if let Some(tab) = self.tabs.selected_mut() {
                    tab.show_table()
                } else {
                    Ok(())
                }
            }

            AppAction::TabularSheetView => {
                if let Some(tab) = self.tabs.selected_mut() {
                    tab.show_sheet()
                } else {
                    Ok(())
                }
            }

            AppAction::TabularChartView(Nav) => {
                if let Some(tab) = self.tabs.selected_mut() {
                    tab.show_chart()
                } else {
                    Ok(())
                }
            }

            AppAction::TabularSwitchView => {
                if let Some(tab) = self.tabs.selected_mut() {
                    tab.switch_view()
                } else {
                    Ok(())
                }
            }

            AppAction::SqlQuery(query) => {
                if let Some(tab) = self.tabs.selected_mut() {
                    tab.set_data_frame(self.sql.execute(&query)?)
                } else {
                    Ok(())
                }
            }

            AppAction::SqlSchema => {
                let idx = self.tabs.iter().enumerate().find_map(|(idx, tab)| {
                    matches!(tab.tabular_type(), TabularType::Schema).then_some(idx)
                });
                if let Some(idx) = idx {
                    self.tabs.select(idx)
                } else {
                    self.tabs
                        .add(Tabular::new(self.sql.schema(), TabularType::Schema))?;
                    self.tabs.select_last()
                }
            }

            AppAction::TabularGoto(line) => {
                if let Some(tab) = self.tabs.selected_mut() {
                    tab.select(line)
                } else {
                    Ok(())
                }
            }

            AppAction::TabularGotoFirst => {
                if let Some(tab) = self.tabs.selected_mut() {
                    tab.select_first()
                } else {
                    Ok(())
                }
            }

            AppAction::TabularGotoLast => {
                if let Some(tab) = self.tabs.selected_mut() {
                    tab.select_last()
                } else {
                    Ok(())
                }
            }

            AppAction::TabularGotoRandom => {
                if let Some(tab) = self.tabs.selected_mut() {
                    tab.select_random()
                } else {
                    Ok(())
                }
            }

            AppAction::TabularGoUp(lines) => {
                if let Some(tab) = self.tabs.selected_mut() {
                    tab.select_up(lines)
                } else {
                    Ok(())
                }
            }

            AppAction::TabularGoUpHalfPage => {
                if let Some(tab) = self.tabs.selected_mut() {
                    tab.select_up(tab.page_len().div(2))
                } else {
                    Ok(())
                }
            }

            AppAction::TabularGoUpFullPage => {
                if let Some(tab) = self.tabs.selected_mut() {
                    tab.select_up(tab.page_len())
                } else {
                    Ok(())
                }
            }

            AppAction::TabularGoDown(lines) => {
                if let Some(tab) = self.tabs.selected_mut() {
                    tab.select_down(lines)
                } else {
                    Ok(())
                }
            }

            AppAction::TabularGoDownHalfPage => {
                if let Some(tab) = self.tabs.selected_mut() {
                    tab.select_down(tab.page_len().div(2))
                } else {
                    Ok(())
                }
            }

            AppAction::TabularGoDownFullPage => {
                if let Some(tab) = self.tabs.selected_mut() {
                    tab.select_down(tab.page_len())
                } else {
                    Ok(())
                }
            }

            AppAction::SheetScrollUp => {
                if let Some(tab) = self.tabs.selected_mut() {
                    tab.scroll_up()
                } else {
                    Ok(())
                }
            }

            AppAction::SheetScrollDown => {
                if let Some(tab) = self.tabs.selected_mut() {
                    tab.scroll_down()
                } else {
                    Ok(())
                }
            }

            AppAction::TabularReset => {
                if let Some(tab) = self.tabs.selected_mut() {
                    tab.set_data_frame(match tab.tabular_type() {
                        TabularType::Help => Commands::default().into_data_frame(),
                        TabularType::Schema => self.sql.schema(),
                        TabularType::Name(name) => self
                            .sql
                            .execute(format!("SELECT * FROM {}", name).as_str())?,
                        TabularType::Query(query) => self.sql.execute(query)?,
                    })
                } else {
                    Ok(())
                }
            }

            AppAction::TabularSelect(select) => {
                if let Some(tab) = self.tabs.selected_mut() {
                    let mut sql = SqlBackend::new();
                    sql.register("df", tab.data_frame().clone(), "".into());
                    tab.set_data_frame(sql.execute(&format!("SELECT {} FROM df", select))?)
                } else {
                    Ok(())
                }
            }

            AppAction::TabularOrder(order) => {
                if let Some(tab) = self.tabs.selected_mut() {
                    let mut sql = SqlBackend::new();
                    sql.register("df", tab.data_frame().clone(), "".into());
                    tab.set_data_frame(
                        sql.execute(&format!("SELECT * FROM df ORDER BY {}", order))?,
                    )
                } else {
                    Ok(())
                }
            }

            AppAction::TabularFilter(filter) => {
                if let Some(tab) = self.tabs.selected_mut() {
                    let mut sql = SqlBackend::new();
                    sql.register("df", tab.data_frame().clone(), "".into());
                    tab.set_data_frame(sql.execute(&format!("SELECT * FROM df where {}", filter))?)
                } else {
                    Ok(())
                }
            }

            AppAction::TabNew(query) => {
                if self.sql.contains_dataframe(&query) {
                    let df = self.sql.execute(&format!("SELECT * FROM {}", query))?;
                    self.tabs.add(Tabular::new(df, TabularType::Name(query)))?;
                } else {
                    let df = self.sql.execute(&query)?;
                    self.tabs.add(Tabular::new(df, TabularType::Query(query)))?;
                }
                self.tabs.select_last()
            }

            AppAction::TabSelect(idx) => {
                if idx == 0 {
                    Err("zero is not a valid tab".into())
                } else if idx <= self.tabs.len() {
                    self.tabs.select(idx.saturating_sub(1))
                } else {
                    Err(format!(
                        "index {} is out of bound, maximum is {}",
                        idx,
                        self.tabs.len()
                    )
                    .into())
                }
            }

            AppAction::TabRemove(idx) => self.tabs.remove(idx),

            AppAction::TabRemoveSelected => self.tabs.remove(self.tabs.idx()),

            AppAction::TabRename(_idx, _new_name) => {
                todo!()
            }

            AppAction::TabSelectedPrev => self.tabs.select_prev(),

            AppAction::TabSelectedNext => self.tabs.select_next(),

            AppAction::TabRemoveOrQuit => {
                if self.tabs.len() == 1 {
                    self.quit()
                } else {
                    self.tabs.remove(self.tabs.idx())
                }
            }

            AppAction::Help => {
                let idx = self.tabs.iter().enumerate().find_map(|(idx, tab)| {
                    matches!(tab.tabular_type(), TabularType::Help).then_some(idx)
                });
                if let Some(idx) = idx {
                    self.tabs.select(idx)
                } else {
                    self.tabs.add(Tabular::new(
                        Commands::default().into_data_frame(),
                        TabularType::Help,
                    ))?;
                    self.tabs.select_last()
                }
            }

            AppAction::Quit => self.quit(),
        }
    }
}

impl Tabs {
    pub fn add(&mut self, tabular: Tabular) -> AppResult<()> {
        self.tabulars.push(tabular);
        Ok(())
    }

    pub fn len(&self) -> usize {
        self.tabulars.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn idx(&self) -> usize {
        self.idx
    }

    pub fn selected(&self) -> Option<&Tabular> {
        self.tabulars.get(self.idx)
    }

    pub fn selected_mut(&mut self) -> Option<&mut Tabular> {
        self.tabulars.get_mut(self.idx)
    }

    pub fn remove(&mut self, idx: usize) -> AppResult<()> {
        self.validate_index(idx)?;
        self.tabulars.remove(idx);
        self.saturating_select(self.idx.saturating_sub(1))
    }

    pub fn remove_selected(&mut self) -> AppResult<()> {
        self.remove(self.idx)
    }

    pub fn saturating_select(&mut self, idx: usize) -> AppResult<()> {
        self.idx = idx.min(self.tabulars.len().saturating_sub(1));
        Ok(())
    }

    pub fn select(&mut self, idx: usize) -> AppResult<()> {
        self.validate_index(idx)?;
        self.idx = idx;
        Ok(())
    }

    pub fn select_next(&mut self) -> AppResult<()> {
        self.saturating_select(self.idx.saturating_add(1))
    }

    pub fn select_prev(&mut self) -> AppResult<()> {
        self.saturating_select(self.idx.saturating_sub(1))
    }

    pub fn select_first(&mut self) -> AppResult<()> {
        self.saturating_select(0)
    }

    pub fn select_last(&mut self) -> AppResult<()> {
        self.saturating_select(usize::MAX)
    }

    fn validate_index(&self, idx: usize) -> AppResult<()> {
        if self.tabulars.is_empty() {
            Err("no tab is currently available".into())
        } else if idx < self.tabulars.len() {
            Ok(())
        } else {
            Err(format!(
                "invalid tab index, valid index range is between 0 and {}",
                self.tabulars.len()
            )
            .into())
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &Tabular> {
        self.tabulars.iter()
    }
}

impl FromIterator<Tabular> for Tabs {
    fn from_iter<T: IntoIterator<Item = Tabular>>(iter: T) -> Self {
        Self {
            tabulars: iter.into_iter().collect(),
            idx: 0,
        }
    }
}
