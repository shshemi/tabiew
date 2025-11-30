use std::path::PathBuf;

use crate::tui::{
    component::Component,
    popups::{
        exporters::exporter::{Export, Exporter, State},
        output_target_picker::{OutputTargetPicker, Target},
        path_picker::PathPicker,
    },
};

pub type TsvExporter = Exporter<InnerState>;

#[derive(Debug)]
pub enum InnerState {
    PickOutputTarget { picker: OutputTargetPicker },
    PickOutputPath { picker: PathPicker },
    ExportToFile { path: PathBuf },
    ExportToClipboard,
}

impl State for InnerState {
    fn next(self) -> Self {
        match self {
            InnerState::PickOutputTarget { picker } => match picker.selected() {
                Some(Target::File) => InnerState::PickOutputPath {
                    picker: Default::default(),
                },
                Some(Target::Clipboard) => InnerState::ExportToClipboard,
                None => InnerState::PickOutputTarget { picker },
            },
            InnerState::PickOutputPath { picker } => InnerState::ExportToFile {
                path: picker.path(),
            },
            InnerState::ExportToFile { path } => InnerState::ExportToFile { path },
            InnerState::ExportToClipboard => InnerState::ExportToClipboard,
        }
    }

    fn responder(&mut self) -> Option<&mut dyn Component> {
        match self {
            InnerState::PickOutputTarget { picker } => Some(picker),
            InnerState::PickOutputPath { picker } => Some(picker),
            _ => None,
        }
    }

    fn export(&self) -> Export {
        match self {
            InnerState::ExportToFile { path } => Export::TsvToFile(path.to_owned()),
            InnerState::ExportToClipboard => Export::TsvToClipboard,
            _ => Export::WaitingForUserInput,
        }
    }
}

impl Default for InnerState {
    fn default() -> Self {
        Self::PickOutputTarget {
            picker: Default::default(),
        }
    }
}
