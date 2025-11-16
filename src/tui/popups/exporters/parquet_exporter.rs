use std::path::PathBuf;

use crossterm::event::KeyEvent;
use ratatui::widgets::StatefulWidget;

use crate::tui::popups::path_picker::{PathPicker, PathPickerState};

#[derive(Debug)]
pub enum ParquetExporterState {
    PickOutputPath { picker: PathPickerState },
    ExportToFile { path: PathBuf },
}

impl Default for ParquetExporterState {
    fn default() -> Self {
        Self::PickOutputPath {
            picker: Default::default(),
        }
    }
}

// #[derive(Debug, Default)]
// pub struct ParquetExporterState {
//     inner: State,
// }

impl ParquetExporterState {
    pub fn step(&mut self) {
        *self = match std::mem::take(self) {
            ParquetExporterState::PickOutputPath { picker } => ParquetExporterState::ExportToFile {
                path: picker.path(),
            },
            ParquetExporterState::ExportToFile { path } => {
                ParquetExporterState::ExportToFile { path }
            }
        };
    }

    pub fn handle(&mut self, event: KeyEvent) {
        if let ParquetExporterState::PickOutputPath { picker } = self {
            picker.handle(event)
        }
    }
}

#[derive(Debug, Default)]
pub struct ParquetExporter {}

impl StatefulWidget for ParquetExporter {
    type State = ParquetExporterState;

    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        state: &mut Self::State,
    ) {
        match state {
            ParquetExporterState::PickOutputPath { picker } => {
                PathPicker::default().render(area, buf, picker)
            }
            ParquetExporterState::ExportToFile { path: _ } => (),
        }
    }
}
