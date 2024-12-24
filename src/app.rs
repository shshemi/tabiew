use std::{ops::Div, path::PathBuf};

use crossterm::event::KeyEvent;
use ratatui::{
    layout::{Constraint, Layout},
    Frame,
};

use crate::{
    handler::{
        command::{commands_help_data_frame, parse_into_action},
        keybind::KeyMap,
    },
    reader::{
        ArrowIpcToDataFrame, CsvToDataFrame, Input, JsonLineToDataFrame, JsonToDataFrame,
        ParquetToDataFrame, FwfToDataFrame, ReadToDataFrames,
    },
    search::Search,
    sql::SqlBackend,
    tui::{
        self,
        status_bar::{StatusBar, StatusBarState, StatusBarTag},
        tabs::Tabs,
    },
    writer::{JsonFormat, WriteToArrow, WriteToCsv, WriteToFile, WriteToJson, WriteToParquet},
    AppResult,
};

use tui::status_bar::StatusBarView;
use tui::tabs::TabsState;
use tui::tabular::{self, TabularState, TabularType};
use tui::Styler;

pub struct App {
    tabs: TabsState,
    status_bar: StatusBarState,
    sql: SqlBackend,
    keybindings: KeyMap,
    running: bool,
    search: Option<Search>,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum AppState {
    Empty,
    Table,
    Sheet,
    Command,
    Error,
    Search,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum AppAction {
    StatusBarInfo,
    StatusBarCommand(String),
    StatausBarError(String),
    StatusBarSearch(String),
    StatusBarHandle(KeyEvent),
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
    TabularEnterPress,
    SheetScrollUp,
    SheetScrollDown,
    TabularReset,
    TabularSelect(String),
    TabularOrder(String),
    TabularFilter(String),
    SearchPattern(String),
    SearchRollback,
    SearchCommit,
    PromptCommit,
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
    ImportDsv {
        path: PathBuf,
        separator: char,
        has_header: bool,
        quote: char,
    },
    ImportParquet(PathBuf),
    ImportJson(PathBuf, JsonFormat),
    ImportArrow(PathBuf),
    ImportFwf {
        path: PathBuf,
        widths: Vec<usize>,
        separator_length: usize,
        flexible_width: bool,
        has_header: bool,
    },
    Help,
    Quit,
}

impl App {
    pub fn new(tabs: TabsState, sql: SqlBackend, key_bind: KeyMap) -> Self {
        Self {
            tabs,
            status_bar: StatusBarState::new(),
            sql,
            keybindings: key_bind,
            running: true,
            search: None,
        }
    }

    pub fn running(&self) -> bool {
        self.running
    }

    pub fn tick(&mut self) -> AppResult<()> {
        if let Some(ser) = &self.search {
            let _ = self
                .tabs
                .selected_mut()
                .unwrap()
                .set_data_frame(ser.latest());
        }

        self.tabs.selected_mut().map(|tab| tab.tick());
        self.status_bar.tick()
    }

    pub fn quit(&mut self) -> AppResult<()> {
        self.running = false;
        Ok(())
    }

    pub fn infer_state(&self) -> AppState {
        match (
            self.tabs.selected().map(TabularState::view),
            self.status_bar.view(),
        ) {
            (Some(tabular::TabularView::Table), StatusBarView::Info) => AppState::Table,
            (Some(tabular::TabularView::Table), StatusBarView::Error(_)) => AppState::Error,
            (Some(tabular::TabularView::Table), StatusBarView::Prompt(_)) => AppState::Command,
            (Some(tabular::TabularView::Table), StatusBarView::Search(_)) => AppState::Search,
            (Some(tabular::TabularView::Sheet(_)), StatusBarView::Info) => AppState::Sheet,
            (Some(tabular::TabularView::Sheet(_)), StatusBarView::Error(_)) => AppState::Error,
            (Some(tabular::TabularView::Sheet(_)), StatusBarView::Prompt(_)) => AppState::Command,
            (Some(tabular::TabularView::Sheet(_)), StatusBarView::Search(_)) => AppState::Sheet,
            (None, StatusBarView::Info) => AppState::Empty,
            (None, StatusBarView::Error(_)) => AppState::Error,
            (None, StatusBarView::Prompt(_)) => AppState::Command,
            (None, StatusBarView::Search(_)) => AppState::Empty,
        }
    }

    pub fn draw<Theme: Styler>(&mut self, frame: &mut Frame) -> AppResult<()> {
        let layout =
            Layout::vertical([Constraint::Fill(1), Constraint::Length(1)]).split(frame.area());

        // Draw table / item
        let state = self.infer_state();
        frame.render_stateful_widget(
            Tabs::<Theme>::new().selection(matches!(state, AppState::Table)),
            layout[0],
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
                layout[1],
                &mut self.status_bar,
            );
        } else {
            frame.render_stateful_widget(
                StatusBar::<Theme>::new(&[]),
                layout[1],
                &mut self.status_bar,
            );
        }
        Ok(())
    }

    pub fn handle_key_event(&mut self, key_event: KeyEvent) -> AppResult<()> {
        let state = self.infer_state();
        match self
            .keybindings
            .get(state, key_event)
            .map(|action| self.invoke(action))
        {
            Some(Ok(())) => Ok(()),
            Some(Err(msg)) => self.invoke(AppAction::StatausBarError(format!("Error: {}", msg))),
            None => Ok(()),
        }
    }

    pub fn invoke(&mut self, action: AppAction) -> AppResult<()> {
        match action {
            AppAction::StatusBarInfo => self.status_bar.switch_info(),

            AppAction::StatusBarCommand(prefix) => self.status_bar.switch_prompt(prefix),

            AppAction::StatausBarError(msg) => self.status_bar.switch_error(msg),

            AppAction::StatusBarSearch(query) => self.status_bar.switch_search(query),

            AppAction::StatusBarHandle(event) => match self.status_bar.view_mut() {
                StatusBarView::Prompt(prompt_state) => {
                    prompt_state.handle(event);
                    if prompt_state.command_len() == 0 {
                        self.invoke(AppAction::StatusBarInfo)
                    } else {
                        Ok(())
                    }
                }

                StatusBarView::Search(prompt_state) => {
                    prompt_state.handle(event);

                    if prompt_state.command_len() > 0 {
                        let pattern = prompt_state.skipped_line(1);
                        self.invoke(AppAction::SearchPattern(pattern))
                    } else {
                        self.invoke(AppAction::SearchRollback)
                    }
                }
                _ => Ok(()),
            },

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
                        .add(TabularState::new(self.sql.schema(), TabularType::Schema))?;
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
                            .execute(format!("SELECT * FROM '{}'", name).as_str())?,
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

            AppAction::TabularEnterPress => {
                if let Some(tab) = self.tabs.selected_mut() {
                    match tab.tabular_type() {
                        TabularType::Help
                        | TabularType::Schema
                        | TabularType::Name(_)
                        | TabularType::Query(_) => self.invoke(AppAction::TabularSheetView),
                    }
                } else {
                    Ok(())
                }
            }

            AppAction::SearchPattern(pattern) => {
                if !matches!(self.status_bar.view(), StatusBarView::Search(_)) {
                    let _ = self.status_bar.switch_search(&pattern);
                }
                if let Some(tab) = self.tabs.selected_mut() {
                    if let Some(search) = &self.search {
                        search.search(pattern);
                    } else {
                        let search = Search::new(tab.data_frame().clone());
                        search.search(pattern);
                        self.search = search.into();
                    }
                }
                Ok(())
            }

            AppAction::SearchRollback => {
                if let Some(df) = self.search.take().map(|ser| ser.into_original_data_frame()) {
                    if let Some(tab) = self.tabs.selected_mut() {
                        let _ = tab.set_data_frame(df);
                    }
                }
                self.status_bar.switch_info()
            }

            AppAction::SearchCommit => {
                if let Some(df) = self.search.take().map(|ser| ser.latest()) {
                    if let Some(tab) = self.tabs.selected_mut() {
                        let _ = tab.set_data_frame(df);
                    }
                }
                self.status_bar.switch_info()
            }

            AppAction::PromptCommit => {
                if let Some(cmd) = self.status_bar.commit_prompt() {
                    parse_into_action(cmd)
                        .and_then(|action| self.invoke(action))
                        .and_then(|_| self.status_bar.switch_info())
                } else {
                    Ok(())
                }
            }

            AppAction::TabNew(query) => {
                if self.sql.contains_dataframe(&query) {
                    let df = self.sql.execute(&format!("SELECT * FROM '{}'", query))?;
                    self.tabs
                        .add(TabularState::new(df, TabularType::Name(query)))?;
                } else {
                    let df = self.sql.execute(&query)?;
                    self.tabs
                        .add(TabularState::new(df, TabularType::Query(query)))?;
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

            AppAction::ImportDsv {
                path,
                separator,
                has_header,
                quote,
            } => {
                let frames = CsvToDataFrame::default()
                    .with_separator(separator)
                    .with_quote_char(quote)
                    .with_no_header(!has_header)
                    .named_frames(Input::File(path.clone()))?;
                for (name, df) in frames {
                    let name = name.unwrap_or(
                        path.clone()
                            .file_stem()
                            .expect("Invalid file name")
                            .to_string_lossy()
                            .into_owned(),
                    );
                    let name = self.sql.register(&name, df.clone(), path.clone());
                    self.tabs
                        .add(TabularState::new(df, TabularType::Name(name)))?;
                }
                self.tabs.select_last()
            }
            AppAction::ImportParquet(path) => {
                let frames = ParquetToDataFrame.named_frames(Input::File(path.clone()))?;
                for (name, df) in frames {
                    let name = name.unwrap_or(
                        path.clone()
                            .file_stem()
                            .expect("Invalid file name")
                            .to_string_lossy()
                            .into_owned(),
                    );
                    let name = self.sql.register(&name, df.clone(), path.clone());
                    self.tabs
                        .add(TabularState::new(df, TabularType::Name(name)))?;
                }
                self.tabs.select_last()
            }
            AppAction::ImportJson(path, json_format) => {
                let frames = match json_format {
                    JsonFormat::Json => {
                        JsonToDataFrame::default().named_frames(Input::File(path.clone()))?
                    }
                    JsonFormat::JsonLine => {
                        JsonLineToDataFrame::default().named_frames(Input::File(path.clone()))?
                    }
                };
                for (name, df) in frames {
                    let name = name.unwrap_or(
                        path.clone()
                            .file_stem()
                            .expect("Invalid file name")
                            .to_string_lossy()
                            .into_owned(),
                    );
                    let name = self.sql.register(&name, df.clone(), path.clone());
                    self.tabs
                        .add(TabularState::new(df, TabularType::Name(name)))?;
                }
                self.tabs.select_last()
            }
            AppAction::ImportArrow(path) => {
                let frames = ArrowIpcToDataFrame.named_frames(Input::File(path.clone()))?;
                for (name, df) in frames {
                    let name = name.unwrap_or(
                        path.clone()
                            .file_stem()
                            .expect("Invalid file name")
                            .to_string_lossy()
                            .into_owned(),
                    );
                    let name = self.sql.register(&name, df.clone(), path.clone());
                    self.tabs
                        .add(TabularState::new(df, TabularType::Name(name)))?;
                }
                self.tabs.select_last()
            }
            AppAction::ImportFwf {
                path,
                widths,
                separator_length,
                flexible_width,
                has_header,
            } => {
                let frames = FwfToDataFrame::default()
                    .with_widths(widths)
                    .with_separator_length(separator_length)
                    .with_flexible_width(flexible_width)
                    .with_has_header(has_header)
                    .named_frames(Input::File(path.clone()))?;
                for (name, df) in frames {
                    let name = name.unwrap_or(
                        path.clone()
                            .file_stem()
                            .expect("Invalid file name")
                            .to_string_lossy()
                            .into_owned(),
                    );
                    let name = self.sql.register(&name, df.clone(), path.clone());
                    self.tabs
                        .add(TabularState::new(df, TabularType::Name(name)))?;
                }
                self.tabs.select_last()
            }

            AppAction::Help => {
                let idx = self.tabs.iter().enumerate().find_map(|(idx, tab)| {
                    matches!(tab.tabular_type(), TabularType::Help).then_some(idx)
                });
                if let Some(idx) = idx {
                    self.tabs.select(idx)
                } else {
                    self.tabs.add(TabularState::new(
                        commands_help_data_frame(),
                        TabularType::Help,
                    ))?;
                    self.tabs.select_last()
                }
            }

            AppAction::Quit => self.quit(),
        }
    }
}
