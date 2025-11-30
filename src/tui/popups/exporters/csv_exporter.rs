use std::path::PathBuf;

use crate::tui::popups::exporters::exporter::{Export, Exporter, State};
use crate::tui::{
    component::Component,
    pickers::text_picker::TextPicker,
    popups::{
        output_target_picker::{OutputTargetPicker, Target},
        path_picker::PathPicker,
    },
};

pub type CsvExporter = Exporter<InnerState>;

#[derive(Debug)]
pub enum InnerState {
    PickSeparator {
        picker: TextPicker,
    },
    PickQuoteChar {
        separator: char,
        picker: TextPicker,
    },
    PickOutputTarget {
        separator: char,
        quote: char,
        picker: OutputTargetPicker,
    },
    PickOutputPath {
        separator: char,
        quote: char,
        picker: PathPicker,
    },
    ExportToFile {
        separator: char,
        quote: char,
        path: PathBuf,
    },
    ExportToClipboard {
        separator: char,
        quote: char,
    },
}

impl State for InnerState {
    fn next(self) -> Self {
        match self {
            Self::PickSeparator { picker } => {
                if let Some(separator) = picker.input().value().chars().next() {
                    Self::PickQuoteChar {
                        separator,
                        picker: TextPicker::default()
                            .with_max_len(1)
                            .with_value("\"".to_owned()),
                    }
                } else {
                    Self::PickSeparator { picker }
                }
            }
            Self::PickQuoteChar { separator, picker } => {
                if let Some(quote) = picker.input().value().chars().next() {
                    Self::PickOutputTarget {
                        separator,
                        quote,
                        picker: OutputTargetPicker::default(),
                    }
                } else {
                    Self::PickQuoteChar { separator, picker }
                }
            }
            Self::PickOutputTarget {
                separator,
                quote,
                picker,
            } => match picker.selected() {
                Some(Target::File) => Self::PickOutputPath {
                    separator,
                    quote,
                    picker: Default::default(),
                },
                Some(Target::Clipboard) => Self::ExportToClipboard { separator, quote },
                None => Self::PickOutputTarget {
                    separator,
                    quote,
                    picker,
                },
            },
            Self::PickOutputPath {
                separator,
                quote,
                picker,
            } => Self::ExportToFile {
                separator,
                quote,
                path: picker.path(),
            },
            Self::ExportToClipboard {
                separator: _,
                quote: _,
            } => self,

            Self::ExportToFile {
                separator: _,
                quote: _,
                path: _,
            } => self,
        }
    }

    fn responder(&mut self) -> Option<&mut dyn Component> {
        match self {
            InnerState::PickSeparator { picker } => Some(picker),
            InnerState::PickQuoteChar {
                separator: _,
                picker,
            } => Some(picker),
            InnerState::PickOutputTarget {
                separator: _,
                quote: _,
                picker,
            } => Some(picker),
            InnerState::PickOutputPath {
                separator: _,
                quote: _,
                picker,
            } => Some(picker),
            _ => None,
        }
    }

    fn export(&self) -> Export {
        match &self {
            InnerState::ExportToFile {
                separator,
                quote,
                path,
            } => Export::CsvToFile(*separator, *quote, true, path.to_owned()),
            InnerState::ExportToClipboard { separator, quote } => {
                Export::CsvToClipboard(*separator, *quote, true)
            }
            _ => Export::WaitingForUserInput,
        }
    }
}

impl Default for InnerState {
    fn default() -> Self {
        Self::PickSeparator {
            picker: TextPicker::default()
                .with_max_len(1)
                .with_value(",".to_owned()),
        }
    }
}
