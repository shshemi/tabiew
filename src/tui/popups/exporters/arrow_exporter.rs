use std::path::PathBuf;

use crossterm::event::KeyEvent;

use crate::tui::{component::Component, popups::path_picker::PathPicker};

#[derive(Debug)]
pub enum ArrowExporter {
    PickOutputPath { picker: PathPicker },
    ExportToFile { path: PathBuf },
}

impl ArrowExporter {
    pub fn step(&mut self) {
        if let ArrowExporter::PickOutputPath { picker } = self {
            *self = ArrowExporter::ExportToFile {
                path: picker.path(),
            };
        };
    }
}

impl Component for ArrowExporter {
    fn render(
        &mut self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        focus_state: crate::tui::component::FocusState,
    ) {
        match self {
            ArrowExporter::PickOutputPath { picker } => {
                picker.render(area, buf, focus_state);
            }
            ArrowExporter::ExportToFile { path: _ } => (),
        }
    }

    fn handle(&mut self, event: KeyEvent) -> bool {
        if let ArrowExporter::PickOutputPath { picker } = self {
            picker.handle(event)
        } else {
            false
        }
    }
}

impl Default for ArrowExporter {
    fn default() -> Self {
        Self::PickOutputPath {
            picker: Default::default(),
        }
    }
}

// #[derive(Debug, Default)]
// pub struct ArrowExporter {}

// impl StatefulWidget for ArrowExporter {
//     type State = ArrowExporterState;

//     fn render(
//         self,
//         area: ratatui::prelude::Rect,
//         buf: &mut ratatui::prelude::Buffer,
//         state: &mut Self::State,
//     ) {
//         match state {
//             ArrowExporterState::PickOutputPath { picker } => {
//                 PathPicker::default().render(area, buf, picker)
//             }
//             ArrowExporterState::ExportToFile { path: _ } => (),
//         }
//     }
// }
