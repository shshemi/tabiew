use std::path::PathBuf;

use crossterm::event::KeyEvent;
use ratatui::widgets::StatefulWidget;

use crate::tui::popups::{
    output_target_picker::{OutputTargetPicker, OutputTargetPickerState, Target},
    path_picker::{PathPicker, PathPickerState},
};

#[derive(Debug)]
pub enum JsonExporterState {
    PickOutputTarget { picker: OutputTargetPickerState },
    PickOutputPath { picker: PathPickerState },
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
            JsonExporterState::PickOutputTarget { picker } => match picker.selected() {
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

    pub fn handle(&mut self, event: KeyEvent) {
        if let JsonExporterState::PickOutputPath { picker } = self {
            picker.handle(event)
        }
    }

    pub fn select_next(&mut self) {
        match self {
            JsonExporterState::PickOutputTarget { picker } => picker.select_next(),
            _ => todo!(),
        }
    }

    pub fn select_previous(&mut self) {
        match self {
            JsonExporterState::PickOutputTarget { picker } => picker.select_previous(),
            _ => todo!(),
        }
    }
}

#[derive(Debug, Default)]
pub struct JsonExporter {}

impl StatefulWidget for JsonExporter {
    type State = JsonExporterState;

    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        state: &mut Self::State,
    ) {
        match state {
            JsonExporterState::PickOutputTarget { picker } => {
                OutputTargetPicker::default().render(area, buf, picker)
            }
            JsonExporterState::PickOutputPath { picker } => {
                PathPicker::default().render(area, buf, picker)
            }
            JsonExporterState::ExportToFile { path: _ } => (),
            JsonExporterState::ExportToClipboard => (),
        }
    }
}
