use std::path::PathBuf;

use crate::tui::{
    component::Component,
    popups::{
        exporters::exporter::{Export, Exporter, State},
        path_picker::PathPicker,
    },
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

    fn export(&self) -> Export {
        match self {
            InnerState::ExportToFile { path } => {
                //
                Export::ParquetToFile(path.to_owned())
                // Some(Action::ExportParquet(Destination::File(path.to_owned())))
            }
            _ => Export::WaitingForUserInput,
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
