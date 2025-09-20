use anyhow::Ok;

use ratatui::{
    Frame,
    layout::{Constraint, Flex, Layout},
    widgets::StatefulWidget,
};

use crate::tui::{
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
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Content {
    Schema,
    Tabulars,
}

pub struct App {
    tabs: TabState,
    schema: SchemaState,
    content: Content,
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
            content: Content::Tabulars,
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

    pub fn tabs(&self) -> &TabState {
        &self.tabs
    }

    pub fn tabs_mut(&mut self) -> &mut TabState {
        &mut self.tabs
    }

    pub fn schema(&self) -> &SchemaState {
        &self.schema
    }

    pub fn schema_mut(&mut self) -> &mut SchemaState {
        &mut self.schema
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

    pub fn content(&self) -> &Content {
        &self.content
    }

    pub fn show_palette(&mut self, cmd: impl ToString) {
        self.palette = Some(CommandPaletteState::new(cmd.to_string()));
    }

    pub fn hide_palette(&mut self) -> Option<String> {
        self.palette
            .take()
            .map(|mut palette| palette.input().value().to_owned())
    }

    pub fn error(&mut self, error: impl ToString) {
        self.error = Some(error.to_string());
    }

    pub fn dismiss_error(&mut self) {
        self.error = None;
    }

    pub fn switch_schema(&mut self) {
        self.content = Content::Schema;
    }

    pub fn switch_tabular(&mut self) {
        self.content = Content::Tabulars;
    }

    pub fn toggle_borders(&mut self) {
        self.borders = !self.borders;
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
        } else if let Content::Schema = self.content {
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
            }
        } else {
            Context::Empty
        }
    }

    pub fn draw(&mut self, frame: &mut Frame) -> AppResult<()> {
        // Draw table / item
        let state = self.context();
        match &mut self.content {
            Content::Schema => {
                frame.render_stateful_widget(Schema::default(), frame.area(), &mut self.schema);
            }
            Content::Tabulars => {
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
