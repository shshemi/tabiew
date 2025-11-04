use std::path::PathBuf;

use crossterm::event::KeyEvent;
use ratatui::widgets::StatefulWidget;

use crate::tui::popups::{
    output_target_picker::{OutputTargetPicker, OutputTargetPickerState, Target},
    path_picker::{PathPicker, PathPickerState},
};

#[derive(Debug)]
pub enum State {
    PickOutputTarget { picker: OutputTargetPickerState },
    PickOutputPath { picker: PathPickerState },
    ExportToFile { path: PathBuf },
    ExportToClipboard,
}

impl Default for State {
    fn default() -> Self {
        Self::PickOutputTarget {
            picker: Default::default(),
        }
    }
}

#[derive(Debug, Default)]
pub struct TsvExporterState {
    inner: State,
}

impl TsvExporterState {
    pub fn step(&mut self) -> &State {
        self.inner = match std::mem::take(&mut self.inner) {
            State::PickOutputTarget { picker } => match picker.selected() {
                Some(Target::File) => State::PickOutputPath {
                    picker: Default::default(),
                },
                Some(Target::Clipboard) => State::ExportToClipboard,
                None => State::PickOutputTarget { picker },
            },
            State::PickOutputPath { picker } => State::ExportToFile {
                path: picker.path(),
            },
            State::ExportToFile { path } => State::ExportToFile { path },
            State::ExportToClipboard => State::ExportToClipboard,
        };
        &self.inner
    }

    pub fn inner(&self) -> &State {
        &self.inner
    }

    pub fn handle(&mut self, event: KeyEvent) {
        match &mut self.inner {
            State::PickOutputPath { picker } => picker.handle(event),
            _ => (),
        }
    }

    pub fn select_next(&mut self) {
        match &mut self.inner {
            State::PickOutputTarget { picker } => picker.select_next(),
            _ => todo!(),
        }
    }

    pub fn select_previous(&mut self) {
        match &mut self.inner {
            State::PickOutputTarget { picker } => picker.select_previous(),
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
        match &mut state.inner {
            State::PickOutputTarget { picker } => {
                OutputTargetPicker::default().render(area, buf, picker)
            }
            State::PickOutputPath { picker } => PathPicker::default().render(area, buf, picker),
            State::ExportToFile { path: _ } => (),
            State::ExportToClipboard => (),
        }
    }
}
