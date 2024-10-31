use std::{ops::Div, path::PathBuf};

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    layout::{Constraint, Layout},
    Frame,
};

use crate::{
    handler::{
        command::{commands_help_data_frame, parse_into_action},
        keybind::{Action, Keybind},
    },
    sql::SqlBackend,
    tui,
    writer::{JsonFormat, WriteToArrow, WriteToCsv, WriteToFile, WriteToJson, WriteToParquet},
    AppResult,
};

use tui::status_bar::{StatusBar, StatusBarState};
use tui::tabs::Tabs;
use tui::tabular::{self, Tabular, TabularType};
use tui::Styler;

pub struct App<Theme> {
    tabs: Tabs<Theme>,
    status_bar: StatusBar<Theme>,
    sql: SqlBackend,
    keybindings: Keybind,
    running: bool,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum AppState {
    Empty,
    Table,
    Sheet,
    Command,
    Error,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum AppAction {
    StatusBarStats,
    StatusBarCommand(String),
    StatausBarError(String),
    TabularTableView,
    TabularSheetView,
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
    ExportDsv {
        path: PathBuf,
        separator: char,
        quote: char,
        header: bool,
    },
    ExportParquet(PathBuf),
    ExportJson(PathBuf, JsonFormat),
    ExportArrow(PathBuf),
    Help,
    Quit,
}

impl<Theme: Styler> App<Theme> {
    pub fn new(tabs: Tabs<Theme>, sql: SqlBackend, key_bind: Keybind) -> Self {
        Self {
            tabs,
            status_bar: StatusBar::<Theme>::default(),
            sql,
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
            (None, StatusBarState::Info) => AppState::Empty,
            (None, StatusBarState::Error(_)) => AppState::Error,
            (None, StatusBarState::Prompt(_)) => AppState::Command,
        }
    }

    pub fn draw(&mut self, frame: &mut Frame) -> AppResult<()> {
        let layout =
            Layout::vertical([Constraint::Fill(1), Constraint::Length(1)]).split(frame.area());

        // Draw table / item
        let state = self.infer_state();
        if let Some(tab) = self.tabs.selected_mut() {
            tab.render(frame, layout[0], matches!(state, AppState::Table))?;
        }
        if let Some(tab) = self.tabs.selected() {
            self.status_bar.render(
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
                            width = self.tabs.len().to_string().len()
                        ),
                    ),
                    (
                        "Row",
                        &format!(
                            "{:>width$}",
                            tab.selected() + 1,
                            width = tab.data_frame().height().to_string().len()
                        ),
                    ),
                    (
                        "Shape",
                        &format!(
                            "{} x {}",
                            tab.data_frame().height(),
                            tab.data_frame().width()
                        ),
                    ),
                ],
            )
        } else {
            self.status_bar.render(frame, layout[1], &[])
        }
    }

    pub fn handle_key_event(&mut self, key_event: KeyEvent) -> AppResult<()> {
        let state = self.infer_state();
        let key_code = key_event.code;
        match (state, key_code) {
            (AppState::Command | AppState::Error, KeyCode::Esc) => self.status_bar.show_info(),

            (AppState::Command, KeyCode::Enter) => {
                if let Some(cmd) = self.status_bar.commit_prompt() {
                    let _ = parse_into_action(cmd)
                        .and_then(|action| self.invoke(action))
                        .and_then(|_| self.status_bar.show_info())
                        .inspect_err(|err| {
                            let _ = self.status_bar.show_error(err);
                        });
                    Ok(())
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

    pub fn invoke(&mut self, action: Action) -> AppResult<()> {
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
                        TabularType::Help => commands_help_data_frame(),
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

            AppAction::ExportDsv {
                path,
                separator,
                quote,
                header,
            } => {
                if let Some(tab) = self.tabs.selected_mut() {
                    WriteToCsv::default()
                        .with_separator_char(separator)
                        .with_quote_char(quote)
                        .with_header(header)
                        .write_to_file(path, tab.data_frame_mut())
                } else {
                    Err("Unable to export the data frame".into())
                }
            }

            AppAction::ExportParquet(path) => {
                if let Some(tab) = self.tabs.selected_mut() {
                    WriteToParquet.write_to_file(path, tab.data_frame_mut())
                } else {
                    Err("Unable to export the data frame".into())
                }
            }
            AppAction::ExportJson(path, fmt) => {
                if let Some(tab) = self.tabs.selected_mut() {
                    WriteToJson::default()
                        .with_format(fmt)
                        .write_to_file(path, tab.data_frame_mut())
                } else {
                    Err("Unable to export the data frame".into())
                }
            }
            AppAction::ExportArrow(path) => {
                if let Some(tab) = self.tabs.selected_mut() {
                    WriteToArrow.write_to_file(path, tab.data_frame_mut())
                } else {
                    Err("Unable to export the data frame".into())
                }
            }

            AppAction::Help => {
                let idx = self.tabs.iter().enumerate().find_map(|(idx, tab)| {
                    matches!(tab.tabular_type(), TabularType::Help).then_some(idx)
                });
                if let Some(idx) = idx {
                    self.tabs.select(idx)
                } else {
                    self.tabs
                        .add(Tabular::new(commands_help_data_frame(), TabularType::Help))?;
                    self.tabs.select_last()
                }
            }

            AppAction::Quit => self.quit(),
        }
    }
}
