use crate::{
    handler::message::Message,
    tui::{
        component::Component,
        popups::{command_picker::CommandPicker, help_modal::Help, theme_selector::ThemeSelector},
        schema::schema::Schema,
    },
};
use crate::{
    misc::history::History,
    tui::{error_popup::ErrorPopup, tabs::Tabs},
};
use crossterm::event::KeyCode;
use ratatui::layout::{Constraint, Flex, Layout};

#[derive(Debug)]
pub enum Overlay {
    Schema(Schema),
    Error(ErrorPopup),
    CommandPicker(CommandPicker),
    ThemeSelector(ThemeSelector),
    Help(Help),
}

impl Overlay {
    fn responder(&mut self) -> &mut dyn Component {
        match self {
            Overlay::Schema(schema) => schema,
            Overlay::Error(error) => error,
            Overlay::CommandPicker(command_palette) => command_palette,
            Overlay::ThemeSelector(theme_selector) => theme_selector,
            Overlay::Help(help) => help,
        }
    }
}

pub struct App {
    tabs: Tabs,
    overlay: Option<Overlay>,
    history: History,
    running: bool,
}

impl App {
    pub fn new(tabs: Tabs, history: History) -> Self {
        Self {
            tabs,
            history,
            overlay: None,
            running: true,
        }
    }

    pub fn running(&self) -> bool {
        self.running
    }

    // pub fn schema(&self) -> Option<&Schema> {
    //     if let Overlay::Schema(schema) = self.overlay() {
    //         Some(schema)
    //     } else {
    //         None
    //     }
    // }

    // pub fn schema_mut(&mut self) -> Option<&mut Schema> {
    //     if let Overlay::Schema(schema) = self.overlay_mut() {
    //         Some(schema)
    //     } else {
    //         None
    //     }
    // }

    // pub fn tabs(&self) -> Option<&TabsState> {
    //     matches!(self.overlay(), Overlay::None).then_some(&self.tabs)
    // }

    // pub fn tabs_mut(&mut self) -> Option<&mut TabsState> {
    //     matches!(self.overlay(), Overlay::None).then_some(&mut self.tabs)
    // }

    // pub fn side_panel(&self) -> Option<&EnumeratedList> {
    //     self.tabs().and_then(|t| t.side_panel())
    // }

    // pub fn side_panel_mut(&mut self) -> Option<&mut EnumeratedList> {
    //     self.tabs_mut().and_then(|t| t.side_panel_mut())
    // }

    // pub fn pane(&self) -> Option<&Pane> {
    //     self.tabs().and_then(|t| t.selected())
    // }

    // pub fn pane_mut(&mut self) -> Option<&mut Pane> {
    //     self.tabs_mut().and_then(|t| t.selected_mut())
    // }

    // pub fn modal(&self) -> Option<&Modal> {
    //     self.pane().map(|c| c.modal())
    // }

    // pub fn modal_mut(&mut self) -> Option<&mut Modal> {
    //     self.pane_mut().map(|c| c.modal_mut())
    // }

    // pub fn modal_take(&mut self) -> Option<Modal> {
    //     self.pane_mut().map(|c| c.modal_take())
    // }

    // pub fn table(&self) -> Option<&DataFrameTableState> {
    //     self.pane().map(|c| c.table())
    // }

    // pub fn table_mut(&mut self) -> Option<&mut DataFrameTableState> {
    //     self.pane_mut().map(|c| c.table_mut())
    // }

    // pub fn data_frame(&self) -> Option<&DataFrame> {
    //     self.table().map(|t| t.data_frame())
    // }

    // pub fn data_frame_mut(&mut self) -> Option<&mut DataFrame> {
    //     self.table_mut().map(|t| t.data_frame_mut())
    // }

    // pub fn palette_mut(&mut self) -> Option<&mut CommandPalette> {
    //     if let Overlay::CommandPalette(palette) = &mut self.overlay {
    //         Some(palette)
    //     } else {
    //         None
    //     }
    // }

    // pub fn theme_selector(&self) -> Option<&ThemeSelector> {
    //     if let Overlay::ThemeSelector(theme_selector) = &self.overlay {
    //         Some(theme_selector)
    //     } else {
    //         None
    //     }
    // }

    // pub fn theme_selector_mut(&mut self) -> Option<&mut ThemeSelector> {
    //     if let Overlay::ThemeSelector(theme_selector) = &mut self.overlay {
    //         Some(theme_selector)
    //     } else {
    //         None
    //     }
    // }

    // pub fn take_theme_selector(&mut self) -> Option<ThemeSelector> {
    //     if matches!(&self.overlay, Overlay::ThemeSelector(_))
    //         && let Overlay::ThemeSelector(theme_selector) = std::mem::take(&mut self.overlay)
    //     {
    //         Some(theme_selector)
    //     } else {
    //         None
    //     }
    // }

    pub fn show_theme_selector(&mut self) {
        self.overlay = Some(Overlay::ThemeSelector(Default::default()));
    }

    // pub fn history_mut(&mut self) -> &mut History {
    //     &mut self.history
    // }

    // pub fn overlay(&self) -> &Overlay {
    //     &self.overlay
    // }

    // pub fn overlay_mut(&mut self) -> &mut Overlay {
    //     &mut self.overlay
    // }
    pub fn show_palette(&mut self, _cmd: impl ToString) {
        self.overlay = Some(Overlay::CommandPicker(CommandPicker::default()));
    }
    // pub fn take_palette(&mut self) -> Option<CommandPalette> {
    //     if matches!(&self.overlay, Overlay::CommandPalette(_))
    //         && let Overlay::CommandPalette(palette) = std::mem::take(&mut self.overlay)
    //     {
    //         Some(palette)
    //     } else {
    //         None
    //     }
    // }

    pub fn show_error(&mut self, message: impl Into<String>) {
        self.overlay = Some(Overlay::Error(ErrorPopup::new(message)));
    }

    // pub fn dismiss_error(&mut self) {
    //     if matches!(&self.overlay, Overlay::Error(_)) {
    //         self.overlay = Overlay::None;
    //     }
    // }

    // pub fn show_schema(&mut self) {
    //     self.overlay = Overlay::Schema(Schema::default());
    // }

    // pub fn show_tabular(&mut self) {
    //     self.overlay = Overlay::None;
    // }

    // pub fn toggle_borders(&mut self) {
    //     self.borders = !self.borders;
    // }

    // pub fn tab_unchecked(&self) -> &TabsState {
    //     &self.tabs
    // }

    // pub fn tab_mut_unchecked(&mut self) -> &mut TabsState {
    //     &mut self.tabs
    // }

    // pub fn show_help(&mut self) {
    //     self.overlay = Overlay::Help(Help::new())
    // }

    // pub fn take_help(&mut self) {
    //     if matches!(self.overlay, Overlay::Help(_)) {
    //         self.overlay = Overlay::None;
    //     }
    // }

    fn hide_overlay(&mut self) {
        self.overlay = None;
    }

    fn quit(&mut self) {
        self.running = false;
    }

    // pub fn context(&self) -> Context {
    //     match self.overlay {
    //         Overlay::Schema(_) => Context::Schema,
    //         Overlay::Error(_) => Context::Error,
    //         Overlay::CommandPalette(_) => Context::Command,
    //         Overlay::ThemeSelector(_) => Context::ThemeSelector,
    //         Overlay::Help => Context::Help,
    //         Overlay::None => match self.modal() {
    //             Some(Modal::None) => {
    //                 if self.tab_unchecked().side_panel().is_some() {
    //                     Context::TabSidePanel
    //                 } else {
    //                     Context::Table
    //                 }
    //             }
    //             Some(Modal::SearchBar(_)) => Context::Search,
    //             Some(Modal::Sheet(_)) => Context::Sheet,
    //             Some(Modal::DataFrameInfo(_)) => Context::DataFrameInfo,
    //             Some(Modal::ScatterPlot(_)) => Context::ScatterPlot,
    //             Some(Modal::HistogramPlot(_)) => Context::HistogramPlot,
    //             Some(Modal::InlineQuery(_)) => Context::InlineQuery,
    //             Some(Modal::GoToLine(_)) => Context::GoToLine,
    //             Some(Modal::ExportWizard(_)) => Context::ExportWizard,
    //             Some(Modal::HistogramWizard(_)) => Context::HistogramWizard,
    //             None => Context::Empty,
    //         },
    //     }
    // }

    // pub fn draw(&mut self, frame: &mut Frame) -> AppResult<()> {
    //     frame.render_stateful_widget(
    //         Tabs::new()
    //             .with_borders(self.borders)
    //             .selection(matches!(self.context(), Context::Table)),
    //         frame.area(),
    //         &mut self.tabs,
    //     );
    //     match &mut self.overlay {
    //         Overlay::Error(msg) => {
    //             let error = ErrorPopup::new().with_message(msg.as_str());
    //             frame.render_widget(error, frame.area());
    //         }
    //         Overlay::CommandPalette(cmd) => {
    //             let upmid = {
    //                 let [mid_ver] = Layout::horizontal([Constraint::Max(80)])
    //                     .flex(Flex::Center)
    //                     .areas(frame.area());
    //                 let [_, mid_hor] =
    //                     Layout::vertical([Constraint::Length(3), Constraint::Length(15)])
    //                         .areas(mid_ver);
    //                 mid_hor
    //             };
    //             frame.render_stateful_widget(
    //                 CommandPalette::new(self.history.iter().take(100)),
    //                 upmid,
    //                 cmd,
    //             );
    //         }
    //         Overlay::ThemeSelector(state) => {
    //             ThemeSelector::default().render(frame.area(), frame.buffer_mut(), state);
    //         }
    //         Overlay::Schema(state) => {
    //             frame.render_stateful_widget(Schema::default(), frame.area(), state);
    //         }
    //         Overlay::Help => {
    //             //
    //             let [area] = Layout::horizontal([Constraint::Length(90)])
    //                 .flex(Flex::Center)
    //                 .areas(frame.area());
    //             let [_, area] =
    //                 Layout::vertical([Constraint::Length(2), Constraint::Length(50)]).areas(area);
    //             Widget::render(HelpModal::new(), area, frame.buffer_mut());
    //         }
    //         Overlay::None => {}
    //     }

    //     Ok(())
    // }
}

impl Component for App {
    fn render(
        &mut self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        focus_state: crate::tui::component::FocusState,
    ) {
        self.tabs.render(area, buf, focus_state);
        match self.overlay.as_mut() {
            Some(Overlay::Error(error)) => {
                error.render(area, buf, focus_state);
            }
            Some(Overlay::CommandPicker(cmd)) => {
                let upmid = {
                    let [mid_ver] = Layout::horizontal([Constraint::Max(80)])
                        .flex(Flex::Center)
                        .areas(area);
                    let [_, mid_hor] =
                        Layout::vertical([Constraint::Length(3), Constraint::Length(15)])
                            .areas(mid_ver);
                    mid_hor
                };
                cmd.render(upmid, buf, focus_state);
            }
            Some(Overlay::ThemeSelector(theme_selector)) => {
                theme_selector.render(area, buf, focus_state);
            }
            Some(Overlay::Schema(schema)) => {
                schema.render(area, buf, focus_state);
            }
            Some(Overlay::Help(help)) => {
                help.render(area, buf, focus_state);
            }
            None => {}
        }
    }

    fn handle(&mut self, event: crossterm::event::KeyEvent) -> bool {
        self.overlay
            .as_mut()
            .map(|overlay| overlay.responder().handle(event))
            .unwrap_or_else(|| self.tabs.handle(event))
            || match event.code {
                KeyCode::Char(':') => {
                    self.show_palette("");
                    true
                }
                KeyCode::Char('Q') => {
                    self.quit();
                    true
                }
                _ => false,
            }
    }

    fn update(&mut self, action: &Message) {
        match action {
            Message::Quit => self.quit(),
            Message::AppDismissOverlay => self.overlay = None,
            Message::AppShowError(message) => self.show_error(message),
            Message::AppShowCommandPicker => self.show_palette(""),
            Message::AppShowThemeSelector => self.show_theme_selector(),
            _ => (),
        };
        if let Some(overlay) = self.overlay.as_mut() {
            overlay.responder().update(action);
        }
        self.tabs.update(action);
    }

    fn tick(&mut self) {
        if let Some(overlay) = self.overlay.as_mut() {
            overlay.responder().tick();
        }
        self.tabs.tick();
    }
}
