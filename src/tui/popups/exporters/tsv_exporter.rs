use std::path::PathBuf;

use crossterm::event::KeyEvent;

use crate::tui::{
    component::Component,
    popups::{
        output_target_picker::{OutputTargetPicker, Target},
        path_picker::PathPicker,
    },
};

#[derive(Debug)]
pub enum TsvExporter {
    PickOutputTarget { picker: OutputTargetPicker },
    PickOutputPath { picker: PathPicker },
    ExportToFile { path: PathBuf },
    ExportToClipboard,
}

impl Default for TsvExporter {
    fn default() -> Self {
        Self::PickOutputTarget {
            picker: Default::default(),
        }
    }
}

impl TsvExporter {
    pub fn step(&mut self) {
        *self = match std::mem::take(self) {
            TsvExporter::PickOutputTarget { picker } => match picker.selected_target() {
                Some(Target::File) => TsvExporter::PickOutputPath {
                    picker: Default::default(),
                },
                Some(Target::Clipboard) => TsvExporter::ExportToClipboard,
                None => TsvExporter::PickOutputTarget { picker },
            },
            TsvExporter::PickOutputPath { picker } => TsvExporter::ExportToFile {
                path: picker.path(),
            },
            TsvExporter::ExportToFile { path } => TsvExporter::ExportToFile { path },
            TsvExporter::ExportToClipboard => TsvExporter::ExportToClipboard,
        };
    }
}

impl Component for TsvExporter {
    fn render(
        &mut self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        focus_state: crate::tui::component::FocusState,
    ) {
        match self {
            TsvExporter::PickOutputTarget { picker } => {
                picker.render(area, buf, focus_state);
            }
            TsvExporter::PickOutputPath { picker } => {
                picker.render(area, buf, focus_state);
            }
            TsvExporter::ExportToFile { path: _ } => (),
            TsvExporter::ExportToClipboard => (),
        }
    }

    fn handle(&mut self, event: KeyEvent) -> bool {
        if let TsvExporter::PickOutputPath { picker } = self {
            picker.handle(event)
        } else {
            false
        }
    }
}

// #[derive(Debug, Default)]
// pub struct TsvExporter {}

// impl StatefulWidget for TsvExporter {
//     type State = TsvExporterState;

//     fn render(
//         self,
//         area: ratatui::prelude::Rect,
//         buf: &mut ratatui::prelude::Buffer,
//         state: &mut Self::State,
//     ) {
//         match state {
//             TsvExporterState::PickOutputTarget { picker } => {
//                 OutputTargetPicker::default().render(area, buf, picker)
//             }
//             TsvExporterState::PickOutputPath { picker } => {
//                 PathPicker::default().render(area, buf, picker)
//             }
//             TsvExporterState::ExportToFile { path: _ } => (),
//             TsvExporterState::ExportToClipboard => (),
//         }
//     }
// }
