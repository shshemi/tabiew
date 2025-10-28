use anyhow::Ok;

use polars::frame::DataFrame;
use ratatui::{
    Frame,
    layout::{Constraint, Flex, Layout},
    widgets::{StatefulWidget, Widget},
};

use crate::tui::{
    PaneState,
    data_frame_table::DataFrameTableState,
    enumerated_list::EnumeratedListState,
    popups::{
        command_palette::{CommandPalette, CommandPaletteState},
        help_modal::HelpModal,
        theme_selector::{ThemeSelector, ThemeSelectorState},
    },
    schema::schema::{Schema, SchemaState},
};
use crate::{
    AppResult,
    misc::history::History,
    tui::{
        error_popup::ErrorPopup,
        pane::Modal,
        tabs::{Tabs, TabsState},
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
    InlineQuery,
    GoToLine,
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
            Context::InlineQuery => Context::Empty.into(),
            Context::Help => Context::Empty.into(),
            Context::GoToLine => Context::Empty.into(),
        }
    }
}

#[derive(Debug, Default)]
pub enum Overlay {
    Schema(SchemaState),
    Error(String),
    CommandPalette(CommandPaletteState),
    ThemeSelector(ThemeSelectorState),
    Help,
    #[default]
    None,
}

pub struct App {
    tabs: TabsState,
    overlay: Overlay,
    history: History,
    borders: bool,
    running: bool,
}

impl App {
    pub fn new(tabs: TabsState, history: History) -> Self {
        Self {
            tabs,
            history,
            overlay: Overlay::None,
            borders: true,
            running: true,
        }
    }

    pub fn running(&self) -> bool {
        self.running
    }

    pub fn schema(&self) -> Option<&SchemaState> {
        if let Overlay::Schema(schema) = self.overlay() {
            Some(schema)
        } else {
            None
        }
    }

    pub fn schema_mut(&mut self) -> Option<&mut SchemaState> {
        if let Overlay::Schema(schema) = self.overlay_mut() {
            Some(schema)
        } else {
            None
        }
    }

    pub fn tabs(&self) -> Option<&TabsState> {
        matches!(self.overlay(), Overlay::None).then_some(&self.tabs)
    }

    pub fn tabs_mut(&mut self) -> Option<&mut TabsState> {
        matches!(self.overlay(), Overlay::None).then_some(&mut self.tabs)
    }

    pub fn side_panel(&self) -> Option<&EnumeratedListState> {
        self.tabs().and_then(|t| t.side_panel())
    }

    pub fn side_panel_mut(&mut self) -> Option<&mut EnumeratedListState> {
        self.tabs_mut().and_then(|t| t.side_panel_mut())
    }

    pub fn pane(&self) -> Option<&PaneState> {
        self.tabs().and_then(|t| t.selected())
    }

    pub fn pane_mut(&mut self) -> Option<&mut PaneState> {
        self.tabs_mut().and_then(|t| t.selected_mut())
    }

    pub fn modal(&self) -> Option<&Modal> {
        self.pane().map(|c| c.modal())
    }

    pub fn modal_mut(&mut self) -> Option<&mut Modal> {
        self.pane_mut().map(|c| c.modal_mut())
    }

    pub fn modal_take(&mut self) -> Option<Modal> {
        self.pane_mut().map(|c| c.modal_take())
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
        if let Overlay::CommandPalette(palette) = &mut self.overlay {
            Some(palette)
        } else {
            None
        }
    }

    pub fn theme_selector(&self) -> Option<&ThemeSelectorState> {
        if let Overlay::ThemeSelector(theme_selector) = &self.overlay {
            Some(theme_selector)
        } else {
            None
        }
    }

    pub fn theme_selector_mut(&mut self) -> Option<&mut ThemeSelectorState> {
        if let Overlay::ThemeSelector(theme_selector) = &mut self.overlay {
            Some(theme_selector)
        } else {
            None
        }
    }

    pub fn take_theme_selector(&mut self) -> Option<ThemeSelectorState> {
        if matches!(&self.overlay, Overlay::ThemeSelector(_))
            && let Overlay::ThemeSelector(theme_selector) = std::mem::take(&mut self.overlay)
        {
            Some(theme_selector)
        } else {
            None
        }
    }

    pub fn show_theme_selector(&mut self) {
        self.overlay = Overlay::ThemeSelector(Default::default())
    }

    pub fn history_mut(&mut self) -> &mut History {
        &mut self.history
    }

    pub fn overlay(&self) -> &Overlay {
        &self.overlay
    }

    pub fn overlay_mut(&mut self) -> &mut Overlay {
        &mut self.overlay
    }

    pub fn show_palette(&mut self, cmd: impl ToString) {
        self.overlay = Overlay::CommandPalette(CommandPaletteState::new(cmd.to_string()));
    }

    pub fn take_palette(&mut self) -> Option<CommandPaletteState> {
        if matches!(&self.overlay, Overlay::CommandPalette(_))
            && let Overlay::CommandPalette(palette) = std::mem::take(&mut self.overlay)
        {
            Some(palette)
        } else {
            None
        }
    }

    pub fn show_error(&mut self, error: impl ToString) {
        self.overlay = Overlay::Error(error.to_string());
    }

    pub fn dismiss_error(&mut self) {
        if matches!(&self.overlay, Overlay::Error(_)) {
            self.overlay = Overlay::None;
        }
    }

    pub fn show_schema(&mut self) {
        self.overlay = Overlay::Schema(SchemaState::default());
    }

    pub fn show_tabular(&mut self) {
        self.overlay = Overlay::None;
    }

    pub fn toggle_borders(&mut self) {
        self.borders = !self.borders;
    }

    pub fn tab_unchecked(&self) -> &TabsState {
        &self.tabs
    }

    pub fn tab_mut_unchecked(&mut self) -> &mut TabsState {
        &mut self.tabs
    }

    pub fn show_help(&mut self) {
        self.overlay = Overlay::Help
    }

    pub fn take_help(&mut self) {
        if matches!(self.overlay, Overlay::Help) {
            self.overlay = Overlay::None;
        }
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
        match self.overlay {
            Overlay::Schema(_) => Context::Schema,
            Overlay::Error(_) => Context::Error,
            Overlay::CommandPalette(_) => Context::Command,
            Overlay::ThemeSelector(_) => Context::ThemeSelector,
            Overlay::Help => Context::Help,
            Overlay::None => match self.modal() {
                Some(Modal::None) => {
                    if self.tab_unchecked().side_panel().is_some() {
                        Context::TabSidePanel
                    } else {
                        Context::Table
                    }
                }
                Some(Modal::SearchBar(_)) => Context::Search,
                Some(Modal::Sheet(_)) => Context::Sheet,
                Some(Modal::DataFrameInfo(_)) => Context::DataFrameInfo,
                Some(Modal::ScatterPlot(_)) => Context::ScatterPlot,
                Some(Modal::HistogramPlot(_)) => Context::HistogramPlot,
                Some(Modal::InlineQuery(_)) => Context::InlineQuery,
                Some(Modal::GoToLine(_)) => Context::GoToLine,
                None => Context::Empty,
            },
        }
    }

    pub fn draw(&mut self, frame: &mut Frame) -> AppResult<()> {
        frame.render_stateful_widget(
            Tabs::new()
                .with_borders(self.borders)
                .selection(matches!(self.context(), Context::Table)),
            frame.area(),
            &mut self.tabs,
        );
        match &mut self.overlay {
            Overlay::Error(msg) => {
                let error = ErrorPopup::new().with_message(msg.as_str());
                frame.render_widget(error, frame.area());
            }
            Overlay::CommandPalette(cmd) => {
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
            Overlay::ThemeSelector(state) => {
                ThemeSelector::default().render(frame.area(), frame.buffer_mut(), state);
            }
            Overlay::Schema(state) => {
                frame.render_stateful_widget(Schema::default(), frame.area(), state);
            }
            Overlay::Help => {
                //
                let [area] = Layout::horizontal([Constraint::Length(90)])
                    .flex(Flex::Center)
                    .areas(frame.area());
                let [_, area] =
                    Layout::vertical([Constraint::Length(2), Constraint::Length(50)]).areas(area);
                Widget::render(HelpModal::new(), area, frame.buffer_mut());
            }
            Overlay::None => {}
        }

        Ok(())
    }
}
