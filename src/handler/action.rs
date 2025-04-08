use std::{ops::Div, path::PathBuf};

use anyhow::{Ok, anyhow};
use rand::Rng;

use crate::{
    AppResult,
    app::App,
    misc::globals::sql,
    reader::{
        ArrowIpcToDataFrame, CsvToDataFrame, FwfToDataFrame, InputSource, JsonLineToDataFrame,
        JsonToDataFrame, ParquetToDataFrame, ReadToDataFrames, SqliteToDataFrames,
    },
    tui::{
        TableType, TabularState, schema::SchemaState, search_bar::SearchBarState, tab::Content,
        tabular::Modal,
    },
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

pub fn execute(action: AppAction, app: &mut App) -> AppResult<Option<AppAction>> {
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
            if let Some(tab) = app.tabs_mut().selected_mut().and_then(Content::tabular_mut) {
                tab.modal_take();
            }
            Ok(None)
        }
        AppAction::SheetShow => {
            if let Some(tab) = app.tabs_mut().selected_mut().and_then(Content::tabular_mut) {
                tab.show_sheet()
            }
            Ok(None)
        }
        AppAction::SearchShow => {
            if let Some(tab) = app.tabs_mut().selected_mut().and_then(Content::tabular_mut) {
                tab.show_search();
            }
            Ok(None)
        }
        AppAction::SqlQuery(query) => {
            if let Some(tab) = app.tabs_mut().selected_mut().and_then(Content::tabular_mut) {
                tab.table_mut().set_data_frame(sql().execute(&query)?)
            }
            Ok(None)
        }
        AppAction::SqlSchema => {
            let idx = app
                .tabs_mut()
                .iter()
                .enumerate()
                .find_map(|(idx, tab)| match tab {
                    Content::Tabular(tab) if matches!(tab.table_type(), TableType::Help) => {
                        Some(idx)
                    }
                    _ => None,
                });
            if let Some(idx) = idx {
                Ok(Some(AppAction::TabSelect(idx)))
            } else {
                app.tabs_mut().add(SchemaState::default());
                Ok(Some(AppAction::TabSelect(
                    app.tabs_mut().len().saturating_sub(1),
                )))
            }
        }
        AppAction::TableGoto(line) => {
            if let Some(tab) = app.tabs_mut().selected_mut().and_then(Content::tabular_mut) {
                tab.table_mut().select(line)
            }
            Ok(None)
        }
        AppAction::TableGotoFirst => {
            if let Some(tab) = app.tabs_mut().selected_mut().and_then(Content::tabular_mut) {
                tab.table_mut().select_first()
            }
            Ok(None)
        }
        AppAction::TableGotoLast => {
            if let Some(tab) = app.tabs_mut().selected_mut().and_then(Content::tabular_mut) {
                tab.table_mut().select_last()
            }
            Ok(None)
        }
        AppAction::TableGotoRandom => {
            if let Some(tab) = app.tabs_mut().selected_mut().and_then(Content::tabular_mut) {
                let random_row = rand::rng().random_range(0..tab.table().data_frame().height());
                tab.table_mut().select(random_row);
            }
            Ok(None)
        }
        AppAction::TableGoUp(lines) => {
            if let Some(tab) = app.tabs_mut().selected_mut().and_then(Content::tabular_mut) {
                tab.table_mut().select_up(lines)
            }
            Ok(None)
        }
        AppAction::TableGoUpHalfPage => {
            if let Some(tab) = app.tabs_mut().selected_mut().and_then(Content::tabular_mut) {
                let len = tab.table().rendered_rows().div(2).into();
                tab.table_mut().select_up(len)
            }
            Ok(None)
        }
        AppAction::TableGoUpFullPage => {
            if let Some(tab) = app.tabs_mut().selected_mut().and_then(Content::tabular_mut) {
                let len = tab.table().rendered_rows().into();
                tab.table_mut().select_up(len)
            }
            Ok(None)
        }
        AppAction::TableGoDown(lines) => {
            if let Some(tab) = app.tabs_mut().selected_mut().and_then(Content::tabular_mut) {
                tab.table_mut().select_down(lines)
            }
            Ok(None)
        }
        AppAction::TableGoDownHalfPage => {
            if let Some(tab) = app.tabs_mut().selected_mut().and_then(Content::tabular_mut) {
                let len = tab.table().rendered_rows().div(2).into();
                tab.table_mut().select_down(len)
            }
            Ok(None)
        }
        AppAction::TableGoDownFullPage => {
            if let Some(tab) = app.tabs_mut().selected_mut().and_then(Content::tabular_mut) {
                let len = tab.table().rendered_rows().into();
                tab.table_mut().select_down(len)
            }
            Ok(None)
        }
        AppAction::SheetScrollUp => {
            if let Some(sheet) = app
                .tabs_mut()
                .selected_mut()
                .and_then(Content::tabular_mut)
                .map(TabularState::modal_mut)
                .and_then(Modal::sheet_mut)
            {
                sheet.scroll_up();
            }
            Ok(None)
        }
        AppAction::SheetScrollDown => {
            if let Some(sheet) = app
                .tabs_mut()
                .selected_mut()
                .and_then(Content::tabular_mut)
                .map(TabularState::modal_mut)
                .and_then(Modal::sheet_mut)
            {
                sheet.scroll_down();
            }
            Ok(None)
        }
        AppAction::TableReset => {
            if let Some(tab) = app.tabs_mut().selected_mut().and_then(Content::tabular_mut) {
                let df = match tab.table_type() {
                    TableType::Name(name) => {
                        Some(sql().execute(&format!("SELECT * FROM '{}'", name))?)
                    }
                    TableType::Query(query) => Some(sql().execute(query)?),
                    _ => None,
                };
                if let Some(df) = df {
                    tab.table_mut().set_data_frame(df);
                }
            }
            Ok(None)
        }
        AppAction::TableSelect(select) => {
            if let Some(tab) = app.tabs_mut().selected_mut().and_then(Content::tabular_mut) {
                tab.table_mut()
                    .set_data_frame(sql().execute(&format!("SELECT {} FROM _", select))?)
            }
            Ok(None)
        }
        AppAction::TableOrder(order) => {
            if let Some(tab) = app.tabs_mut().selected_mut().and_then(Content::tabular_mut) {
                tab.table_mut()
                    .set_data_frame(sql().execute(&format!("SELECT * FROM _ ORDER BY {}", order))?)
            }
            Ok(None)
        }
        AppAction::TableFilter(filter) => {
            if let Some(tab) = app.tabs_mut().selected_mut().and_then(Content::tabular_mut) {
                tab.table_mut()
                    .set_data_frame(sql().execute(&format!("SELECT * FROM _ where {}", filter))?)
            }
            Ok(None)
        }
        AppAction::TabNew(query) => {
            if let Result::Ok(df) = sql().execute(&format!("SELECT * FROM '{}'", query)) {
                app.tabs_mut()
                    .add(TabularState::new(df, TableType::Name(query)));
            } else {
                let df = sql().execute(&query)?;
                app.tabs_mut()
                    .add(TabularState::new(df, TableType::Query(query)));
            }

            Ok(Some(AppAction::TabSelect(
                app.tabs_mut().len().saturating_sub(1),
            )))
        }
        AppAction::TabSelect(idx) => {
            let idx = idx.min(app.tabs_mut().len().saturating_sub(1));
            app.tabs_mut().select(idx);

            if let Some(Content::Tabular(tabular)) = app.tabs_mut().selected_mut() {
                sql().set_default(tabular.table_mut().data_frame().clone());
            } else {
                sql().unset_default();
            }
            Ok(None)
        }
        AppAction::TabRemove(idx) => {
            app.tabs_mut().remove(idx);
            Ok(Some(AppAction::TabSelect(idx)))
        }
        AppAction::TabRename(_idx, _new_name) => {
            todo!()
        }
        AppAction::TabPrev => Ok(Some(AppAction::TabSelect(
            app.tabs_mut().idx().saturating_sub(1),
        ))),
        AppAction::TabNext => Ok(Some(AppAction::TabSelect(
            app.tabs_mut().idx().saturating_add(1),
        ))),
        AppAction::TabRemoveOrQuit => {
            if app.tabs_mut().len() == 1 {
                app.quit();
                Ok(None)
            } else {
                let idx = app.tabs_mut().idx();
                app.tabs_mut().remove(idx);
                Ok(Some(AppAction::TabSelect(idx)))
            }
        }
        AppAction::ExportDsv {
            path,
            separator,
            quote,
            header,
        } => {
            if let Some(tab) = app.tabs_mut().selected_mut().and_then(Content::tabular_mut) {
                WriteToCsv::default()
                    .with_separator_char(separator)
                    .with_quote_char(quote)
                    .with_header(header)
                    .write_to_file(path, tab.table_mut().data_frame_mut())?;
                Ok(None)
            } else {
                Err(anyhow!("Unable to export the data frame"))
            }
        }
        AppAction::ExportParquet(path) => {
            if let Some(tab) = app.tabs_mut().selected_mut().and_then(Content::tabular_mut) {
                WriteToParquet.write_to_file(path, tab.table_mut().data_frame_mut())?;
                Ok(None)
            } else {
                Err(anyhow!("Unable to export the data frame"))
            }
        }
        AppAction::ExportJson(path, fmt) => {
            if let Some(tab) = app.tabs_mut().selected_mut().and_then(Content::tabular_mut) {
                WriteToJson::default()
                    .with_format(fmt)
                    .write_to_file(path, tab.table_mut().data_frame_mut())?;
                Ok(None)
            } else {
                Err(anyhow!("Unable to export the data frame"))
            }
        }
        AppAction::ExportArrow(path) => {
            if let Some(tab) = app.tabs_mut().selected_mut().and_then(Content::tabular_mut) {
                WriteToArrow.write_to_file(path, tab.table_mut().data_frame_mut())?;
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
            let source = InputSource::File(path.clone());
            let frames = CsvToDataFrame::default()
                .with_separator(separator)
                .with_quote_char(quote)
                .with_no_header(!has_header)
                .named_frames(source.clone())?;
            for (name, df) in frames {
                let name = sql().register(&name, df.clone(), source.clone());
                app.tabs_mut()
                    .add(TabularState::new(df, TableType::Name(name)));
            }
            Ok(None)
        }
        AppAction::ImportParquet(path) => {
            let source = InputSource::File(path);
            let frames = ParquetToDataFrame.named_frames(source.clone())?;
            for (name, df) in frames {
                let name = sql().register(&name, df.clone(), source.clone());
                app.tabs_mut()
                    .add(TabularState::new(df, TableType::Name(name)));
            }
            Ok(Some(AppAction::TabSelect(
                app.tabs_mut().len().saturating_sub(1),
            )))
        }
        AppAction::ImportJson(path, json_format) => {
            let source = InputSource::File(path);
            let frames = match json_format {
                JsonFormat::Json => JsonToDataFrame::default().named_frames(source.clone())?,
                JsonFormat::JsonLine => {
                    JsonLineToDataFrame::default().named_frames(source.clone())?
                }
            };
            for (name, df) in frames {
                let name = sql().register(&name, df.clone(), source.clone());
                app.tabs_mut()
                    .add(TabularState::new(df, TableType::Name(name)));
            }
            Ok(Some(AppAction::TabSelect(
                app.tabs_mut().len().saturating_sub(1),
            )))
        }
        AppAction::ImportArrow(path) => {
            let source = InputSource::File(path.clone());
            let frames = ArrowIpcToDataFrame.named_frames(source.clone())?;
            for (name, df) in frames {
                let name = sql().register(&name, df.clone(), source.clone());
                app.tabs_mut()
                    .add(TabularState::new(df, TableType::Name(name)));
            }
            Ok(Some(AppAction::TabSelect(
                app.tabs_mut().len().saturating_sub(1),
            )))
        }
        AppAction::ImportSqlite(path) => {
            let source = InputSource::File(path.clone());
            let frames = SqliteToDataFrames.named_frames(source.clone())?;
            for (name, df) in frames {
                let name = sql().register(&name, df.clone(), source.clone());
                app.tabs_mut()
                    .add(TabularState::new(df, TableType::Name(name)));
            }
            Ok(Some(AppAction::TabSelect(
                app.tabs_mut().len().saturating_sub(1),
            )))
        }
        AppAction::ImportFwf {
            path,
            widths,
            separator_length,
            flexible_width,
            has_header,
        } => {
            let source = InputSource::File(path.clone());
            let frames = FwfToDataFrame::default()
                .with_widths(widths)
                .with_separator_length(separator_length)
                .with_flexible_width(flexible_width)
                .with_has_header(has_header)
                .named_frames(source.clone())?;
            for (name, df) in frames {
                let name = sql().register(&name, df.clone(), source.clone());
                app.tabs_mut()
                    .add(TabularState::new(df, TableType::Name(name)));
            }
            Ok(Some(AppAction::TabSelect(
                app.tabs_mut().len().saturating_sub(1),
            )))
        }
        AppAction::SearchGotoNext => {
            if let Some(sb) = app
                .tabs_mut()
                .selected_mut()
                .and_then(Content::tabular_mut)
                .map(TabularState::modal_mut)
                .and_then(Modal::search_bar_mut)
            {
                sb.goto_next();
            }
            Ok(None)
        }
        AppAction::SearchGotoPrev => {
            if let Some(sb) = app
                .tabs_mut()
                .selected_mut()
                .and_then(Content::tabular_mut)
                .map(TabularState::modal_mut)
                .and_then(Modal::search_bar_mut)
            {
                sb.goto_prev();
            }
            Ok(None)
        }
        AppAction::SearchGotoStart => {
            if let Some(sb) = app
                .tabs_mut()
                .selected_mut()
                .and_then(Content::tabular_mut)
                .map(TabularState::modal_mut)
                .and_then(Modal::search_bar_mut)
            {
                sb.goto_start();
            }
            Ok(None)
        }
        AppAction::SearchGotoEnd => {
            if let Some(sb) = app
                .tabs_mut()
                .selected_mut()
                .and_then(Content::tabular_mut)
                .map(TabularState::modal_mut)
                .and_then(Modal::search_bar_mut)
            {
                sb.goto_end();
            }
            Ok(None)
        }
        AppAction::SearchDeleteNext => {
            if let Some(sb) = app
                .tabs_mut()
                .selected_mut()
                .and_then(Content::tabular_mut)
                .map(TabularState::modal_mut)
                .and_then(Modal::search_bar_mut)
            {
                sb.delete_next();
            }
            Ok(None)
        }
        AppAction::SearchDeletePrev => {
            if let Some(sb) = app
                .tabs_mut()
                .selected_mut()
                .and_then(Content::tabular_mut)
                .map(TabularState::modal_mut)
                .and_then(Modal::search_bar_mut)
            {
                sb.delete_prev();
            }
            Ok(None)
        }
        AppAction::SearchInsert(c) => {
            if let Some(sb) = app
                .tabs_mut()
                .selected_mut()
                .and_then(Content::tabular_mut)
                .map(TabularState::modal_mut)
                .and_then(Modal::search_bar_mut)
            {
                sb.insert(c);
            }
            Ok(None)
        }
        AppAction::SearchCommit => {
            if let Some(tab) = app.tabs_mut().selected_mut().and_then(Content::tabular_mut) {
                if let Some(df) = tab
                    .modal_take()
                    .into_search_bar()
                    .and_then(|sb| sb.search().latest())
                {
                    tab.table_mut().set_data_frame(df);
                }
            }
            Ok(None)
        }
        AppAction::SearchRollback => {
            if let Some(tab) = app.tabs_mut().selected_mut().and_then(Content::tabular_mut) {
                if let Some(df) = tab
                    .modal_take()
                    .into_search_bar()
                    .map(SearchBarState::into_rollback_df)
                {
                    tab.table_mut().set_data_frame(df);
                }
            }
            Ok(None)
        }
        AppAction::Help => {
            let idx = app
                .tabs_mut()
                .iter()
                .enumerate()
                .find_map(|(idx, tab)| match tab {
                    Content::Tabular(tab) if matches!(tab.table_type(), TableType::Help) => {
                        Some(idx)
                    }
                    _ => None,
                });
            if let Some(idx) = idx {
                Ok(Some(AppAction::TabSelect(idx)))
            } else {
                app.tabs_mut().add(TabularState::new(
                    commands_help_data_frame(),
                    TableType::Help,
                ));
                Ok(Some(AppAction::TabSelect(
                    app.tabs_mut().len().saturating_sub(1),
                )))
            }
        }
        AppAction::Quit => {
            app.quit();
            Ok(None)
        }
        AppAction::TableScrollRight => {
            if let Some(tab) = app.tabs_mut().selected_mut().and_then(Content::tabular_mut) {
                tab.table_mut().scroll_right();
            }
            Ok(None)
        }
        AppAction::TableScrollLeft => {
            if let Some(tab) = app.tabs_mut().selected_mut().and_then(Content::tabular_mut) {
                tab.table_mut().scroll_left();
            }
            Ok(None)
        }
        AppAction::TableScrollRightColumn => {
            if let Some(tab) = app.tabs_mut().selected_mut().and_then(Content::tabular_mut) {
                tab.table_mut().scroll_right();
            }
            Ok(None)
        }
        AppAction::TableScrollLeftColumn => {
            if let Some(tab) = app.tabs_mut().selected_mut().and_then(Content::tabular_mut) {
                tab.table_mut().scroll_left();
            }
            Ok(None)
        }
        AppAction::TableScrollStart => {
            if let Some(tab) = app.tabs_mut().selected_mut().and_then(Content::tabular_mut) {
                tab.table_mut().scroll_start();
            }
            Ok(None)
        }
        AppAction::TableScrollEnd => {
            if let Some(tab) = app.tabs_mut().selected_mut().and_then(Content::tabular_mut) {
                tab.table_mut().scroll_end();
            }
            Ok(None)
        }
        AppAction::TableToggleExpansion => {
            if let Some(tab) = app.tabs_mut().selected_mut().and_then(Content::tabular_mut) {
                tab.table_mut().toggle_expansion()?;
            }
            Ok(None)
        }
        AppAction::PalleteGotoNext => {
            if let Some(pallete) = app.pallete_mut() {
                pallete.input().goto_next();
            }
            Ok(None)
        }
        AppAction::PalleteGotoPrev => {
            if let Some(pallete) = app.pallete_mut() {
                pallete.input().goto_prev();
            }
            Ok(None)
        }
        AppAction::PalleteGotoStart => {
            if let Some(pallete) = app.pallete_mut() {
                pallete.input().goto_start();
            }
            Ok(None)
        }
        AppAction::PalleteGotoEnd => {
            if let Some(pallete) = app.pallete_mut() {
                pallete.input().goto_end();
            }
            Ok(None)
        }
        AppAction::PalleteDeleteNext => {
            if let Some(pallete) = app.pallete_mut() {
                pallete.input().delete_next();
            }
            Ok(None)
        }
        AppAction::PalleteDeletePrev => {
            if let Some(pallete) = app.pallete_mut() {
                pallete.input().delete_prev();
            }
            Ok(None)
        }
        AppAction::PalleteInsert(c) => {
            if let Some(pallete) = app.pallete_mut() {
                pallete.input().insert(c);
                pallete.list().select(None);
            }
            Ok(None)
        }
        AppAction::PalleteInsertSelectedOrCommit => {
            if let Some(selected) = app
                .pallete_mut()
                .and_then(|pallete| pallete.list().selected())
            {
                if let Some(cmd) = app.history_mut().get(selected).map(String::to_owned) {
                    if let Some(pallete) = app.pallete_mut() {
                        pallete.set_input(cmd);
                        pallete.list().select(None);
                    }
                }
                Ok(None)
            } else if let Some(cmd) = app.hide_pallete() {
                if cmd.is_empty() {
                    Ok(Some(AppAction::PalleteDeselectOrDismiss))
                } else {
                    app.history_mut().push(cmd.clone());
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
            if let Some(pallete) = app.pallete_mut() {
                if pallete.list().selected().is_some() {
                    pallete.list().select(None);
                } else {
                    app.hide_pallete();
                }
            }
            Ok(None)
        }
        AppAction::PalleteSelectPrevious => {
            if let Some(pallete) = app.pallete_mut() {
                pallete.list().select_previous();
            }
            Ok(None)
        }
        AppAction::PalleteSelectNext => {
            if let Some(pallete) = app.pallete_mut() {
                pallete.list().select_next();
            }
            Ok(None)
        }
    }
}
