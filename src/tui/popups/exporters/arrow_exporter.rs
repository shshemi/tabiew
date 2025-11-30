use std::path::PathBuf;

use crate::tui::{
    component::Component,
    popups::{
        exporters::exporter::{Export, Exporter, State},
        path_picker::PathPicker,
    },
};

pub type ArrowExporter = Exporter<InnerState>;

#[derive(Debug)]
pub enum InnerState {
    PickOutputPath { picker: PathPicker },
    ExportToFile { path: PathBuf },
}

impl State for InnerState {
    fn next(self) -> Self {
        if let InnerState::PickOutputPath { picker } = self {
            InnerState::ExportToFile {
                path: picker.path(),
            }
        } else {
            self
        }
    }

    fn responder(&mut self) -> Option<&mut dyn Component> {
        match self {
            InnerState::PickOutputPath { picker } => Some(picker),
            _ => None,
        }
    }

    fn export(&self) -> Export {
        if let InnerState::ExportToFile { path } = self {
            Export::ArrowToFile(path.to_owned())
        } else {
            Export::WaitingForUserInput
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
