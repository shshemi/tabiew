use std::path::PathBuf;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::{
    handler::action::Action,
    tui::{
        component::Component,
        popups::{
            exporters::exporter::{Exporter, State},
            path_picker::PathPicker,
        },
    },
    writer::Destination,
};

pub type ParquetExporter = Exporter<InnerState>;

#[derive(Debug)]
pub enum InnerState {
    PickOutputPath { picker: PathPicker },
    ExportToFile { path: PathBuf },
}

impl State for InnerState {
    fn next(self) -> Self {
        match self {
            InnerState::PickOutputPath { picker } => InnerState::ExportToFile {
                path: picker.path(),
            },
            InnerState::ExportToFile { path } => InnerState::ExportToFile { path },
        }
    }

    fn responder(&mut self) -> Option<&mut dyn Component> {
        match self {
            InnerState::PickOutputPath { picker } => Some(picker),
            _ => None,
        }
    }

    fn export_action(&self) -> Option<Action> {
        match self {
            InnerState::ExportToFile { path } => {
                Some(Action::ExportParquet(Destination::File(path.to_owned())))
            }
            _ => None,
        }
    }
}

impl Default for InnerState {
    fn default() -> Self {
        Self::PickOutputPath {
            picker: Default::default(),
        }
    }
}
