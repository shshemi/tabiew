use crate::{
    io::{Resource, reader::JsonLineToDataFrame},
    tui::popups::{
        importers::{
            dismiss_overlay_and_load_data_frame,
            import_source_picker::{ImportSource, ImportSourcePicker},
        },
        multi_step_overlay::OverlayStep,
        path_picker::PathPicker,
    },
};

#[derive(Debug)]
pub enum State {
    PickSource { picker: ImportSourcePicker },
    PickPath { picker: PathPicker },
}

impl OverlayStep for State {
    fn next(self) -> Self {
        match self {
            State::PickSource { picker } => match picker.value() {
                Some(ImportSource::Stdin) => {
                    dismiss_overlay_and_load_data_frame(
                        Resource::Stdin,
                        JsonLineToDataFrame::default(),
                    );
                    State::PickSource { picker }
                }
                Some(ImportSource::File) => State::PickPath {
                    picker: Default::default(),
                },
                None => State::PickSource { picker },
            },
            State::PickPath { picker } => {
                dismiss_overlay_and_load_data_frame(
                    Resource::File(picker.path()),
                    JsonLineToDataFrame::default(),
                );
                Default::default()
            }
        }
    }

    fn responder(&mut self) -> &mut dyn crate::tui::component::Component {
        match self {
            State::PickSource { picker } => picker,
            State::PickPath { picker } => picker,
        }
    }
}

impl Default for State {
    fn default() -> Self {
        State::PickSource {
            picker: Default::default(),
        }
    }
}
