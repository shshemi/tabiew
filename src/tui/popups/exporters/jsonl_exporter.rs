use std::path::PathBuf;

use crossterm::event::KeyEvent;
use ratatui::widgets::StatefulWidget;

use crate::tui::popups::{
    output_target_picker::{OutputTargetPicker, OutputTargetPickerState, Target},
    path_picker::{PathPicker, PathPickerState},
};

#[derive(Debug)]
pub enum JsonLExporterState {
    PickOutputTarget { picker: OutputTargetPickerState },
    PickOutputPath { picker: PathPickerState },
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

    pub fn handle(&mut self, event: KeyEvent) {
        if let JsonLExporterState::PickOutputPath { picker } = self {
            picker.handle(event)
        }
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

impl Default for JsonLExporterState {
    fn default() -> Self {
        JsonLExporterState::PickOutputTarget {
            picker: Default::default(),
        }
    }
}

#[derive(Debug, Default)]
pub struct JsonLExporter {}

impl StatefulWidget for JsonLExporter {
    type State = JsonLExporterState;

    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        state: &mut Self::State,
    ) {
        match state {
            JsonLExporterState::PickOutputTarget { picker } => {
                OutputTargetPicker::default().render(area, buf, picker)
            }
            JsonLExporterState::PickOutputPath { picker } => {
                PathPicker::default().render(area, buf, picker)
            }
            JsonLExporterState::ExportToFile { path: _ } => (),
            JsonLExporterState::ExportToClipboard => (),
        }
    }
}
