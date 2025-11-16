use std::path::PathBuf;

use crossterm::event::KeyEvent;
use ratatui::widgets::StatefulWidget;

use crate::tui::popups::{
    output_target_picker::{OutputTargetPicker, OutputTargetPickerState, Target},
    path_picker::{PathPicker, PathPickerState},
};

#[derive(Debug)]
pub enum TsvExporterState {
    PickOutputTarget { picker: OutputTargetPickerState },
    PickOutputPath { picker: PathPickerState },
    ExportToFile { path: PathBuf },
    ExportToClipboard,
}

impl Default for TsvExporterState {
    fn default() -> Self {
        Self::PickOutputTarget {
            picker: Default::default(),
        }
    }
}

impl TsvExporterState {
    pub fn step(&mut self) {
        *self = match std::mem::take(self) {
            TsvExporterState::PickOutputTarget { picker } => match picker.selected() {
                Some(Target::File) => TsvExporterState::PickOutputPath {
                    picker: Default::default(),
                },
                Some(Target::Clipboard) => TsvExporterState::ExportToClipboard,
                None => TsvExporterState::PickOutputTarget { picker },
            },
            TsvExporterState::PickOutputPath { picker } => TsvExporterState::ExportToFile {
                path: picker.path(),
            },
            TsvExporterState::ExportToFile { path } => TsvExporterState::ExportToFile { path },
            TsvExporterState::ExportToClipboard => TsvExporterState::ExportToClipboard,
        };
    }

    pub fn handle(&mut self, event: KeyEvent) {
        if let TsvExporterState::PickOutputPath { picker } = self {
            picker.handle(event)
        }
    }

    pub fn select_next(&mut self) {
        match self {
            TsvExporterState::PickOutputTarget { picker } => picker.select_next(),
            _ => todo!(),
        }
    }

    pub fn select_previous(&mut self) {
        match self {
            TsvExporterState::PickOutputTarget { picker } => picker.select_previous(),
            _ => todo!(),
        }
    }
}

#[derive(Debug, Default)]
pub struct TsvExporter {}

impl StatefulWidget for TsvExporter {
    type State = TsvExporterState;

    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        state: &mut Self::State,
    ) {
        match state {
            TsvExporterState::PickOutputTarget { picker } => {
                OutputTargetPicker::default().render(area, buf, picker)
            }
            TsvExporterState::PickOutputPath { picker } => {
                PathPicker::default().render(area, buf, picker)
            }
            TsvExporterState::ExportToFile { path: _ } => (),
            TsvExporterState::ExportToClipboard => (),
        }
    }
}
