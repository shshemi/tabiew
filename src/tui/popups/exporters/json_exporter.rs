use std::path::PathBuf;

use crate::{
    handler::action::Action,
    tui::{
        component::Component,
        popups::{
            exporters::exporter::{Exporter, State},
            output_target_picker::{OutputTargetPicker, Target},
            path_picker::PathPicker,
        },
    },
    writer::{Destination, JsonFormat},
};

pub type JsonExporter = Exporter<InnerState>;

#[derive(Debug)]
pub enum InnerState {
    PickOutputTarget { picker: OutputTargetPicker },
    PickOutputPath { picker: PathPicker },
    ExportToFile { path: PathBuf },
    ExportToClipboard,
}

impl State for InnerState {
    fn next(self) -> InnerState {
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
            InnerState::ExportToFile { path: _ } => None,
            InnerState::ExportToClipboard => None,
        }
    }

    fn export_action(&self) -> Option<Action> {
        match self {
            InnerState::ExportToFile { path } => Some(Action::ExportJson(
                Destination::File(path.to_owned()),
                JsonFormat::Json,
            )),
            InnerState::ExportToClipboard => {
                Some(Action::ExportJson(Destination::Clipboard, JsonFormat::Json))
            }
            _ => None,
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
