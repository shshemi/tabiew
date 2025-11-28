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
pub enum JsonExporterState {
    PickOutputTarget { picker: OutputTargetPicker },
    PickOutputPath { picker: PathPicker },
    ExportToFile { path: PathBuf },
    ExportToClipboard,
}

impl Default for JsonExporterState {
    fn default() -> Self {
        Self::PickOutputTarget {
            picker: Default::default(),
        }
    }
}

// #[derive(Debug, Default)]
// pub struct JsonExporterState {
//     inner: State,
// }

impl JsonExporterState {
    pub fn step(&mut self) {
        *self = match std::mem::take(self) {
            JsonExporterState::PickOutputTarget { picker } => match picker.selected_target() {
                Some(Target::File) => JsonExporterState::PickOutputPath {
                    picker: Default::default(),
                },
                Some(Target::Clipboard) => JsonExporterState::ExportToClipboard,
                None => JsonExporterState::PickOutputTarget { picker },
            },
            JsonExporterState::PickOutputPath { picker } => JsonExporterState::ExportToFile {
                path: picker.path(),
            },
            JsonExporterState::ExportToFile { path } => JsonExporterState::ExportToFile { path },
            JsonExporterState::ExportToClipboard => JsonExporterState::ExportToClipboard,
        };
    }
}

impl Component for JsonExporterState {
    fn render(
        &mut self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        focus_state: crate::tui::component::FocusState,
    ) {
        match self {
            JsonExporterState::PickOutputTarget { picker } => {
                picker.render(area, buf, focus_state);
            }
            JsonExporterState::PickOutputPath { picker } => {
                picker.render(area, buf, focus_state);
            }
            JsonExporterState::ExportToFile { path: _ } => (),
            JsonExporterState::ExportToClipboard => (),
        }
    }

    fn handle(&mut self, event: KeyEvent) -> bool {
        if let JsonExporterState::PickOutputPath { picker } = self {
            picker.handle(event)
        } else {
            false
        }
    }
}

// #[derive(Debug, Default)]
// pub struct JsonExporter {}

// impl StatefulWidget for JsonExporter {
//     type State = JsonExporterState;

//     fn render(
//         self,
//         area: ratatui::prelude::Rect,
//         buf: &mut ratatui::prelude::Buffer,
//         state: &mut Self::State,
//     ) {
//         match state {
//             JsonExporterState::PickOutputTarget { picker } => {
//                 OutputTargetPicker::default().render(area, buf, picker)
//             }
//             JsonExporterState::PickOutputPath { picker } => {
//                 PathPicker::default().render(area, buf, picker)
//             }
//             JsonExporterState::ExportToFile { path: _ } => (),
//             JsonExporterState::ExportToClipboard => (),
//         }
//     }
// }
