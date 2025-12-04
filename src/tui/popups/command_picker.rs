use std::fmt::Display;

use crossterm::event::{KeyCode, KeyModifiers};
use strum::IntoEnumIterator;
use strum_macros::{EnumIter, IntoStaticStr};

use crate::{
    handler::message::Message,
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
        match (event.code, event.modifiers) {
            (KeyCode::Enter, KeyModifiers::NONE) => {
                Message::AppDismissOverlay.enqueue();
                if let Some(item) = self.picker.selected_item() {
                    match item {
                        Command::Export => Message::PaneShowExportWizard.enqueue(),
                        Command::Import => (),
                        Command::Order => Message::PaneShowInlineOrder.enqueue(),
                        Command::Sort => Message::PaneShowInlineOrder.enqueue(),
                        Command::Filter => Message::PaneShowInlineFilter.enqueue(),
                        Command::Query => (),
                        Command::ThemeSelector => Message::AppShowThemeSelector.enqueue(),
                        Command::Select => Message::PaneShowInlineSelect.enqueue(),
                        Command::Histogram => Message::PaneShowHistogramWizard.enqueue(),
                        Command::Scatter => Message::PaneShowScatterPlotWizard.enqueue(),
                    }
                }
                true
            }
            (KeyCode::Esc, KeyModifiers::NONE) => {
                Message::AppDismissOverlay.enqueue();
                true
            }
            _ => self.picker.handle(event),
        }
    }
}

#[derive(Debug, Clone, Copy, EnumIter, IntoStaticStr)]
enum Command {
    Export,
    Import,
    Order,
    Sort,
    Filter,
    Query,
    Select,
    ThemeSelector,
    Histogram,
    Scatter,
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
