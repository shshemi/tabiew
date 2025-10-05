use anyhow::Ok;

use polars::frame::DataFrame;
use ratatui::{
    Frame,
    layout::{Constraint, Flex, Layout},
    widgets::StatefulWidget,
};

use crate::tui::{
    PaneState,
    data_frame_table::DataFrameTableState,
    enumerated_list::EnumeratedListState,
    popups::{
        command_palette::{CommandPalette, CommandPaletteState},
        theme_selector::{ThemeSelector, ThemeSelectorState},
    },
    schema::schema::{Schema, SchemaState},
};
use crate::{
    AppResult,
    misc::history::History,
    tui::{
        error_popup::ErrorPopup,
        tab::{Tab, TabState},
        tab_content::Modal,
    },
};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Context {
    Empty,
    Table,
    Sheet,
    Command,
    Error,
    Search,
    Schema,
    TabSidePanel,
    DataFrameInfo,
    ScatterPlot,
    HistogramPlot,
    ThemeSelector,
    Help,
}

impl Context {
    pub fn parent(&self) -> Option<Context> {
        match self {
            Context::Empty => None,
            Context::Table => Context::Empty.into(),
            Context::Sheet => Context::Table.into(),
            Context::Command => Context::Empty.into(),
            Context::Error => Context::Empty.into(),
            Context::Search => Context::Table.into(),
            Context::Schema => Context::Empty.into(),
            Context::TabSidePanel => Context::Empty.into(),
            Context::DataFrameInfo => Context::Empty.into(),
            Context::ScatterPlot => Context::Empty.into(),
            Context::HistogramPlot => Context::Empty.into(),
            Context::ThemeSelector => Context::Empty.into(),
            Context::Help => Context::Empty.into(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Overlay {
    Schema,
    None,
}

pub struct App {
    tabs: TabState,
    schema: SchemaState,
    overlay: Overlay,
    error: Option<String>,
    palette: Option<CommandPaletteState>,
    theme_selector: Option<ThemeSelectorState>,
    history: History,
    borders: bool,
    running: bool,
}

impl App {
    pub fn new(tabs: TabState, history: History) -> Self {
        Self {
            tabs,
            history,
            schema: SchemaState::default(),
            overlay: Overlay::None,
            error: None,
            palette: None,
            theme_selector: None,
            borders: true,
            running: true,
        }
    }

    pub fn running(&self) -> bool {
        self.running
    }

    pub fn schema(&self) -> Option<&SchemaState> {
        (self.overlay == Overlay::Schema).then_some(&self.schema)
    }

    pub fn schema_mut(&mut self) -> Option<&mut SchemaState> {
        (self.overlay == Overlay::Schema).then_some(&mut self.schema)
    }

    pub fn tab(&self) -> Option<&TabState> {
        (self.overlay == Overlay::None).then_some(&self.tabs)
    }

    pub fn tab_mut(&mut self) -> Option<&mut TabState> {
        (self.overlay == Overlay::None).then_some(&mut self.tabs)
    }

    pub fn side_panel(&self) -> Option<&EnumeratedListState> {
        self.tab().and_then(|t| t.side_panel())
    }

    pub fn side_panel_mut(&mut self) -> Option<&mut EnumeratedListState> {
        self.tab_mut().and_then(|t| t.side_panel_mut())
    }

    pub fn pane(&self) -> Option<&PaneState> {
        self.tab().and_then(|t| t.selected())
    }

    pub fn pane_mut(&mut self) -> Option<&mut PaneState> {
        self.tab_mut().and_then(|t| t.selected_mut())
    }

    pub fn modal(&self) -> Option<&Modal> {
        self.pane().map(|c| c.modal())
    }

    pub fn modal_mut(&mut self) -> Option<&mut Modal> {
        self.pane_mut().map(|c| c.modal_mut())
    }

    pub fn table(&self) -> Option<&DataFrameTableState> {
        self.pane().map(|c| c.table())
    }

    pub fn table_mut(&mut self) -> Option<&mut DataFrameTableState> {
        self.pane_mut().map(|c| c.table_mut())
    }

    pub fn data_frame(&self) -> Option<&DataFrame> {
        self.table().map(|t| t.data_frame())
    }

    pub fn data_frame_mut(&mut self) -> Option<&mut DataFrame> {
        self.table_mut().map(|t| t.data_frame_mut())
    }

    pub fn palette_mut(&mut self) -> Option<&mut CommandPaletteState> {
        self.palette.as_mut()
    }

    pub fn theme_selector(&self) -> Option<&ThemeSelectorState> {
        self.theme_selector.as_ref()
    }

    pub fn theme_selector_mut(&mut self) -> Option<&mut ThemeSelectorState> {
        self.theme_selector.as_mut()
    }

    pub fn take_theme_selector(&mut self) -> Option<ThemeSelectorState> {
        self.theme_selector.take()
    }

    pub fn show_theme_selector(&mut self) {
        self.theme_selector = Some(Default::default())
    }

    pub fn history_mut(&mut self) -> &mut History {
        &mut self.history
    }

    pub fn overlay(&self) -> &Overlay {
        &self.overlay
    }

    pub fn show_palette(&mut self, cmd: impl ToString) {
        self.palette = Some(CommandPaletteState::new(cmd.to_string()));
    }

    pub fn hide_palette(&mut self) -> Option<String> {
        self.palette
            .take()
            .map(|mut palette| palette.input().value().to_owned())
    }

    pub fn show_error(&mut self, error: impl ToString) {
        self.error = Some(error.to_string());
    }

    pub fn dismiss_error(&mut self) {
        self.error = None;
    }

    pub fn show_schema(&mut self) {
        self.overlay = Overlay::Schema;
    }

    pub fn show_tabular(&mut self) {
        self.overlay = Overlay::None;
    }

    pub fn toggle_borders(&mut self) {
        self.borders = !self.borders;
    }

    pub fn tab_unchecked(&self) -> &TabState {
        &self.tabs
    }

    pub fn tab_mut_unchecked(&mut self) -> &mut TabState {
        &mut self.tabs
    }

    pub fn tick(&mut self) -> AppResult<()> {
        for tab in self.tabs.iter_mut() {
            tab.tick();
        }
        Ok(())
    }

    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn context(&self) -> Context {
        if self.error.is_some() {
            Context::Error
        } else if self.palette.is_some() {
            Context::Command
        } else if self.theme_selector.is_some() {
            Context::ThemeSelector
        } else if let Overlay::Schema = self.overlay {
            Context::Schema
        } else if self.tabs.side_panel().is_some() {
            Context::TabSidePanel
        } else if let Some(tabular) = self.tabs.selected() {
            match tabular.modal() {
                Modal::SearchBar(_) => Context::Search,
                Modal::Sheet(_) => Context::Sheet,
                Modal::None => Context::Table,
                Modal::DataFrameInfo(_) => Context::DataFrameInfo,
                Modal::ScatterPlot(_) => Context::ScatterPlot,
                Modal::HistogramPlot(_) => Context::HistogramPlot,
                Modal::Help => Context::Help,
            }
        } else {
            Context::Empty
        }
    }

    pub fn draw(&mut self, frame: &mut Frame) -> AppResult<()> {
        // Draw table / item
        let state = self.context();
        match &mut self.overlay {
            Overlay::Schema => {
                frame.render_stateful_widget(Schema::default(), frame.area(), &mut self.schema);
            }
            Overlay::None => {
                frame.render_stateful_widget(
                    Tab::new()
                        .with_borders(self.borders)
                        .selection(matches!(state, Context::Table)),
                    frame.area(),
                    &mut self.tabs,
                );
            }
        }

        if let Some(msg) = self.error.as_ref() {
            let error = ErrorPopup::new().with_message(msg.as_str());
            frame.render_widget(error, frame.area());
        }

        if let Some(ts) = self.theme_selector.as_mut() {
            ThemeSelector::default().render(frame.area(), frame.buffer_mut(), ts);
        }

        if let Some(cmd) = self.palette.as_mut() {
            let upmid = {
                let [mid_ver] = Layout::horizontal([Constraint::Max(80)])
                    .flex(Flex::Center)
                    .areas(frame.area());
                let [_, mid_hor] =
                    Layout::vertical([Constraint::Length(3), Constraint::Length(15)])
                        .areas(mid_ver);
                mid_hor
            };
            frame.render_stateful_widget(
                CommandPalette::new(self.history.iter().take(100)),
                upmid,
                cmd,
            );
        }

        Ok(())
    }
}
