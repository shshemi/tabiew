use std::path::PathBuf;

use crate::{
    reader::{Source, SqliteToDataFrames},
    tui::{
        pickers::text_picker::TextPicker,
        popups::{importers::final_step, path_picker::PathPicker, wizard::WizardState},
    },
};

#[derive(Debug)]
pub enum State {
    PickPath { picker: PathPicker },
    PickPassword { path: PathBuf, picker: TextPicker },
}

impl WizardState for State {
    fn next(self) -> Self {
        match self {
            State::PickPath { picker } => State::PickPassword {
                path: picker.path(),
                picker: TextPicker::default()
                    .with_title("Password")
                    .with_hint("Leave empty for no password"),
            },
            State::PickPassword { path, picker } => {
                final_step(
                    Source::File(path),
                    if picker.value().is_empty() {
                        SqliteToDataFrames::default()
                    } else {
                        SqliteToDataFrames::default().key(picker.value().to_owned())
                    },
                );
                Default::default()
            }
        }
    }

    fn responder(&mut self) -> &mut dyn crate::tui::component::Component {
        match self {
            State::PickPath { picker } => picker,
            State::PickPassword { path: _, picker } => picker,
        }
    }
}

impl Default for State {
    fn default() -> Self {
        State::PickPath {
            picker: Default::default(),
        }
    }
}
