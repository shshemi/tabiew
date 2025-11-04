use std::path::PathBuf;

use crossterm::event::KeyEvent;
use ratatui::widgets::StatefulWidget;

use crate::tui::{
    pickers::text_picker::{TextPicker, TextPickerState},
    popups::{
        output_target_picker::{OutputTargetPicker, OutputTargetPickerState, Target},
        path_picker::{PathPicker, PathPickerState},
    },
};

#[derive(Debug)]
pub enum State {
    PickSeparator {
        picker: TextPickerState,
    },
    PickQuoteChar {
        separator: char,
        picker: TextPickerState,
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

impl Default for State {
    fn default() -> Self {
        Self::PickSeparator {
            picker: TextPickerState::default()
                .with_max_len(1)
                .with_value(",".to_owned()),
        }
    }
}

#[derive(Debug, Default)]
pub struct CsvExporterState {
    inner: State,
}

impl CsvExporterState {
    pub fn step(&mut self) -> &State {
        self.inner = match std::mem::take(&mut self.inner) {
            State::PickSeparator { picker } => {
                if let Some(separator) = picker.input().value().chars().next() {
                    State::PickQuoteChar {
                        separator,
                        picker: TextPickerState::default()
                            .with_max_len(1)
                            .with_value("\"".to_owned()),
                    }
                } else {
                    State::PickSeparator { picker }
                }
            }
            State::PickQuoteChar { separator, picker } => {
                if let Some(quote) = picker.input().value().chars().next() {
                    State::PickOutputTarget {
                        separator,
                        quote,
                        picker: OutputTargetPickerState::default(),
                    }
                } else {
                    State::PickQuoteChar { separator, picker }
                }
            }
            State::PickOutputTarget {
                separator,
                quote,
                picker,
            } => match picker.selected() {
                Some(Target::File) => State::PickOutputPath {
                    separator,
                    quote,
                    picker: Default::default(),
                },
                Some(Target::Clipboard) => State::ExportToClipboard { separator, quote },
                None => State::PickOutputTarget {
                    separator,
                    quote,
                    picker,
                },
            },
            State::PickOutputPath {
                separator,
                quote,
                picker,
            } => State::ExportToFile {
                separator,
                quote,
                path: picker.path(),
            },
            State::ExportToClipboard { separator, quote } => {
                State::ExportToClipboard { separator, quote }
            }
            State::ExportToFile {
                separator,
                quote,
                path,
            } => State::ExportToFile {
                separator,
                quote,
                path,
            },
        };
        &self.inner
    }

    pub fn inner(&self) -> &State {
        &self.inner
    }

    pub fn handle(&mut self, event: KeyEvent) {
        match &mut self.inner {
            State::PickSeparator { picker } => picker.input_mut().handle(event),
            State::PickQuoteChar {
                separator: _,
                picker,
            } => picker.input_mut().handle(event),
            State::PickOutputPath {
                separator: _,
                quote: _,
                picker,
            } => picker.handle(event),
            _ => (),
        }
    }

    pub fn select_previous(&mut self) {
        if let State::PickOutputTarget {
            separator: _,
            quote: _,
            picker,
        } = &mut self.inner
        {
            picker.select_previous();
        }
    }

    pub fn select_next(&mut self) {
        if let State::PickOutputTarget {
            separator: _,
            quote: _,
            picker,
        } = &mut self.inner
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
        match &mut state.inner {
            State::PickSeparator { picker } => TextPicker::default()
                .title("Separator")
                .render(area, buf, picker),
            State::PickQuoteChar {
                separator: _,
                picker,
            } => TextPicker::default()
                .title("Quote")
                .render(area, buf, picker),
            State::PickOutputTarget {
                separator: _,
                quote: _,
                picker,
            } => OutputTargetPicker::default().render(area, buf, picker),
            State::PickOutputPath {
                separator: _,
                quote: _,
                picker,
            } => PathPicker::default().render(area, buf, picker),
            State::ExportToFile {
                separator: _,
                quote: _,
                path: _,
            } => (),
            State::ExportToClipboard {
                separator: _,
                quote: _,
            } => (),
        }
    }
}
