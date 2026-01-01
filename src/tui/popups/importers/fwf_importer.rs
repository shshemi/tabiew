use crate::{
    reader::{FwfToDataFrame, Source},
    tui::{
        pickers::text_picker::TextPicker,
        popups::{
            import_source_picker::{self, ImportSourcePicker},
            importers::final_step,
            path_picker::PathPicker,
            wizard::WizardState,
            yes_no_picker::YesNoPicker,
        },
        widgets::input::InputType,
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
    PickWidths {
        source: Source,
        picker: TextPicker,
    },
    PickHeader {
        widths: Vec<usize>,
        source: Source,
        picker: YesNoPicker,
    },
    PickSeparatorLength {
        has_header: bool,
        widths: Vec<usize>,
        source: Source,
        picker: TextPicker,
    },
    PickFlexibleWidth {
        separator_length: usize,
        has_header: bool,
        widths: Vec<usize>,
        source: Source,
        picker: YesNoPicker,
    },
}

impl WizardState for State {
    fn next(self) -> Self {
        match self {
            State::PickSource { picker } => match picker.value() {
                Some(import_source_picker::Source::File) => State::PickPath {
                    picker: PathPicker::default(),
                },
                Some(import_source_picker::Source::Stdin) => State::PickWidths {
                    source: Source::Stdin,
                    picker: TextPicker::default()
                        .with_input_type(InputType::MultiNumeric)
                        .with_title("Widths")
                        .with_hint("4 8 12 or leave empty to auto detect"),
                },
                None => State::PickSource { picker },
            },
            State::PickPath { picker } => State::PickWidths {
                source: Source::File(picker.path()),
                picker: TextPicker::default()
                    .with_input_type(InputType::MultiNumeric)
                    .with_title("Widths")
                    .with_hint("4 8 12 or leave empty to auto detect"),
            },
            State::PickWidths { source, picker } => {
                let widths = picker
                    .value()
                    .split(' ')
                    .map(|s| s.parse())
                    .collect::<Result<_, _>>()
                    .unwrap_or_default();
                State::PickHeader {
                    widths,
                    source,
                    picker: YesNoPicker::default().with_title("Has Header"),
                }
            }
            State::PickHeader {
                widths,
                source,
                picker,
            } => match picker.value() {
                Some(has_header) => State::PickSeparatorLength {
                    has_header,
                    widths,
                    source,
                    picker: TextPicker::default()
                        .with_input_type(InputType::Numeric)
                        .with_title("Separator Legnth"),
                },
                None => State::PickHeader {
                    widths,
                    source,
                    picker,
                },
            },
            State::PickSeparatorLength {
                has_header,
                widths,
                source,
                picker,
            } => {
                let separator_length = picker.value().parse().unwrap_or(0);
                State::PickFlexibleWidth {
                    separator_length,
                    has_header,
                    widths,
                    source,
                    picker: YesNoPicker::default().with_title("Flexible Width"),
                }
            }
            State::PickFlexibleWidth {
                separator_length,
                has_header,
                widths,
                source,
                picker,
            } => {
                let flexible_width = picker.value().unwrap_or(true);
                let rtdf = FwfToDataFrame::default()
                    .with_flexible_width(flexible_width)
                    .with_has_header(has_header)
                    .with_separator_length(separator_length)
                    .with_widths(widths);
                final_step(source, rtdf);
                Default::default()
            }
        }
    }

    fn responder(&mut self) -> &mut dyn crate::tui::component::Component {
        match self {
            State::PickSource { picker } => picker,
            State::PickPath { picker } => picker,
            State::PickWidths { source: _, picker } => picker,
            State::PickHeader {
                widths: _,
                source: _,
                picker,
            } => picker,
            State::PickSeparatorLength {
                has_header: _,
                widths: _,
                source: _,
                picker,
            } => picker,
            State::PickFlexibleWidth {
                separator_length: _,
                has_header: _,
                widths: _,
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
