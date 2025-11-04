use std::path::PathBuf;

use crossterm::event::KeyEvent;
use ratatui::widgets::StatefulWidget;

use crate::tui::popups::path_picker::{PathPicker, PathPickerState};

#[derive(Debug)]
pub enum State {
    PickOutputPath { picker: PathPickerState },
    ExportToFile { path: PathBuf },
}

impl Default for State {
    fn default() -> Self {
        Self::PickOutputPath {
            picker: Default::default(),
        }
    }
}

#[derive(Debug, Default)]
pub struct ArrowExporterState {
    inner: State,
}

impl ArrowExporterState {
    pub fn step(&mut self) -> &State {
        match &mut self.inner {
            State::PickOutputPath { picker } => {
                self.inner = State::ExportToFile {
                    path: picker.path(),
                };
            }
            _ => (),
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
}

#[derive(Debug, Default)]
pub struct ArrowExporter {}

impl StatefulWidget for ArrowExporter {
    type State = ArrowExporterState;

    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        state: &mut Self::State,
    ) {
        match &mut state.inner {
            State::PickOutputPath { picker } => PathPicker::default().render(area, buf, picker),
            State::ExportToFile { path: _ } => (),
        }
    }
}
