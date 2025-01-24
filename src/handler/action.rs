use std::{ops::Div, path::PathBuf};

use anyhow::{anyhow, Ok};
use crossterm::event::KeyEvent;

use crate::{
    app::App,
    reader::{
        ArrowIpcToDataFrame, CsvToDataFrame, FwfToDataFrame, Input, JsonLineToDataFrame,
        JsonToDataFrame, ParquetToDataFrame, ReadToDataFrames, SqliteToDataFrames,
    },
    sql::SqlBackend,
    tui::{status_bar::StatusBarView, TabContentState, Source},
    writer::{JsonFormat, WriteToArrow, WriteToCsv, WriteToFile, WriteToJson, WriteToParquet},
    AppResult,
};

use super::command::{commands_help_data_frame, parse_into_action};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum AppAction {
    NoAction,
    DismissError,
    StatusBarCommand(String),
    StatusBarHandle(KeyEvent),
    TabularTableMode,
    TabularSheetMode,
    TabularSearchMode,
    TabularToggleSheetMode,
    TabularScrollRight,
    TabularScrollLeft,
    TabularScrollStart,
    TabularScrollEnd,
    TabularToggleExpansion,
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
    TabularSelect(String),
    TabularOrder(String),
    TabularFilter(String),
    SqlQuery(String),
    TabularReset,
    SqlSchema,
    SearchGotoNext,
    SearchGotoPrev,
    SearchGotoStart,
    SearchGotoEnd,
    SearchDeleteNext,
    SearchDeletePrev,
    SearchInsert(char),
    SearchRollback,
    SearchCommit,
    PromptCommit,
    TabNew(String),
    TabSelect(usize),
    TabRemove(usize),
    TabRemoveSelected,
    TabPrev,
    TabNext,
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
    ImportSqlite(PathBuf),
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

pub fn execute(
    action: AppAction,
    app: &mut App,
    sql: &mut SqlBackend,
) -> AppResult<Option<AppAction>> {
    match action {
        AppAction::NoAction => Ok(None),
        AppAction::DismissError => {
            app.dismiss_error();
            Ok(None)
        }

        AppAction::StatusBarCommand(prefix) => {
            app.status_bar().switch_prompt(prefix);
            Ok(None)
        }

        AppAction::StatusBarHandle(event) => match app.status_bar().view_mut() {
            StatusBarView::Prompt(prompt_state) => {
                prompt_state.handle(event);
                if prompt_state.command_len() == 0 {
                    Ok(Some(AppAction::DismissError))
                } else {
                    Ok(None)
                }
            }
            _ => Ok(None),
        },

        AppAction::TabularTableMode => {
            if let Some(tab) = app.tabs().selected_mut() {
                tab.table_mode()
            }
            Ok(None)
        }

        AppAction::TabularSheetMode => {
            if let Some(tab) = app.tabs().selected_mut() {
                tab.sheet_mode()
            }
            Ok(None)
        }

        AppAction::TabularSearchMode => {
            if let Some(tab) = app.tabs().selected_mut() {
                tab.search_mode();
            }
            Ok(None)
        }

        AppAction::TabularToggleSheetMode => {
            if let Some(tab) = app.tabs().selected_mut() {
                tab.switch_view()
            }
            Ok(None)
        }

        AppAction::SqlQuery(query) => {
            if let Some(tab) = app.tabs().selected_mut() {
                tab.set_data_frame(sql.execute(&query)?)
            }
            Ok(None)
        }

        AppAction::SqlSchema => {
            let idx = app.tabs().iter().enumerate().find_map(|(idx, tab)| {
                matches!(tab.tabular_source(), Source::Help).then_some(idx)
            });
            if let Some(idx) = idx {
                app.tabs().select(idx);
                Ok(None)
            } else {
                app.tabs()
                    .add(TabContentState::new(sql.schema(), Source::Schema));
                app.tabs().select_last();
                Ok(None)
            }
        }

        AppAction::TabularGoto(line) => {
            if let Some(tab) = app.tabs().selected_mut() {
                tab.select(line)
            }
            Ok(None)
        }

        AppAction::TabularGotoFirst => {
            if let Some(tab) = app.tabs().selected_mut() {
                tab.select_first()
            }
            Ok(None)
        }

        AppAction::TabularGotoLast => {
            if let Some(tab) = app.tabs().selected_mut() {
                tab.select_last()
            }
            Ok(None)
        }

        AppAction::TabularGotoRandom => {
            if let Some(tab) = app.tabs().selected_mut() {
                tab.select_random()
            }
            Ok(None)
        }

        AppAction::TabularGoUp(lines) => {
            if let Some(tab) = app.tabs().selected_mut() {
                tab.select_up(lines)
            }
            Ok(None)
        }

        AppAction::TabularGoUpHalfPage => {
            if let Some(tab) = app.tabs().selected_mut() {
                tab.select_up(tab.page_len().div(2))
            }
            Ok(None)
        }

        AppAction::TabularGoUpFullPage => {
            if let Some(tab) = app.tabs().selected_mut() {
                tab.select_up(tab.page_len())
            }
            Ok(None)
        }

        AppAction::TabularGoDown(lines) => {
            if let Some(tab) = app.tabs().selected_mut() {
                tab.select_down(lines)
            }
            Ok(None)
        }

        AppAction::TabularGoDownHalfPage => {
            if let Some(tab) = app.tabs().selected_mut() {
                tab.select_down(tab.page_len().div(2))
            }
            Ok(None)
        }

        AppAction::TabularGoDownFullPage => {
            if let Some(tab) = app.tabs().selected_mut() {
                tab.select_down(tab.page_len())
            }
            Ok(None)
        }

        AppAction::SheetScrollUp => {
            if let Some(tab) = app.tabs().selected_mut() {
                tab.scroll_up()
            }
            Ok(None)
        }

        AppAction::SheetScrollDown => {
            if let Some(tab) = app.tabs().selected_mut() {
                tab.scroll_down()
            }
            Ok(None)
        }

        AppAction::TabularReset => {
            if let Some(tab) = app.tabs().selected_mut() {
                tab.rollback();
            }
            Ok(None)
        }

        AppAction::TabularSelect(select) => {
            if let Some(tab) = app.tabs().selected_mut() {
                let mut sql = SqlBackend::new();
                sql.register("df", tab.data_frame().clone(), "".into());
                tab.set_data_frame(sql.execute(&format!("SELECT {} FROM df", select))?)
            }
            Ok(None)
        }

        AppAction::TabularOrder(order) => {
            if let Some(tab) = app.tabs().selected_mut() {
                let mut sql = SqlBackend::new();
                sql.register("df", tab.data_frame().clone(), "".into());
                tab.set_data_frame(sql.execute(&format!("SELECT * FROM df ORDER BY {}", order))?)
            }
            Ok(None)
        }

        AppAction::TabularFilter(filter) => {
            if let Some(tab) = app.tabs().selected_mut() {
                let mut sql = SqlBackend::new();
                sql.register("df", tab.data_frame().clone(), "".into());
                tab.set_data_frame(sql.execute(&format!("SELECT * FROM df where {}", filter))?)
            }
            Ok(None)
        }

        AppAction::SearchCommit => {
            if let Some(tab) = app.tabs().selected_mut() {
                tab.search_commit();
                tab.table_mode();
            }
            Ok(None)
        }

        AppAction::PromptCommit => {
            if let Some(cmd) = app.status_bar().commit_prompt() {
                app.status_bar().switch_info();
                parse_into_action(cmd).map(Some)
            } else {
                Ok(None)
            }
        }

        AppAction::TabNew(query) => {
            if sql.contains_dataframe(&query) {
                let df = sql.execute(&format!("SELECT * FROM '{}'", query))?;
                app.tabs()
                    .add(TabContentState::new(df, Source::Name(query)));
            } else {
                let df = sql.execute(&query)?;
                app.tabs()
                    .add(TabContentState::new(df, Source::Query(query)));
            }
            app.tabs().select_last();
            Ok(None)
        }

        AppAction::TabSelect(idx) => {
            if idx < app.tabs().len() {
                app.tabs().select(idx);
                Ok(None)
            } else {
                Err(anyhow!(
                    "index {} is out of bound, maximum is {}",
                    idx,
                    app.tabs().len()
                ))
            }
        }

        AppAction::TabRemove(idx) => {
            app.tabs().remove(idx);
            Ok(None)
        }

        AppAction::TabRemoveSelected => {
            let idx = app.tabs().idx();
            app.tabs().remove(idx);
            Ok(None)
        }

        AppAction::TabRename(_idx, _new_name) => {
            todo!()
        }

        AppAction::TabPrev => {
            app.tabs().select_prev();
            Ok(None)
        }

        AppAction::TabNext => {
            app.tabs().select_next();
            Ok(None)
        }

        AppAction::TabRemoveOrQuit => {
            if app.tabs().len() == 1 {
                app.quit();
            } else {
                let idx = app.tabs().idx();
                app.tabs().remove(idx);
            }
            Ok(None)
        }

        AppAction::ExportDsv {
            path,
            separator,
            quote,
            header,
        } => {
            if let Some(tab) = app.tabs().selected_mut() {
                WriteToCsv::default()
                    .with_separator_char(separator)
                    .with_quote_char(quote)
                    .with_header(header)
                    .write_to_file(path, tab.data_frame_mut())?;
                Ok(None)
            } else {
                Err(anyhow!("Unable to export the data frame"))
            }
        }

        AppAction::ExportParquet(path) => {
            if let Some(tab) = app.tabs().selected_mut() {
                WriteToParquet.write_to_file(path, tab.data_frame_mut())?;
                Ok(None)
            } else {
                Err(anyhow!("Unable to export the data frame"))
            }
        }

        AppAction::ExportJson(path, fmt) => {
            if let Some(tab) = app.tabs().selected_mut() {
                WriteToJson::default()
                    .with_format(fmt)
                    .write_to_file(path, tab.data_frame_mut())?;
                Ok(None)
            } else {
                Err(anyhow!("Unable to export the data frame"))
            }
        }

        AppAction::ExportArrow(path) => {
            if let Some(tab) = app.tabs().selected_mut() {
                WriteToArrow.write_to_file(path, tab.data_frame_mut())?;
                Ok(None)
            } else {
                Err(anyhow!("Unable to export the data frame"))
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
                let name = sql.register(&name, df.clone(), path.clone());
                app.tabs()
                    .add(TabContentState::new(df, Source::Name(name)));
            }
            Ok(None)
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
                let name = sql.register(&name, df.clone(), path.clone());
                app.tabs()
                    .add(TabContentState::new(df, Source::Name(name)));
            }
            app.tabs().select_last();
            Ok(None)
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
                let name = sql.register(&name, df.clone(), path.clone());
                app.tabs()
                    .add(TabContentState::new(df, Source::Name(name)));
            }
            app.tabs().select_last();
            Ok(None)
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
                let name = sql.register(&name, df.clone(), path.clone());
                app.tabs()
                    .add(TabContentState::new(df, Source::Name(name)));
            }
            app.tabs().select_last();
            Ok(None)
        }

        AppAction::ImportSqlite(path) => {
            let frames = SqliteToDataFrames.named_frames(Input::File(path.clone()))?;
            for (name, df) in frames {
                let name = name.unwrap_or(
                    path.clone()
                        .file_stem()
                        .expect("Invalid file name")
                        .to_string_lossy()
                        .into_owned(),
                );
                let name = sql.register(&name, df.clone(), path.clone());
                app.tabs()
                    .add(TabContentState::new(df, Source::Name(name)));
            }
            app.tabs().select_last();
            Ok(None)
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
                let name = sql.register(&name, df.clone(), path.clone());
                app.tabs()
                    .add(TabContentState::new(df, Source::Name(name)));
            }
            app.tabs().select_last();
            Ok(None)
        }

        AppAction::SearchGotoNext => {
            if let Some(tab) = app.tabs().selected_mut() {
                tab.search_goto_next();
            }
            Ok(None)
        }

        AppAction::SearchGotoPrev => {
            if let Some(tab) = app.tabs().selected_mut() {
                tab.search_goto_prev();
            }
            Ok(None)
        }

        AppAction::SearchGotoStart => {
            if let Some(tab) = app.tabs().selected_mut() {
                tab.search_goto_start();
            }
            Ok(None)
        }

        AppAction::SearchGotoEnd => {
            if let Some(tab) = app.tabs().selected_mut() {
                tab.search_goto_end();
            }
            Ok(None)
        }

        AppAction::SearchDeleteNext => {
            if let Some(tab) = app.tabs().selected_mut() {
                tab.search_delete_next();
            }
            Ok(None)
        }

        AppAction::SearchDeletePrev => {
            if let Some(tab) = app.tabs().selected_mut() {
                tab.search_delete_prev();
            }
            Ok(None)
        }

        AppAction::SearchInsert(c) => {
            if let Some(tab) = app.tabs().selected_mut() {
                tab.search_insert(c);
            }
            Ok(None)
        }

        AppAction::SearchRollback => {
            if let Some(tab) = app.tabs().selected_mut() {
                tab.table_mode();
                tab.rollback();
            }
            app.status_bar().switch_info();
            Ok(None)
        }

        AppAction::Help => {
            let idx = app.tabs().iter().enumerate().find_map(|(idx, tab)| {
                matches!(tab.tabular_source(), Source::Help).then_some(idx)
            });
            if let Some(idx) = idx {
                app.tabs().select(idx)
            } else {
                app.tabs().add(TabContentState::new(
                    commands_help_data_frame(),
                    Source::Help,
                ));
                app.tabs().select_last();
            }
            Ok(None)
        }

        AppAction::Quit => {
            app.quit();
            Ok(None)
        }

        AppAction::TabularScrollRight => {
            if let Some(tab) = app.tabs().selected_mut() {
                tab.scroll_right();
            }
            Ok(None)
        }

        AppAction::TabularScrollLeft => {
            if let Some(tab) = app.tabs().selected_mut() {
                tab.scroll_left();
            }
            Ok(None)
        }

        AppAction::TabularScrollStart => {
            if let Some(tab) = app.tabs().selected_mut() {
                tab.scroll_start();
            }
            Ok(None)
        }

        AppAction::TabularScrollEnd => {
            if let Some(tab) = app.tabs().selected_mut() {
                tab.scroll_end();
            }
            Ok(None)
        }

        AppAction::TabularToggleExpansion => {
            if let Some(tab) = app.tabs().selected_mut() {
                tab.toggle_expansion();
            }
            Ok(None)
        }
    }
}
