use crate::{
    handler::message::Message,
    reader::{CsvToDataFrame, Source},
    tui::{
        pickers::text_picker::TextPicker,
        popups::{
            importers::final_step,
            input_source_picker::{self, InputSourcePicker},
            path_picker::PathPicker,
            wizard::WizardState,
        },
    },
};

#[derive(Debug)]
pub enum State {
    PickSource {
        picker: InputSourcePicker,
    },
    PickPath {
        picker: PathPicker,
    },
    PickSeparator {
        source: Source,
        picker: TextPicker,
    },
    PickQuote {
        separator: char,
        source: Source,
        picker: TextPicker,
    },
}

impl WizardState for State {
    fn next(self) -> Self {
        match self {
            State::PickSource { picker } => match picker.value() {
                Some(input_source_picker::Source::Stdin) => State::PickSeparator {
                    source: Source::Stdin,
                    picker: TextPicker::default()
                        .with_title("Separator")
                        .with_max_len(1)
                        .with_value(",".to_owned()),
                },
                Some(input_source_picker::Source::File) => State::PickPath {
                    picker: Default::default(),
                },
                None => State::PickSource {
                    picker: Default::default(),
                },
            },
            State::PickPath { picker } => State::PickSeparator {
                source: Source::File(picker.path()),
                picker: TextPicker::default()
                    .with_title("Separator")
                    .with_max_len(1)
                    .with_value(",".to_owned()),
            },
            State::PickSeparator { source, picker } => {
                if let Some(separator) = picker.value().chars().next() {
                    State::PickQuote {
                        separator,
                        source,
                        picker: TextPicker::default()
                            .with_title("Quote")
                            .with_max_len(1)
                            .with_value("\"".to_owned()),
                    }
                } else {
                    State::PickSeparator { source, picker }
                }
            }
            State::PickQuote {
                separator,
                source,
                picker,
            } => {
                Message::AppDismissOverlay.enqueue();
                if let Some(quote) = picker.value().chars().next() {
                    final_step(
                        source,
                        CsvToDataFrame::default()
                            .with_no_header(false)
                            .with_separator(separator)
                            .with_quote_char(quote),
                    );
                }
                Default::default()
            }
        }
    }

    fn responder(&mut self) -> &mut dyn crate::tui::component::Component {
        match self {
            State::PickSource { picker } => picker,
            State::PickPath { picker } => picker,
            State::PickSeparator { source: _, picker } => picker,
            State::PickQuote {
                separator: _,
                source: _,
                picker,
            } => picker,
        }
    }
}

impl Default for State {
    fn default() -> Self {
        Self::PickSource {
            picker: Default::default(),
        }
    }
}
