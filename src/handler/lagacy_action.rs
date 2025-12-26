use std::{fs, ops::Div};

use anyhow::anyhow;
use crossterm::event::KeyEvent;
use itertools::Itertools;
use polars::frame::DataFrame;
use rand::Rng;

use crate::{
    AppResult,
    app::App,
    misc::{
        globals::{config, set_theme, sql},
        jagged_vec::JaggedVec,
        paths::config_path,
        polars_ext::{IntoString, PlotData},
        type_inferer::TypeInferer,
    },
    reader::{
        ArrowIpcToDataFrame, CsvToDataFrame, FwfToDataFrame, JsonLineToDataFrame, JsonToDataFrame,
        ParquetToDataFrame, ReadToDataFrames, Source, SqliteToDataFrames,
    },
    tui::{
        Pane, TableType,
        data_frame_table::DataFrameTableState,
        pane::Modal,
        plots::{histogram_plot::HistogramPlot, scatter_plot::ScatterPlot},
        popups::{
            export_wizard::ExportWizard,
            exporters::{
                arrow_exporter::ArrowExporter, csv_exporter::CsvExporterState,
                json_exporter::JsonExporterState, jsonl_exporter::JsonLExporterState,
                parquet_exporter::ParquetExporterState, tsv_exporter::TsvExporter,
            },
            histogram_wizard::HistogramWizard,
            inline_query::InlineQueryType,
        },
        themes::theme::Theme,
    },
    writer::{
        Destination, JsonFormat, WriteToArrow, WriteToCsv, WriteToFile, WriteToJson, WriteToParquet,
    },
};

use super::command::{commands_help_data_frame, parse_into_action};

#[derive(Debug, Clone)]
pub enum AppAction {
    Help,
    ImportArrow(Source),
    ImportDsv {
        source: Source,
        separator: char,
        has_header: bool,
        quote: char,
    },
    ImportFwf {
        source: Source,
        widths: Vec<usize>,
        separator_length: usize,
        flexible_width: bool,
        has_header: bool,
    },
    ImportJson(Source, JsonFormat),
    ImportParquet(Source),
    ImportSqlite(Source),
    RegisterDataFrame(String),
    SchemaNamesSelectFirst,
    SchemaNamesSelectLast,
    SchemaOpenTable,
    SchemaUnloadTable,
    HelpShow,
    HelpDismiss,
    TableGotoRandom,
    TableInferColumns(TypeInferer),
    TableReset,
    TableScrollEnd,
    TableScrollLeft,
    TableScrollLeftColumn,
    TableScrollRight,
    TableScrollRightColumn,
    TableScrollStart,
    ToggleBorders,
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
        AppAction::SwitchToSchema => {
            app.show_schema();
            Ok(None)
        }
        AppAction::SwitchToTabulars => {
            app.show_tabular();
            Ok(None)
        }
        AppAction::DismissErrorAndShowPalette => {
            app.dismiss_error();
            app.show_palette("");
            Ok(None)
        }
        AppAction::HelpDismiss => {
            app.take_help();
            Ok(None)
        }
        AppAction::SheetShow => {
            if let Some(tab) = app.pane_mut() {
                tab.show_sheet()
            }
            Ok(None)
        }
        AppAction::SearchFuzzyShow => {
            if let Some(tab) = app.pane_mut() {
                tab.show_fuzzy_search();
            }
            Ok(None)
        }
        AppAction::SearchExactShow => {
            if let Some(tab) = app.pane_mut() {
                tab.show_exact_search();
            }
            Ok(None)
        }
        AppAction::GotoLine(line) => {
            match app.context() {
                crate::app::Context::Table => {
                    if let Some(tabular) = app.pane_mut() {
                        tabular.table_mut().select(line)
                    }
                }
                crate::app::Context::Schema => {
                    if let Some(schema) = app.schema_mut() {
                        schema.names_mut().table_mut().select(line.into());
                    }
                }
                _ => (),
            }
            Ok(None)
        }
        AppAction::TableInferColumns(type_inferer) => {
            if let Some(tab) = app.table_mut() {
                let df = tab.data_frame_mut();
                type_inferer.update(df);
            }
            Ok(None)
        }
        AppAction::TableGotoFirst => {
            if let Some(tab) = app.table_mut() {
                tab.select_first()
            }
            Ok(None)
        }
        AppAction::TableGotoLast => {
            if let Some(tab) = app.table_mut() {
                tab.select_last()
            }
            Ok(None)
        }
        AppAction::TableGotoRandom => {
            if let Some(tab) = app.table_mut() {
                let random_row = rand::rng().random_range(0..tab.data_frame().height());
                tab.select(random_row);
            }
            Ok(None)
        }
        AppAction::TableGoUp(lines) => {
            if let Some(tab) = app.table_mut() {
                tab.select_up(lines)
            }
            Ok(None)
        }
        AppAction::TableGoUpHalfPage => {
            if let Some(tab) = app.table_mut() {
                tab.select_up(tab.rendered_rows().div(2).into())
            }
            Ok(None)
        }
        AppAction::TableGoUpFullPage => {
            if let Some(tab) = app.table_mut() {
                tab.select_up(tab.rendered_rows().into())
            }
            Ok(None)
        }
        AppAction::TableGoDown(lines) => {
            if let Some(tab) = app.table_mut() {
                tab.select_down(lines)
            }
            Ok(None)
        }
        AppAction::TableGoDownHalfPage => {
            if let Some(tab) = app.table_mut() {
                tab.select_down(tab.rendered_rows().div(2).into())
            }
            Ok(None)
        }
        AppAction::TableGoDownFullPage => {
            if let Some(tab) = app.table_mut() {
                tab.select_down(tab.rendered_rows().into())
            }
            Ok(None)
        }
        AppAction::TableSelect(select) => Ok(Some(AppAction::TableQuery(format!(
            "SELECT {select} FROM _"
        )))),
        AppAction::TableOrder(order) => Ok(Some(AppAction::TableQuery(format!(
            "SELECT * FROM _ ORDER BY {order}"
        )))),
        AppAction::TableFilter(filter) => Ok(Some(AppAction::TableQuery(format!(
            "SELECT * FROM _ where {filter}"
        )))),
        AppAction::TableQuery(query) => {
            let df = sql().execute(
                &query,
                app.pane()
                    .map(Pane::table)
                    .map(DataFrameTableState::data_frame)
                    .cloned(),
            )?;
            Ok(Some(AppAction::TableSetDataFrame(df)))
        }
        AppAction::TableSetDataFrame(df) => {
            if let Some(tab) = app.pane_mut() {
                tab.table_mut().set_data_frame(df.clone());
            }
            Ok(None)
        }
        AppAction::TableReset => {
            let query = match app.pane().map(|ts| ts.table_type()) {
                Some(TableType::Name(name)) => Some(format!("SELECT * FROM '{name}'")),
                Some(TableType::Query(query)) => Some(query.to_owned()),
                Some(_) => None,
                None => None,
            };
            Ok(query.map(AppAction::TableQuery))
        }
        AppAction::SheetScrollUp => {
            if let Some(Modal::Sheet(sheet)) = app.modal_mut() {
                sheet.scroll_up();
            }
            Ok(None)
        }
        AppAction::SheetScrollDown => {
            if let Some(Modal::Sheet(sheet)) = app.modal_mut() {
                sheet.scroll_down();
            }
            Ok(None)
        }
        AppAction::TabNewQuery(query) => {
            if sql().schema().iter().any(|(name, _)| name == &query) {
                let df = sql().execute(&format!("SELECT * FROM '{query}'"), None)?;
                app.tab_mut_unchecked()
                    .add(Pane::new(df, TableType::Name(query)));
            } else {
                let df = sql().execute(&query, app.data_frame().cloned())?;
                app.tab_mut_unchecked()
                    .add(Pane::new(df, TableType::Query(query)));
            }

            Ok(Some(AppAction::TabSelect(usize::MAX)))
        }
        AppAction::TabSelect(idx) => {
            app.tab_mut_unchecked().select(idx);
            app.show_tabular();
            Ok(None)
        }
        AppAction::TabRemove(idx) => {
            app.tab_mut_unchecked().remove(idx);
            Ok(Some(AppAction::TabSelect(idx)))
        }
        AppAction::TabPrev => {
            if let Some(tab) = app.tabs() {
                Ok(Some(AppAction::TabSelect(tab.idx().saturating_sub(1))))
            } else {
                Ok(None)
            }
        }
        AppAction::TabNext => {
            if let Some(tab) = app.tabs() {
                Ok(Some(AppAction::TabSelect(tab.idx().saturating_add(1))))
            } else {
                Ok(None)
            }
        }
        AppAction::TabRemoveOrQuit => {
            if let Some(tab) = app.tabs_mut() {
                if tab.len() == 1 {
                    app.quit();
                } else {
                    tab.remove(tab.idx());
                }
            }
            Ok(None)
        }
        AppAction::ExportDsv {
            destination,
            separator,
            quote,
            header,
        } => {
            if let Some(df) = app.data_frame_mut() {
                WriteToCsv::default()
                    .with_separator_char(separator)
                    .with_quote_char(quote)
                    .with_header(header)
                    .write_to_file(destination, df)?;
                Ok(None)
            } else {
                Err(anyhow!("Unable to export the data frame"))
            }
        }
        AppAction::ExportParquet(path) => {
            if let Some(df) = app.data_frame_mut() {
                WriteToParquet.write_to_file(path, df)?;
                Ok(None)
            } else {
                Err(anyhow!("Unable to export the data frame"))
            }
        }
        AppAction::ExportJson(path, fmt) => {
            if let Some(df) = app.data_frame_mut() {
                WriteToJson::default()
                    .with_format(fmt)
                    .write_to_file(path, df)?;
                Ok(None)
            } else {
                Err(anyhow!("Unable to export the data frame"))
            }
        }
        AppAction::ExportArrow(path) => {
            if let Some(df) = app.data_frame_mut() {
                WriteToArrow.write_to_file(path, df)?;
                Ok(None)
            } else {
                Err(anyhow!("Unable to export the data frame"))
            }
        }
        AppAction::ImportDsv {
            source,
            separator,
            has_header,
            quote,
        } => {
            let frames = CsvToDataFrame::default()
                .with_separator(separator)
                .with_quote_char(quote)
                .with_no_header(!has_header)
                .named_frames(source.clone())?;
            for (name, df) in frames {
                let name = sql().register(&name, df.clone(), source.clone());
                app.tab_mut_unchecked()
                    .add(Pane::new(df, TableType::Name(name)));
            }
            Ok(Some(AppAction::TabSelect(usize::MAX)))
        }
        AppAction::ImportParquet(source) => {
            let frames = ParquetToDataFrame.named_frames(source.clone())?;
            for (name, df) in frames {
                let name = sql().register(&name, df.clone(), source.clone());
                app.tab_mut_unchecked()
                    .add(Pane::new(df, TableType::Name(name)));
            }
            Ok(Some(AppAction::TabSelect(usize::MAX)))
        }
        AppAction::ImportJson(source, json_format) => {
            let frames = match json_format {
                JsonFormat::Json => JsonToDataFrame::default().named_frames(source.clone())?,
                JsonFormat::JsonLine => {
                    JsonLineToDataFrame::default().named_frames(source.clone())?
                }
            };
            for (name, df) in frames {
                let name = sql().register(&name, df.clone(), source.clone());
                app.tab_mut_unchecked()
                    .add(Pane::new(df, TableType::Name(name)));
            }
            Ok(Some(AppAction::TabSelect(usize::MAX)))
        }
        AppAction::ImportArrow(source) => {
            let frames = ArrowIpcToDataFrame.named_frames(source.clone())?;
            for (name, df) in frames {
                let name = sql().register(&name, df.clone(), source.clone());
                app.tab_mut_unchecked()
                    .add(Pane::new(df, TableType::Name(name)));
            }
            Ok(Some(AppAction::TabSelect(usize::MAX)))
        }
        AppAction::ImportSqlite(source) => {
            let frames = SqliteToDataFrames::default().named_frames(source.clone())?;
            for (name, df) in frames {
                let name = sql().register(&name, df.clone(), source.clone());
                app.tab_mut_unchecked()
                    .add(Pane::new(df, TableType::Name(name)));
            }
            Ok(Some(AppAction::TabSelect(usize::MAX)))
        }
        AppAction::ImportFwf {
            source,
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
                .named_frames(source.clone())?;
            for (name, df) in frames {
                let name = sql().register(&name, df.clone(), source.clone());
                app.tab_mut_unchecked()
                    .add(Pane::new(df, TableType::Name(name)));
            }
            Ok(Some(AppAction::TabSelect(usize::MAX)))
        }
        AppAction::SearchHandleKeyEvent(event) => {
            if let Some(Modal::SearchBar(sb)) = app.modal_mut() {
                sb.handle_key(event);
            }
            Ok(None)
        }
        AppAction::SearchCommit => {
            if let Some(tab) = app.pane_mut()
                && let Modal::SearchBar(sb) = tab.modal_take()
                && let Some(df) = sb.searcher().latest()
            {
                tab.table_mut().set_data_frame(df);
            }
            Ok(None)
        }
        AppAction::SearchRollback => {
            if let Some(tab) = app.pane_mut()
                && let Modal::SearchBar(sb) = tab.modal_take()
            {
                tab.table_mut().set_data_frame(sb.into_rollback_df());
            }
            Ok(None)
        }
        AppAction::Help => {
            let idx = app
                .tab_unchecked()
                .iter()
                .enumerate()
                .find_map(|(idx, tab)| match tab.table_type() {
                    TableType::Help => Some(idx),
                    _ => None,
                });
            if let Some(idx) = idx {
                Ok(Some(AppAction::TabSelect(idx)))
            } else {
                app.tab_mut_unchecked()
                    .add(Pane::new(commands_help_data_frame(), TableType::Help));
                Ok(Some(AppAction::TabSelect(usize::MAX)))
            }
        }
        AppAction::HelpShow => {
            app.show_help();
            Ok(None)
        }
        AppAction::Quit => {
            app.quit();
            Ok(None)
        }
        AppAction::TableScrollRight => {
            if let Some(tab) = app.table_mut() {
                tab.scroll_right();
            }
            Ok(None)
        }
        AppAction::TableScrollLeft => {
            if let Some(tab) = app.table_mut() {
                tab.scroll_left();
            }
            Ok(None)
        }
        AppAction::TableScrollRightColumn => {
            if let Some(tab) = app.table_mut() {
                tab.scroll_right_column();
            }
            Ok(None)
        }
        AppAction::TableScrollLeftColumn => {
            if let Some(tab) = app.table_mut() {
                tab.scroll_left_column();
            }
            Ok(None)
        }
        AppAction::TableScrollStart => {
            if let Some(tab) = app.table_mut() {
                tab.scroll_start();
            }
            Ok(None)
        }
        AppAction::TableScrollEnd => {
            if let Some(tab) = app.table_mut() {
                tab.scroll_end();
            }
            Ok(None)
        }
        AppAction::TableToggleExpansion => {
            if let Some(tab) = app.table_mut() {
                tab.toggle_expansion()?;
            }
            Ok(None)
        }
        AppAction::PaletteGotoNext => {
            if let Some(palette) = app.palette_mut() {
                palette.input().goto_next();
            }
            Ok(None)
        }
        AppAction::PaletteGotoPrev => {
            if let Some(palette) = app.palette_mut() {
                palette.input().goto_prev();
            }
            Ok(None)
        }
        AppAction::PaletteGotoStart => {
            if let Some(palette) = app.palette_mut() {
                palette.input().goto_start();
            }
            Ok(None)
        }
        AppAction::PaletteGotoNextWord => {
            if let Some(palette) = app.palette_mut() {
                palette.input().goto_next_word();
            }
            Ok(None)
        }
        AppAction::PaletteGotoPrevWord => {
            if let Some(palette) = app.palette_mut() {
                palette.input().goto_prev_word();
            }
            Ok(None)
        }
        AppAction::PaletteGotoEnd => {
            if let Some(palette) = app.palette_mut() {
                palette.input().goto_end();
            }
            Ok(None)
        }
        AppAction::PaletteDeleteNext => {
            if let Some(palette) = app.palette_mut() {
                palette.input().delete_next();
            }
            Ok(None)
        }
        AppAction::PaletteDeletePrev => {
            if let Some(palette) = app.palette_mut() {
                palette.input().delete_prev();
            }
            Ok(None)
        }
        AppAction::PaletteDeleteNextWord => {
            if let Some(palette) = app.palette_mut() {
                palette.input().delete_next_word();
            }
            Ok(None)
        }
        AppAction::PaletteDeletePrevWord => {
            if let Some(palette) = app.palette_mut() {
                palette.input().delete_prev_word();
            }
            Ok(None)
        }
        AppAction::PaletteInsert(c) => {
            if let Some(palette) = app.palette_mut() {
                palette.input().insert(c);
                palette.list().select(None);
            }
            Ok(None)
        }
        AppAction::PaletteInsertSelectedOrCommit => {
            if let Some(selected) = app
                .palette_mut()
                .and_then(|palette| palette.list().selected())
            {
                if let Some(cmd) = app.history_mut().get(selected).map(String::to_owned)
                    && let Some(palette) = app.palette_mut()
                {
                    palette.set_input(cmd);
                    palette.list().select(None);
                }
                Ok(None)
            } else if let Some(cmd) = app
                .take_palette()
                .map(|mut cp| cp.input().value().to_owned())
            {
                if cmd.is_empty() {
                    Ok(Some(AppAction::PaletteDeselectOrDismiss))
                } else {
                    app.history_mut().push(cmd.clone());
                    parse_into_action(cmd).map(Some)
                }
            } else {
                Ok(None)
            }
        }
        AppAction::PaletteShow(text) => {
            app.show_palette(text);
            Ok(None)
        }
        AppAction::PaletteDeselectOrDismiss => {
            if let Some(palette) = app.palette_mut() {
                if palette.list().selected().is_some() {
                    palette.list().select(None);
                } else {
                    app.take_palette();
                }
            }
            Ok(None)
        }
        AppAction::PaletteSelectPrevious => {
            if let Some(palette) = app.palette_mut() {
                palette.list().select_previous();
            }
            Ok(None)
        }
        AppAction::PaletteSelectNext => {
            if let Some(palette) = app.palette_mut() {
                palette.list().select_next();
            }
            Ok(None)
        }
        AppAction::SchemaNamesSelectPrev => {
            if let Some(schema) = app.schema_mut() {
                schema.names_mut().table_mut().select_previous();
            }
            Ok(None)
        }
        AppAction::SchemaNamesSelectNext => {
            if let Some(schema) = app.schema_mut() {
                schema.names_mut().table_mut().select_next();
            }
            Ok(None)
        }
        AppAction::SchemaNamesSelectFirst => {
            if let Some(schema) = app.schema_mut() {
                schema.names_mut().table_mut().select_first();
            }
            Ok(None)
        }
        AppAction::SchemaNamesSelectLast => {
            if let Some(schema) = app.schema_mut() {
                schema.names_mut().table_mut().select_last();
            }
            Ok(None)
        }
        AppAction::SchemaFieldsScrollUp => {
            if let Some(schema) = app.schema_mut() {
                schema.info_mut().fields_mut().scroll_up();
            }
            Ok(None)
        }
        AppAction::SchemaFieldsScrollDown => {
            if let Some(schema) = app.schema_mut() {
                schema.info_mut().fields_mut().scroll_down();
            }
            Ok(None)
        }
        AppAction::SchemaOpenTable => {
            if let Some(schema) = app.schema() {
                let table_name = schema
                    .names()
                    .table()
                    .selected()
                    .and_then(|idx| {
                        sql()
                            .schema()
                            .get_by_index(idx)
                            .map(|(name, _)| name.to_owned())
                    })
                    .ok_or(anyhow!("No table is selected"))?;

                let tab_idx = app
                    .tab_unchecked()
                    .iter()
                    .map(|tabular| tabular.table_type())
                    .enumerate()
                    .find_map(|(idx, tab_type)| match tab_type {
                        TableType::Name(name) if name.as_str() == table_name => Some(idx),
                        _ => None,
                    });

                if let Some(tab_idx) = tab_idx {
                    Ok(Some(AppAction::TabSelect(tab_idx)))
                } else {
                    Ok(Some(AppAction::TabNewQuery(table_name)))
                }
            } else {
                Ok(None)
            }
        }
        AppAction::SchemaUnloadTable => {
            if let Some(schema) = app.schema() {
                sql().unregister(
                    schema
                        .names()
                        .table()
                        .selected()
                        .and_then(|idx| {
                            sql()
                                .schema()
                                .get_by_index(idx)
                                .map(|(name, _)| name.to_owned())
                        })
                        .ok_or(anyhow!("No table is selected"))?
                        .as_str(),
                );
            }
            Ok(None)
        }
        AppAction::TabShowPanel => {
            if let Some(tabs) = app.tabs_mut() {
                tabs.show_side_panel();
            }
            Ok(None)
        }
        AppAction::TabHidePanel => {
            if let Some(tabs) = app.tabs_mut() {
                tabs.take_side_panel();
            }
            Ok(None)
        }
        AppAction::TabPanelPrev => {
            if let Some(side_panel) = app.side_panel_mut() {
                side_panel.list_mut().select_previous();
            }
            Ok(None)
        }
        AppAction::TabPanelNext => {
            if let Some(side_panel) = app.side_panel_mut() {
                side_panel.list_mut().select_next();
            }
            Ok(None)
        }
        AppAction::TabPanelSelect => {
            if let Some(idx) = app
                .tabs_mut()
                .and_then(|tab| tab.take_side_panel())
                .and_then(|panel| panel.list().selected())
            {
                Ok(Some(AppAction::TabSelect(idx)))
            } else {
                Ok(None)
            }
        }
        AppAction::RegisterDataFrame(name) => {
            if sql().schema().iter().map(|(name, _)| name).contains(&name) {
                Err(anyhow!("Data frame with name '{}' already exists.", &name))
            } else {
                if let Some(data_frame) = app.table().map(DataFrameTableState::data_frame).cloned()
                {
                    sql().register(&name, data_frame, crate::misc::sql::Source::User);
                }

                Ok(None)
            }
        }
        AppAction::DataFrameInfoScrollUp => {
            if let Some(Modal::DataFrameInfo(info)) = app.modal_mut() {
                info.fields_mut().scroll_up();
            }
            Ok(None)
        }
        AppAction::DataFrameInfoScrollDown => {
            if let Some(Modal::DataFrameInfo(info)) = app.modal_mut() {
                info.fields_mut().scroll_down();
            }
            Ok(None)
        }
        AppAction::DataFrameInfoShow => {
            if let Some(content) = app.pane_mut() {
                content.show_data_frame_info();
            }
            Ok(None)
        }
        AppAction::ScatterPlot(x_lab, y_lab, group_by) => {
            if let Some(tab_content) = app.pane_mut() {
                let df = tab_content.table().data_frame();
                if group_by.is_empty() {
                    let mut data = JaggedVec::new();
                    data.push(df.scatter_plot_data(&x_lab, &y_lab)?);
                    tab_content.show_scatter_plot(ScatterPlot::new(x_lab, y_lab, data)?)
                } else {
                    let mut groups = Vec::new();
                    let mut data = JaggedVec::new();
                    for df in df.partition_by(&group_by, true)? {
                        let name = group_by
                            .iter()
                            .map(|col| {
                                df.column(col)
                                    .and_then(|column| column.get(0))
                                    .map(IntoString::into_single_line)
                                    .unwrap_or("null".to_owned())
                            })
                            .join(" - ");
                        groups.push(name);
                        data.push(df.scatter_plot_data(&x_lab, &y_lab)?);
                    }
                    tab_content.show_scatter_plot(
                        ScatterPlot::new(x_lab, y_lab, data)?.with_groups(groups),
                    )
                }
            }
            Ok(None)
        }
        AppAction::HistogramPlot(group_by, buckets) => {
            if let Some(tab_content) = app.pane_mut() {
                let df = tab_content.table().data_frame();
                tab_content.show_histogram_plot(HistogramPlot::new(
                    df.histogram_plot_data(&group_by, buckets)?,
                ))
            }
            Ok(None)
        }
        AppAction::HistogramScrollUp => {
            if let Some(Modal::HistogramPlot(hist)) = app.modal_mut() {
                hist.scroll_up();
            }
            Ok(None)
        }
        AppAction::HistogramScrollDown => {
            if let Some(Modal::HistogramPlot(hist)) = app.modal_mut() {
                hist.scroll_down();
            }
            Ok(None)
        }
        AppAction::PreviewTheme(theme) => {
            set_theme(theme);
            Ok(None)
        }
        AppAction::StoreConfig => {
            let config_path = config_path().ok_or(anyhow!("Home not found"))?;
            fs::create_dir_all(config_path.parent().ok_or(anyhow!("Config parent error"))?)?;
            fs::write(config_path, config().store()?)?;
            Ok(None)
        }
        AppAction::ThemeSelectorSelectPrev => {
            if let Some(theme_selector) = app.theme_selector_mut() {
                theme_selector
                    .search_picker_mut()
                    .list_mut()
                    .select_previous();
            }
            Ok(None)
        }
        AppAction::ThemeSelectorSelectNext => {
            if let Some(theme_selector) = app.theme_selector_mut() {
                theme_selector.search_picker_mut().list_mut().select_next();
            }
            Ok(None)
        }
        AppAction::ThemeSelectorRollback => {
            if let Some(theme_selector) = app.take_theme_selector() {
                Ok(Some(AppAction::PreviewTheme(
                    theme_selector.into_rollback_theme(),
                )))
            } else {
                Ok(None)
            }
        }
        AppAction::ThemeSelectorCommit => {
            if let Some(theme_selector) = app.theme_selector_mut()
                && theme_selector.search_picker().selected().is_some()
            {
                app.take_theme_selector();
                Ok(Some(AppAction::StoreConfig))
            } else {
                Ok(None)
            }
        }
        AppAction::ThemeSelectorHandleKeyEvent(event) => {
            if let Some(theme_selector) = app.theme_selector_mut() {
                theme_selector.search_picker_mut().input_mut().handle(event);
            }
            Ok(None)
        }
        AppAction::ThemeSelectorShow => {
            app.show_theme_selector();
            Ok(None)
        }
        AppAction::PalleteHandleKeyEvent(key_event) => {
            if let Some(palette) = app.palette_mut() {
                palette.input().handle(key_event);
            }
            Ok(None)
        }
        AppAction::InlineQueryShow(query_type) => {
            if let Some(pane) = app.pane_mut() {
                pane.show_inline_query(query_type);
            }
            Ok(None)
        }
        AppAction::InlineQueryCommit => {
            if let Some(Modal::InlineQuery(inline_query)) = app.modal_take() {
                match inline_query.query_type() {
                    InlineQueryType::Filter => Ok(Some(AppAction::TableFilter(
                        inline_query.value().to_owned(),
                    ))),
                    InlineQueryType::Order => Ok(Some(AppAction::TableFilter(
                        inline_query.value().to_owned(),
                    ))),
                }
            } else {
                Ok(None)
            }
        }
        AppAction::InlineQueryHandleKeyEvent(event) => {
            if let Some(Modal::InlineQuery(inline_query)) = app.modal_mut() {
                inline_query.handle(event);
            }
            Ok(None)
        }
        AppAction::GoToLineShow => {
            if let Some(pane) = app.pane_mut() {
                pane.show_go_to_line();
            }
            Ok(None)
        }
        AppAction::GoToLineShowWithValue(value) => {
            if let Some(pane) = app.pane_mut() {
                pane.show_go_to_line_with_value(value);
            }
            Ok(None)
        }
        AppAction::GoToLineRollback => {
            if let Some(Modal::GoToLine(gtl)) = app.modal_take()
                && let Some(pane) = app.pane_mut()
            {
                pane.table_mut().select(gtl.rollback());
            }
            Ok(None)
        }
        AppAction::GoToLineCommit => {
            app.modal_take();
            Ok(None)
        }
        AppAction::GoToLineHandleKeyEvent(event) => {
            if let Some(Modal::GoToLine(go_to_line)) = app.modal_mut() {
                go_to_line.handle(event);
            }
            Ok(None)
        }
        AppAction::ExportDataFrameShow => {
            if let Some(pane) = app.pane_mut() {
                pane.show_export_data_frame();
            }
            Ok(None)
        }
        AppAction::DismissModal => {
            app.modal_take();
            Ok(None)
        }
        AppAction::ExportWizardSelectNext => {
            if let Some(Modal::ExportWizard(wizard)) = app.modal_mut() {
                wizard.select_next();
            }
            Ok(None)
        }
        AppAction::ExportWizardSelectPrev => {
            if let Some(Modal::ExportWizard(wizard)) = app.modal_mut() {
                wizard.select_previous();
            }
            Ok(None)
        }
        AppAction::ExportWizardNextStep => {
            if let Some(Modal::ExportWizard(wizard)) = app.modal_mut() {
                wizard.step();
                let next = match wizard {
                    ExportWizard::Csv(CsvExporterState::ExportToFile {
                        separator,
                        quote,
                        path,
                    }) => Some(AppAction::ExportDsv {
                        destination: Destination::File(path.clone()),
                        separator: *separator,
                        quote: *quote,
                        header: true,
                    }),
                    ExportWizard::Csv(CsvExporterState::ExportToClipboard { separator, quote }) => {
                        Some(AppAction::ExportDsv {
                            destination: Destination::Clipboard,
                            separator: *separator,
                            quote: *quote,
                            header: true,
                        })
                    }
                    ExportWizard::Tsv(TsvExporter::ExportToFile { path }) => {
                        Some(AppAction::ExportDsv {
                            destination: Destination::File(path.clone()),
                            separator: '\t',
                            quote: '"',
                            header: false,
                        })
                    }
                    ExportWizard::Tsv(TsvExporter::ExportToClipboard) => {
                        Some(AppAction::ExportDsv {
                            destination: Destination::Clipboard,
                            separator: '\t',
                            quote: '"',
                            header: false,
                        })
                    }
                    ExportWizard::Json(JsonExporterState::ExportToFile { path }) => Some(
                        AppAction::ExportJson(Destination::File(path.clone()), JsonFormat::Json),
                    ),
                    ExportWizard::Json(JsonExporterState::ExportToClipboard) => Some(
                        AppAction::ExportJson(Destination::Clipboard, JsonFormat::Json),
                    ),
                    ExportWizard::JsonL(JsonLExporterState::ExportToFile { path }) => {
                        Some(AppAction::ExportJson(
                            Destination::File(path.clone()),
                            JsonFormat::JsonLine,
                        ))
                    }
                    ExportWizard::JsonL(JsonLExporterState::ExportToClipboard) => Some(
                        AppAction::ExportJson(Destination::Clipboard, JsonFormat::JsonLine),
                    ),
                    ExportWizard::Parquet(ParquetExporterState::ExportToFile { path }) => {
                        Some(AppAction::ExportParquet(Destination::File(path.clone())))
                    }
                    ExportWizard::Arrow(ArrowExporter::ExportToFile { path }) => {
                        Some(AppAction::ExportArrow(Destination::File(path.clone())))
                    }
                    _ => None,
                };
                if next.is_some() {
                    app.modal_take();
                }
                Ok(next)
            } else {
                Ok(None)
            }
        }
        AppAction::ExportWizardHandleKeyEvent(event) => {
            if let Some(Modal::ExportWizard(wizard)) = app.modal_mut() {
                wizard.handle(event);
            }
            Ok(None)
        }
        AppAction::HistogramWizardShow => {
            if let Some(pane) = app.pane_mut() {
                pane.show_histogram_wizard();
            }
            Ok(None)
        }
        AppAction::HistogramWizardSelectNext => {
            if let Some(Modal::HistogramWizard(wizard)) = app.modal_mut() {
                wizard.select_next();
            }
            Ok(None)
        }
        AppAction::HistogramWizardSelectPrev => {
            if let Some(Modal::HistogramWizard(wizard)) = app.modal_mut() {
                wizard.select_previous();
            }
            Ok(None)
        }
        AppAction::HistogramWizardNextStep => {
            if let Some(Modal::HistogramWizard(wizard)) = app.modal_mut() {
                wizard.step();
                let next = if let HistogramWizard::Show { column, buckets } = wizard {
                    let next = AppAction::HistogramPlot(column.to_owned(), *buckets);
                    app.modal_take();
                    Some(next)
                } else {
                    None
                };
                Ok(next)
            } else {
                Ok(None)
            }
        }
        AppAction::HistogramWizardHandleKeyEvent(key_event) => {
            if let Some(Modal::HistogramWizard(wizard)) = app.modal_mut() {
                wizard.handle(key_event);
            }
            Ok(None)
        }
    }
}
