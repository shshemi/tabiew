use crate::{
    misc::type_ext::UnwrapOrEnqueueError,
    writer::{Destination, WriteToFile},
};
use polars::frame::DataFrame;

use crate::{
    handler::message::Message,
    tui::{
        component::Component,
        pickers::text_picker::TextPicker,
        popups::{
            output_target_picker::{OutputTargetPicker, Target},
            path_picker::PathPicker,
            wizard::WizardState,
        },
    },
    writer::WriteToCsv,
};

#[derive(Debug)]
pub enum State {
    PickSeparator {
        df: DataFrame,
        picker: TextPicker,
    },
    PickQuoteChar {
        df: DataFrame,
        separator: char,
        picker: TextPicker,
    },
    PickOutputTarget {
        df: DataFrame,
        separator: char,
        quote: char,
        picker: OutputTargetPicker,
    },
    PickOutputPath {
        df: DataFrame,
        separator: char,
        quote: char,
        picker: PathPicker,
    },
}

impl From<DataFrame> for State {
    fn from(value: DataFrame) -> Self {
        Self::PickSeparator {
            df: value,
            picker: TextPicker::default()
                .with_title("Separator")
                .with_max_len(1)
                .with_value(",".to_owned()),
        }
    }
}

impl WizardState for State {
    fn next(self) -> Self {
        match self {
            State::PickSeparator { df, picker } => {
                if let Some(separator) = picker.value().chars().next() {
                    State::PickQuoteChar {
                        df,
                        separator,
                        picker: TextPicker::default()
                            .with_title("Quote")
                            .with_max_len(1)
                            .with_value("\"".to_owned()),
                    }
                } else {
                    State::PickSeparator { df, picker }
                }
            }
            State::PickQuoteChar {
                df,
                separator,
                picker,
            } => {
                if let Some(quote) = picker.value().chars().next() {
                    State::PickOutputTarget {
                        df,
                        separator,
                        quote,
                        picker: OutputTargetPicker::default(),
                    }
                } else {
                    State::PickQuoteChar {
                        df,
                        separator,
                        picker,
                    }
                }
            }
            State::PickOutputTarget {
                mut df,
                separator,
                quote,
                picker,
            } => match picker.selected() {
                Some(Target::Clipboard) => {
                    WriteToCsv::default()
                        .with_separator_char(separator)
                        .with_quote_char(quote)
                        .with_header(true)
                        .write_to_file(Destination::Clipboard, &mut df)
                        .unwrap_or_enqueue_error();
                    Message::PaneDismissModal.enqueue();
                    State::PickOutputTarget {
                        df,
                        separator,
                        quote,
                        picker,
                    }
                }
                Some(Target::File) => State::PickOutputPath {
                    df,
                    separator,
                    quote,
                    picker: PathPicker::default(),
                },
                None => State::PickOutputTarget {
                    df,
                    separator,
                    quote,
                    picker,
                },
            },
            State::PickOutputPath {
                mut df,
                separator,
                quote,
                picker,
            } => {
                WriteToCsv::default()
                    .with_separator_char(separator)
                    .with_quote_char(quote)
                    .with_header(true)
                    .write_to_file(Destination::File(picker.path()), &mut df)
                    .unwrap_or_enqueue_error();
                Message::PaneDismissModal.enqueue();
                State::PickOutputPath {
                    df,
                    separator,
                    quote,
                    picker,
                }
            }
        }
    }

    fn responder(&mut self) -> &mut dyn Component {
        match self {
            State::PickSeparator { picker, df: _ } => picker,
            State::PickQuoteChar {
                separator: _,
                picker,
                df: _,
            } => picker,
            State::PickOutputTarget {
                separator: _,
                quote: _,
                picker,
                df: _,
            } => picker,
            State::PickOutputPath {
                separator: _,
                quote: _,
                picker,
                df: _,
            } => picker,
        }
    }
}
