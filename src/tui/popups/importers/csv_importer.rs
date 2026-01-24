use crate::{
    handler::message::Message,
    reader::{CsvToDataFrame, Source},
    tui::{
        pickers::text_picker::TextPicker,
        popups::{
            import_source_picker::{self, ImportSourcePicker},
            importers::final_step,
            path_picker::PathPicker,
            step_by_step::StepByStepState,
            yes_no_picker::YesNoPicker,
        },
    },
};

#[derive(Debug)]
pub enum State {
    PickSource {
        picker: ImportSourcePicker,
    },
    PickPath {
        picker: PathPicker,
    },
    PickHasHeader {
        source: Source,
        picker: YesNoPicker,
    },
    PickSeparator {
        source: Source,
        has_header: bool,
        picker: TextPicker,
    },
    PickQuote {
        separator: char,
        source: Source,
        has_header: bool,
        picker: TextPicker,
    },
}

impl StepByStepState for State {
    fn next(self) -> Self {
        match self {
            State::PickSource { picker } => match picker.value() {
                Some(import_source_picker::Source::Stdin) => State::PickHasHeader {
                    source: Source::Stdin,
                    picker: YesNoPicker::default(),
                },
                Some(import_source_picker::Source::File) => State::PickPath {
                    picker: Default::default(),
                },
                None => State::PickSource {
                    picker: Default::default(),
                },
            },
            State::PickPath { picker } => State::PickHasHeader {
                source: Source::File(picker.path()),
                picker: YesNoPicker::default().with_title("Has Header"),
            },
            State::PickHasHeader { source, picker } => State::PickSeparator {
                source,
                has_header: picker.value().unwrap_or(true),
                picker: TextPicker::default()
                    .with_title("Separator")
                    .with_max_len(1)
                    .with_value(",".to_owned()),
            },
            State::PickSeparator {
                source,
                has_header,
                picker,
            } => {
                if let Some(separator) = picker.value().chars().next() {
                    State::PickQuote {
                        separator,
                        source,
                        has_header,
                        picker: TextPicker::default()
                            .with_title("Quote")
                            .with_max_len(1)
                            .with_value("\"".to_owned()),
                    }
                } else {
                    State::PickSeparator {
                        source,
                        has_header,
                        picker,
                    }
                }
            }
            State::PickQuote {
                separator,
                source,
                has_header,
                picker,
            } => {
                Message::AppDismissOverlay.enqueue();
                if let Some(quote) = picker.value().chars().next() {
                    final_step(
                        source,
                        CsvToDataFrame::default()
                            .with_no_header(!has_header)
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
            State::PickHasHeader { source: _, picker } => picker,
            State::PickSeparator {
                source: _,
                has_header: _,
                picker,
            } => picker,
            State::PickQuote {
                separator: _,
                source: _,
                has_header: _,
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
