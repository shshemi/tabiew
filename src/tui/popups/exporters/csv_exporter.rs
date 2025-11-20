use std::path::PathBuf;

use crossterm::event::KeyEvent;
use ratatui::widgets::StatefulWidget;

use crate::tui::{
    pickers::text_picker::{TextPicker, TextPicker},
    popups::{
        output_target_picker::{OutputTargetPicker, OutputTargetPickerState, Target},
        path_picker::{PathPicker, PathPickerState},
    },
};

#[derive(Debug)]
pub enum CsvExporterState {
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
        picker: OutputTargetPickerState,
    },
    PickOutputPath {
        separator: char,
        quote: char,
        picker: PathPickerState,
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

impl Default for CsvExporterState {
    fn default() -> Self {
        Self::PickSeparator {
            picker: TextPicker::default()
                .with_max_len(1)
                .with_value(",".to_owned()),
        }
    }
}

// #[derive(Debug, Default)]
// pub struct CsvExporterState {
//     inner: State,
// }

impl CsvExporterState {
    pub fn step(&mut self) {
        *self = match std::mem::take(self) {
            CsvExporterState::PickSeparator { picker } => {
                if let Some(separator) = picker.input().value().chars().next() {
                    CsvExporterState::PickQuoteChar {
                        separator,
                        picker: TextPicker::default()
                            .with_max_len(1)
                            .with_value("\"".to_owned()),
                    }
                } else {
                    CsvExporterState::PickSeparator { picker }
                }
            }
            CsvExporterState::PickQuoteChar { separator, picker } => {
                if let Some(quote) = picker.input().value().chars().next() {
                    CsvExporterState::PickOutputTarget {
                        separator,
                        quote,
                        picker: OutputTargetPickerState::default(),
                    }
                } else {
                    CsvExporterState::PickQuoteChar { separator, picker }
                }
            }
            CsvExporterState::PickOutputTarget {
                separator,
                quote,
                picker,
            } => match picker.selected() {
                Some(Target::File) => CsvExporterState::PickOutputPath {
                    separator,
                    quote,
                    picker: Default::default(),
                },
                Some(Target::Clipboard) => CsvExporterState::ExportToClipboard { separator, quote },
                None => CsvExporterState::PickOutputTarget {
                    separator,
                    quote,
                    picker,
                },
            },
            CsvExporterState::PickOutputPath {
                separator,
                quote,
                picker,
            } => CsvExporterState::ExportToFile {
                separator,
                quote,
                path: picker.path(),
            },
            CsvExporterState::ExportToClipboard { separator, quote } => {
                CsvExporterState::ExportToClipboard { separator, quote }
            }
            CsvExporterState::ExportToFile {
                separator,
                quote,
                path,
            } => CsvExporterState::ExportToFile {
                separator,
                quote,
                path,
            },
        };
    }

    pub fn handle(&mut self, event: KeyEvent) {
        match self {
            CsvExporterState::PickSeparator { picker } => picker.input_mut().handle(event),
            CsvExporterState::PickQuoteChar {
                separator: _,
                picker,
            } => picker.input_mut().handle(event),
            CsvExporterState::PickOutputPath {
                separator: _,
                quote: _,
                picker,
            } => picker.handle(event),
            _ => (),
        }
    }

    pub fn select_previous(&mut self) {
        if let CsvExporterState::PickOutputTarget {
            separator: _,
            quote: _,
            picker,
        } = self
        {
            picker.select_previous();
        }
    }

    pub fn select_next(&mut self) {
        if let CsvExporterState::PickOutputTarget {
            separator: _,
            quote: _,
            picker,
        } = self
        {
            picker.select_next();
        }
    }
}

#[derive(Debug, Default)]
pub struct CsvExporter {}

impl StatefulWidget for CsvExporter {
    type State = CsvExporterState;

    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        state: &mut Self::State,
    ) {
        match state {
            CsvExporterState::PickSeparator { picker } => TextPicker::default()
                .title("Separator")
                .render(area, buf, picker),
            CsvExporterState::PickQuoteChar {
                separator: _,
                picker,
            } => TextPicker::default()
                .title("Quote")
                .render(area, buf, picker),
            CsvExporterState::PickOutputTarget {
                separator: _,
                quote: _,
                picker,
            } => OutputTargetPicker::default().render(area, buf, picker),
            CsvExporterState::PickOutputPath {
                separator: _,
                quote: _,
                picker,
            } => PathPicker::default().render(area, buf, picker),
            CsvExporterState::ExportToFile {
                separator: _,
                quote: _,
                path: _,
            } => (),
            CsvExporterState::ExportToClipboard {
                separator: _,
                quote: _,
            } => (),
        }
    }
}
