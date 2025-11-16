use std::path::PathBuf;

use crossterm::event::KeyEvent;
use ratatui::widgets::StatefulWidget;

use crate::tui::popups::path_picker::{PathPicker, PathPickerState};

#[derive(Debug)]
pub enum ArrowExporterState {
    PickOutputPath { picker: PathPickerState },
    ExportToFile { path: PathBuf },
}

impl Default for ArrowExporterState {
    fn default() -> Self {
        Self::PickOutputPath {
            picker: Default::default(),
        }
    }
}

impl ArrowExporterState {
    pub fn step(&mut self) {
        if let ArrowExporterState::PickOutputPath { picker } = self {
            *self = ArrowExporterState::ExportToFile {
                path: picker.path(),
            };
        };
    }

    pub fn handle(&mut self, event: KeyEvent) {
        if let ArrowExporterState::PickOutputPath { picker } = self {
            picker.handle(event)
        }
    }
}

#[derive(Debug, Default)]
pub struct ArrowExporter {}

impl StatefulWidget for ArrowExporter {
    type State = ArrowExporterState;

    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        state: &mut Self::State,
    ) {
        match state {
            ArrowExporterState::PickOutputPath { picker } => {
                PathPicker::default().render(area, buf, picker)
            }
            ArrowExporterState::ExportToFile { path: _ } => (),
        }
    }
}
