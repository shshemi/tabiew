use std::{ops::Div, path::PathBuf};

use anyhow::{Ok, anyhow};

use crate::{
    AppResult,
    app::App,
    reader::{
        ArrowIpcToDataFrame, CsvToDataFrame, FwfToDataFrame, Input, JsonLineToDataFrame,
        JsonToDataFrame, ParquetToDataFrame, ReadToDataFrames, SqliteToDataFrames,
    },
    sql::SqlBackend,
    tui::{Source, TabContentState},
    writer::{JsonFormat, WriteToArrow, WriteToCsv, WriteToFile, WriteToJson, WriteToParquet},
};

use super::command::{commands_help_data_frame, parse_into_action};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum AppAction {
    NoAction,
    ToggleBorders,
    DismissError,
    DismissErrorAndShowPallete,

    TableDismissModal,
    TableScrollRight,
    TableScrollLeft,
    TableScrollRightColumn,
    TableScrollLeftColumn,
    TableScrollStart,
    TableScrollEnd,
    TableToggleExpansion,
    TableGoto(usize),
    TableGotoFirst,
    TableGotoLast,
    TableGotoRandom,
    TableGoUp(usize),
    TableGoUpHalfPage,
    TableGoUpFullPage,
    TableGoDown(usize),
    TableGoDownHalfPage,
    TableGoDownFullPage,

    SheetShow,
    SheetScrollUp,
    SheetScrollDown,

    TableSelect(String),
    TableOrder(String),
    TableFilter(String),
    SqlQuery(String),
    TableReset,
    SqlSchema,

    PalleteGotoNext,
    PalleteGotoPrev,
    PalleteGotoStart,
    PalleteGotoEnd,
    PalleteDeleteNext,
    PalleteDeletePrev,
    PalleteInsert(char),
    PalleteInsertSelectedOrCommit,
    PalleteShow(String),
    PalleteDeselectOrDismiss,
    PalleteSelectPrevious,
    PalleteSelectNext,

    SearchShow,
    SearchGotoNext,
    SearchGotoPrev,
    SearchGotoStart,
    SearchGotoEnd,
    SearchDeleteNext,
    SearchDeletePrev,
    SearchInsert(char),
    SearchRollback,
    SearchCommit,

    TabNew(String),
    TabSelect(usize),
    TabRemove(usize),
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
        AppAction::ToggleBorders => {
            app.toggle_borders();
            Ok(None)
        }
        AppAction::DismissErrorAndShowPallete => {
            app.dismiss_error();
            app.show_pallete("");
            Ok(None)
        }
        AppAction::TableDismissModal => {
            if let Some(tab) = app.tabs().selected_mut() {
                tab.table_mode()
            }
            Ok(None)
        }
        AppAction::SheetShow => {
            if let Some(tab) = app.tabs().selected_mut() {
                tab.sheet_mode()
            }
            Ok(None)
        }
        AppAction::SearchShow => {
            if let Some(tab) = app.tabs().selected_mut() {
                tab.search_mode();
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
            let idx =
                app.tabs().iter().enumerate().find_map(|(idx, tab)| {
                    matches!(tab.tabular_source(), Source::Help).then_some(idx)
                });
            if let Some(idx) = idx {
                Ok(Some(AppAction::TabSelect(idx)))
            } else {
                app.tabs()
                    .add(TabContentState::new(sql.schema(), Source::Schema));
                Ok(Some(AppAction::TabSelect(
                    app.tabs().len().saturating_sub(1),
                )))
            }
        }
        AppAction::TableGoto(line) => {
            if let Some(tab) = app.tabs().selected_mut() {
                tab.select(line)
            }
            Ok(None)
        }
        AppAction::TableGotoFirst => {
            if let Some(tab) = app.tabs().selected_mut() {
                tab.select_first()
            }
            Ok(None)
        }
        AppAction::TableGotoLast => {
            if let Some(tab) = app.tabs().selected_mut() {
                tab.select_last()
            }
            Ok(None)
        }
        AppAction::TableGotoRandom => {
            if let Some(tab) = app.tabs().selected_mut() {
                tab.select_random()
            }
            Ok(None)
        }
        AppAction::TableGoUp(lines) => {
            if let Some(tab) = app.tabs().selected_mut() {
                tab.select_up(lines)
            }
            Ok(None)
        }
        AppAction::TableGoUpHalfPage => {
            if let Some(tab) = app.tabs().selected_mut() {
                tab.select_up(tab.page_len().div(2))
            }
            Ok(None)
        }
        AppAction::TableGoUpFullPage => {
            if let Some(tab) = app.tabs().selected_mut() {
                tab.select_up(tab.page_len())
            }
            Ok(None)
        }
        AppAction::TableGoDown(lines) => {
            if let Some(tab) = app.tabs().selected_mut() {
                tab.select_down(lines)
            }
            Ok(None)
        }
        AppAction::TableGoDownHalfPage => {
            if let Some(tab) = app.tabs().selected_mut() {
                tab.select_down(tab.page_len().div(2))
            }
            Ok(None)
        }
        AppAction::TableGoDownFullPage => {
            if let Some(tab) = app.tabs().selected_mut() {
                tab.select_down(tab.page_len())
            }
            Ok(None)
        }
        AppAction::SheetScrollUp => {
            if let Some(tab) = app.tabs().selected_mut() {
                tab.sheet_scroll_up()
            }
            Ok(None)
        }
        AppAction::SheetScrollDown => {
            if let Some(tab) = app.tabs().selected_mut() {
                tab.sheet_scroll_down()
            }
            Ok(None)
        }
        AppAction::TableReset => {
            if let Some(tab) = app.tabs().selected_mut() {
                match tab.tabular_source() {
                    Source::Name(name) => {
                        tab.set_data_frame(sql.execute(&format!("SELECT * FROM '{}'", name))?);
                    }
                    Source::Query(query) => {
                        tab.set_data_frame(sql.execute(query)?);
                    }
                    _ => (),
                }
            }
            Ok(None)
        }
        AppAction::TableSelect(select) => {
            if let Some(tab) = app.tabs().selected_mut() {
                tab.set_data_frame(sql.execute(&format!("SELECT {} FROM _", select))?)
            }
            Ok(None)
        }
        AppAction::TableOrder(order) => {
            if let Some(tab) = app.tabs().selected_mut() {
                tab.set_data_frame(sql.execute(&format!("SELECT * FROM _ ORDER BY {}", order))?)
            }
            Ok(None)
        }
        AppAction::TableFilter(filter) => {
            if let Some(tab) = app.tabs().selected_mut() {
                tab.set_data_frame(sql.execute(&format!("SELECT * FROM _ where {}", filter))?)
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

            Ok(Some(AppAction::TabSelect(
                app.tabs().len().saturating_sub(1),
            )))
        }
        AppAction::TabSelect(idx) => {
            let idx = idx.min(app.tabs().len().saturating_sub(1));
            app.tabs().select(idx);
            sql.set_default(app.tabs().selected_data_frame().unwrap_or_default());
            Ok(None)
        }
        AppAction::TabRemove(idx) => {
            app.tabs().remove(idx);
            Ok(Some(AppAction::TabSelect(idx)))
        }
        AppAction::TabRename(_idx, _new_name) => {
            todo!()
        }
        AppAction::TabPrev => Ok(Some(AppAction::TabSelect(
            app.tabs().idx().saturating_sub(1),
        ))),
        AppAction::TabNext => Ok(Some(AppAction::TabSelect(
            app.tabs().idx().saturating_add(1),
        ))),
        AppAction::TabRemoveOrQuit => {
            if app.tabs().len() == 1 {
                app.quit();
                Ok(None)
            } else {
                let idx = app.tabs().idx();
                app.tabs().remove(idx);
                Ok(Some(AppAction::TabSelect(idx)))
            }
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
                let name = sql.register(&name, df.clone(), path.clone());
                app.tabs().add(TabContentState::new(df, Source::Name(name)));
            }
            Ok(None)
        }
        AppAction::ImportParquet(path) => {
            let frames = ParquetToDataFrame.named_frames(Input::File(path.clone()))?;
            for (name, df) in frames {
                let name = sql.register(&name, df.clone(), path.clone());
                app.tabs().add(TabContentState::new(df, Source::Name(name)));
            }
            Ok(Some(AppAction::TabSelect(
                app.tabs().len().saturating_sub(1),
            )))
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
                let name = sql.register(&name, df.clone(), path.clone());
                app.tabs().add(TabContentState::new(df, Source::Name(name)));
            }
            Ok(Some(AppAction::TabSelect(
                app.tabs().len().saturating_sub(1),
            )))
        }
        AppAction::ImportArrow(path) => {
            let frames = ArrowIpcToDataFrame.named_frames(Input::File(path.clone()))?;
            for (name, df) in frames {
                let name = sql.register(&name, df.clone(), path.clone());
                app.tabs().add(TabContentState::new(df, Source::Name(name)));
            }
            Ok(Some(AppAction::TabSelect(
                app.tabs().len().saturating_sub(1),
            )))
        }
        AppAction::ImportSqlite(path) => {
            let frames = SqliteToDataFrames.named_frames(Input::File(path.clone()))?;
            for (name, df) in frames {
                let name = sql.register(&name, df.clone(), path.clone());
                app.tabs().add(TabContentState::new(df, Source::Name(name)));
            }
            Ok(Some(AppAction::TabSelect(
                app.tabs().len().saturating_sub(1),
            )))
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
                let name = sql.register(&name, df.clone(), path.clone());
                app.tabs().add(TabContentState::new(df, Source::Name(name)));
            }
            Ok(Some(AppAction::TabSelect(
                app.tabs().len().saturating_sub(1),
            )))
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
                tab.search_rollback();
            }
            Ok(None)
        }
        AppAction::Help => {
            let idx =
                app.tabs().iter().enumerate().find_map(|(idx, tab)| {
                    matches!(tab.tabular_source(), Source::Help).then_some(idx)
                });
            if let Some(idx) = idx {
                Ok(Some(AppAction::TabSelect(idx)))
            } else {
                app.tabs().add(TabContentState::new(
                    commands_help_data_frame(),
                    Source::Help,
                ));
                Ok(Some(AppAction::TabSelect(
                    app.tabs().len().saturating_sub(1),
                )))
            }
        }
        AppAction::Quit => {
            app.quit();
            Ok(None)
        }
        AppAction::TableScrollRight => {
            if let Some(tab) = app.tabs().selected_mut() {
                tab.table_scroll_right();
            }
            Ok(None)
        }
        AppAction::TableScrollLeft => {
            if let Some(tab) = app.tabs().selected_mut() {
                tab.table_scroll_left();
            }
            Ok(None)
        }
        AppAction::TableScrollRightColumn => {
            if let Some(tab) = app.tabs().selected_mut() {
                tab.table_scroll_right_column();
            }
            Ok(None)
        }
        AppAction::TableScrollLeftColumn => {
            if let Some(tab) = app.tabs().selected_mut() {
                tab.table_scroll_left_column();
            }
            Ok(None)
        }
        AppAction::TableScrollStart => {
            if let Some(tab) = app.tabs().selected_mut() {
                tab.table_goto_start();
            }
            Ok(None)
        }
        AppAction::TableScrollEnd => {
            if let Some(tab) = app.tabs().selected_mut() {
                tab.table_goto_end();
            }
            Ok(None)
        }
        AppAction::TableToggleExpansion => {
            if let Some(tab) = app.tabs().selected_mut() {
                tab.toggle_expansion()?;
            }
            Ok(None)
        }
        AppAction::PalleteGotoNext => {
            if let Some(pallete) = app.pallete() {
                pallete.input().goto_next();
            }
            Ok(None)
        }
        AppAction::PalleteGotoPrev => {
            if let Some(pallete) = app.pallete() {
                pallete.input().goto_prev();
            }
            Ok(None)
        }
        AppAction::PalleteGotoStart => {
            if let Some(pallete) = app.pallete() {
                pallete.input().goto_start();
            }
            Ok(None)
        }
        AppAction::PalleteGotoEnd => {
            if let Some(pallete) = app.pallete() {
                pallete.input().goto_end();
            }
            Ok(None)
        }
        AppAction::PalleteDeleteNext => {
            if let Some(pallete) = app.pallete() {
                pallete.input().delete_next();
            }
            Ok(None)
        }
        AppAction::PalleteDeletePrev => {
            if let Some(pallete) = app.pallete() {
                pallete.input().delete_prev();
            }
            Ok(None)
        }
        AppAction::PalleteInsert(c) => {
            if let Some(pallete) = app.pallete() {
                pallete.input().insert(c);
                pallete.list().select(None);
            }
            Ok(None)
        }
        AppAction::PalleteInsertSelectedOrCommit => {
            if let Some(selected) = app.pallete().and_then(|pallete| pallete.list().selected()) {
                if let Some(cmd) = app.history().get(selected).map(String::to_owned) {
                    if let Some(pallete) = app.pallete() {
                        pallete.set_input(cmd);
                        pallete.list().select(None);
                    }
                }
                Ok(None)
            } else if let Some(cmd) = app.hide_pallete() {
                if cmd.is_empty() {
                    Ok(Some(AppAction::PalleteDeselectOrDismiss))
                } else {
                    app.history().push(cmd.clone());
                    parse_into_action(cmd).map(Some)
                }
            } else {
                Ok(None)
            }
        }
        AppAction::PalleteShow(text) => {
            app.show_pallete(text);
            Ok(None)
        }
        AppAction::PalleteDeselectOrDismiss => {
            if let Some(pallete) = app.pallete() {
                if pallete.list().selected().is_some() {
                    pallete.list().select(None);
                } else {
                    app.hide_pallete();
                }
            }
            Ok(None)
        }
        AppAction::PalleteSelectPrevious => {
            if let Some(pallete) = app.pallete() {
                pallete.list().select_previous();
            }
            Ok(None)
        }
        AppAction::PalleteSelectNext => {
            if let Some(pallete) = app.pallete() {
                pallete.list().select_next();
            }
            Ok(None)
        }
    }
}
