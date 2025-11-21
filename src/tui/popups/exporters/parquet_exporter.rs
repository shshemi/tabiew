use std::path::PathBuf;

use crossterm::event::KeyEvent;

use crate::tui::{component::Component, popups::path_picker::PathPicker};

#[derive(Debug)]
pub enum ParquetExporterState {
    PickOutputPath { picker: PathPicker },
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
}

impl Component for ParquetExporterState {
    fn render(
        &mut self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        focus_state: crate::tui::component::FocusState,
    ) {
        match self {
            ParquetExporterState::PickOutputPath { picker } => {
                picker.render(area, buf, focus_state);
            }
            ParquetExporterState::ExportToFile { path: _ } => (),
        }
    }

    fn handle(&mut self, event: KeyEvent) -> bool {
        if let ParquetExporterState::PickOutputPath { picker } = self {
            picker.handle(event)
        } else {
            false
        }
    }
}

// #[derive(Debug, Default)]
// pub struct ParquetExporter {}

// impl StatefulWidget for ParquetExporter {
//     type State = ParquetExporterState;

//     fn render(
//         self,
//         area: ratatui::prelude::Rect,
//         buf: &mut ratatui::prelude::Buffer,
//         state: &mut Self::State,
//     ) {
//         match state {
//             ParquetExporterState::PickOutputPath { picker } => {
//                 PathPicker::default().render(area, buf, picker)
//             }
//             ParquetExporterState::ExportToFile { path: _ } => (),
//         }
//     }
// }
