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
pub enum JsonLExporterState {
    PickOutputTarget { picker: OutputTargetPicker },
    PickOutputPath { picker: PathPicker },
    ExportToFile { path: PathBuf },
    ExportToClipboard,
}

impl JsonLExporterState {
    pub fn step(&mut self) {
        *self = match std::mem::take(self) {
            JsonLExporterState::PickOutputTarget { picker } => match picker.selected() {
                Some(Target::File) => JsonLExporterState::PickOutputPath {
                    picker: Default::default(),
                },
                Some(Target::Clipboard) => JsonLExporterState::ExportToClipboard,
                None => JsonLExporterState::PickOutputTarget { picker },
            },
            JsonLExporterState::PickOutputPath { picker } => JsonLExporterState::ExportToFile {
                path: picker.path(),
            },
            JsonLExporterState::ExportToFile { path } => JsonLExporterState::ExportToFile { path },
            JsonLExporterState::ExportToClipboard => JsonLExporterState::ExportToClipboard,
        };
    }

    pub fn select_next(&mut self) {
        if let JsonLExporterState::PickOutputTarget { picker } = self {
            picker.select_next()
        }
    }

    pub fn select_previous(&mut self) {
        if let JsonLExporterState::PickOutputTarget { picker } = self {
            picker.select_previous()
        }
    }
}

impl Component for JsonLExporterState {
    fn render(
        &mut self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        focus_state: crate::tui::component::FocusState,
    ) {
        match self {
            JsonLExporterState::PickOutputTarget { picker } => {
                picker.render(area, buf, focus_state);
            }
            JsonLExporterState::PickOutputPath { picker } => {
                picker.render(area, buf, focus_state);
            }
            JsonLExporterState::ExportToFile { path: _ } => (),
            JsonLExporterState::ExportToClipboard => (),
        }
    }

    fn handle(&mut self, event: KeyEvent) -> bool {
        if let JsonLExporterState::PickOutputPath { picker } = self {
            picker.handle(event)
        } else {
            false
        }
    }
}

impl Default for JsonLExporterState {
    fn default() -> Self {
        JsonLExporterState::PickOutputTarget {
            picker: Default::default(),
        }
    }
}

// #[derive(Debug, Default)]
// pub struct JsonLExporter {}

// impl StatefulWidget for JsonLExporter {
//     type State = JsonLExporterState;

//     fn render(
//         self,
//         area: ratatui::prelude::Rect,
//         buf: &mut ratatui::prelude::Buffer,
//         state: &mut Self::State,
//     ) {
//         match state {
//             JsonLExporterState::PickOutputTarget { picker } => {
//                 OutputTargetPicker::default().render(area, buf, picker)
//             }
//             JsonLExporterState::PickOutputPath { picker } => {
//                 PathPicker::default().render(area, buf, picker)
//             }
//             JsonLExporterState::ExportToFile { path: _ } => (),
//             JsonLExporterState::ExportToClipboard => (),
//         }
//     }
// }
