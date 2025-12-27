use std::fmt::Display;

use crossterm::event::{KeyCode, KeyModifiers};
use strum::IntoEnumIterator;
use strum_macros::{EnumIter, IntoStaticStr};

use crate::{
    handler::message::Message,
    misc::{config::config, type_ext::UnwrapOrEnqueueError},
    tui::{component::Component, pickers::search_picker::SearchPicker},
};

#[derive(Debug)]
pub struct CommandPicker {
    picker: SearchPicker<Command>,
}

impl Default for CommandPicker {
    fn default() -> Self {
        Self {
            picker: SearchPicker::new(Command::all()),
        }
    }
}

impl Component for CommandPicker {
    fn render(
        &mut self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        focus_state: crate::tui::component::FocusState,
    ) {
        self.picker.render(area, buf, focus_state);
    }

    fn handle(&mut self, event: crossterm::event::KeyEvent) -> bool {
        if self.picker.handle(event) {
            match self.picker.text() {
                "s " => {
                    Message::AppDismissOverlay.enqueue();
                    Message::PaneShowInlineOrder.enqueue();
                }
                "o " => {
                    Message::AppDismissOverlay.enqueue();
                    Message::PaneShowInlineOrder.enqueue();
                }
                "f " => {
                    Message::AppDismissOverlay.enqueue();
                    Message::PaneShowInlineFilter.enqueue();
                }
                "q " => {
                    Message::AppDismissOverlay.enqueue();
                    Message::PaneShowSqlQuery.enqueue();
                }
                _ => (),
            }
            true
        } else {
            match (event.code, event.modifiers) {
                (KeyCode::Enter, KeyModifiers::NONE) => {
                    Message::AppDismissOverlay.enqueue();
                    if let Some(item) = self.picker.selected_item() {
                        match item {
                            Command::Cast => Message::PaneShowColumnCasterWizard.enqueue(),
                            Command::DataFrameInfo => Message::PaneShowTableInfo.enqueue(),
                            Command::Export => Message::PaneShowExportWizard.enqueue(),
                            Command::Filter => Message::PaneShowInlineFilter.enqueue(),
                            Command::Histogram => Message::PaneShowHistogramWizard.enqueue(),
                            Command::Import => Message::AppShowImportWizard.enqueue(),
                            Command::Order => Message::PaneShowInlineOrder.enqueue(),
                            Command::Query => Message::PaneShowSqlQuery.enqueue(),
                            Command::Quit | Command::Q => Message::Quit.enqueue(),
                            Command::Register => Message::PaneShowTableRegisterer.enqueue(),
                            Command::Scatter => Message::PaneShowScatterPlotWizard.enqueue(),
                            Command::Schema => Message::AppShowSchema.enqueue(),
                            Command::Select => Message::PaneShowInlineSelect.enqueue(),
                            Command::Sort => Message::PaneShowInlineOrder.enqueue(),
                            Command::ThemeSelector => Message::AppShowThemeSelector.enqueue(),
                            Command::ToggleBorders => {
                                config().toggle_show_table_borders();
                                config().store().unwrap_or_enqueue_error();
                            }
                            Command::ToggleRowNumbers => {
                                config().toggle_show_table_row_numbers();
                                config().store().unwrap_or_enqueue_error();
                            }
                        }
                    }
                    true
                }
                (KeyCode::Esc, KeyModifiers::NONE) => {
                    Message::AppDismissOverlay.enqueue();
                    true
                }
                _ => false,
            }
        }
    }
}

#[derive(Debug, Clone, Copy, EnumIter, IntoStaticStr)]
enum Command {
    Cast,
    DataFrameInfo,
    Export,
    Filter,
    Histogram,
    Import,
    Order,
    Q,
    Query,
    Quit,
    Register,
    Scatter,
    Schema,
    Select,
    Sort,
    ThemeSelector,
    ToggleBorders,
    ToggleRowNumbers,
}

impl Command {
    fn all() -> Vec<Command> {
        Command::iter().collect()
    }
}

impl Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Into::<&str>::into(self))
    }
}
