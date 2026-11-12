use crate::{
    tui::{
        component::Component,
        icons,
        popups::{
            pickers::{
                export_target_picker::{ExportTargetPicker, Target},
                file_picker::FilePicker,
                text_picker::TextPicker,
            },
            wizard::{Wizard, WizardStep},
        },
    },
    writers::{Destination, ExportFormat, export},
};

pub type CsvExporter = Wizard<State>;

#[derive(Debug)]
pub enum State {
    PickSeparator {
        view: String,
        picker: TextPicker,
    },
    PickQuoteChar {
        view: String,
        separator: char,
        picker: TextPicker,
    },
    PickOutputTarget {
        view: String,
        separator: char,
        quote: char,
        picker: ExportTargetPicker,
    },
    PickOutputPath {
        view: String,
        separator: char,
        quote: char,
        picker: FilePicker,
    },
}

impl From<String> for State {
    fn from(value: String) -> Self {
        Self::PickSeparator {
            view: value,
            picker: TextPicker::default()
                .with_title(icons::SEPARATOR.title("Separator"))
                .with_max_len(1)
                .with_value(",".to_owned()),
        }
    }
}

impl WizardStep for State {
    fn next(self) -> Self {
        match self {
            State::PickSeparator { view, picker } => {
                if let Some(separator) = picker.value().chars().next() {
                    State::PickQuoteChar {
                        view,
                        separator,
                        picker: TextPicker::default()
                            .with_title(icons::QUOTE.title("Quote"))
                            .with_max_len(1)
                            .with_value("\"".to_owned()),
                    }
                } else {
                    State::PickSeparator { view, picker }
                }
            }
            State::PickQuoteChar {
                view,
                separator,
                picker,
            } => {
                if let Some(quote) = picker.value().chars().next() {
                    State::PickOutputTarget {
                        view,
                        separator,
                        quote,
                        picker: ExportTargetPicker::default(),
                    }
                } else {
                    State::PickQuoteChar {
                        view,
                        separator,
                        picker,
                    }
                }
            }
            State::PickOutputTarget {
                view,
                separator,
                quote,
                picker,
            } => match picker.selected() {
                Some(Target::Clipboard) => {
                    export(
                        &view,
                        ExportFormat::Csv {
                            separator,
                            quote,
                            header: true,
                        },
                        Destination::Clipboard,
                    );
                    State::PickOutputTarget {
                        view,
                        separator,
                        quote,
                        picker,
                    }
                }
                Some(Target::File) => State::PickOutputPath {
                    view,
                    separator,
                    quote,
                    picker: FilePicker::default(),
                },
                None => State::PickOutputTarget {
                    view,
                    separator,
                    quote,
                    picker,
                },
            },
            State::PickOutputPath {
                view,
                separator,
                quote,
                picker,
            } => {
                export(
                    &view,
                    ExportFormat::Csv {
                        separator,
                        quote,
                        header: true,
                    },
                    Destination::File(picker.path()),
                );
                State::PickOutputPath {
                    view,
                    separator,
                    quote,
                    picker,
                }
            }
        }
    }

    fn responder(&mut self) -> &mut dyn Component {
        match self {
            State::PickSeparator { picker, view: _ } => picker,
            State::PickQuoteChar {
                separator: _,
                picker,
                view: _,
            } => picker,
            State::PickOutputTarget {
                separator: _,
                quote: _,
                picker,
                view: _,
            } => picker,
            State::PickOutputPath {
                separator: _,
                quote: _,
                picker,
                view: _,
            } => picker,
        }
    }
}
