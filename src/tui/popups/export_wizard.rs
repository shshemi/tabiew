use std::{borrow::Cow, path::PathBuf};

use crossterm::event::KeyEvent;
use ratatui::widgets::StatefulWidget;
use strum::IntoEnumIterator;
use strum_macros::{EnumIter, IntoStaticStr};

use crate::tui::pickers::{
    list_picker::{ListPicker, ListPickerState},
    search_picker::{SearchPicker, SearchPickerState},
    text_picker::{TextPicker, TextPickerState},
};

#[derive(Debug, Default)]
pub struct ExportWizardState {
    state: InnerState,
}

impl ExportWizardState {
    pub fn next_step(&mut self) -> &InnerState {
        self.state = match std::mem::take(&mut self.state) {
            InnerState::None { picker } => match picker.list().selected().and_then(Format::new) {
                Some(Format::Csv) => InnerState::CsvSeparator {
                    picker: TextPickerState::default()
                        .with_max_len(1)
                        .with_value(",".to_owned()),
                },
                Some(Format::Tsv) => InnerState::TsvDestination {
                    picker: Default::default(),
                },
                Some(Format::Parquet) => InnerState::ParquetPath {
                    picker: Default::default(),
                },
                Some(Format::Json) => InnerState::JsonDestination {
                    picker: Default::default(),
                },
                Some(Format::JsonL) => InnerState::JsonLDestination {
                    picker: Default::default(),
                },
                Some(Format::Arrow) => InnerState::ArrowPath {
                    picker: Default::default(),
                },
                None => InnerState::None { picker },
            },
            InnerState::CsvSeparator { picker } => {
                if let Some(sep) = picker.input().value().chars().next() {
                    InnerState::CsvQuote {
                        sep,
                        picker: TextPickerState::default()
                            .with_max_len(1)
                            .with_value("\"".to_owned()),
                    }
                } else {
                    InnerState::CsvSeparator { picker }
                }
            }
            InnerState::CsvQuote { picker, sep } => {
                if let Some(quote) = picker.input().value().chars().next() {
                    InnerState::CsvDestination {
                        sep,
                        quote,
                        picker: Default::default(),
                    }
                } else {
                    InnerState::CsvSeparator { picker }
                }
            }
            InnerState::CsvDestination { picker, sep, quote } => {
                match picker.list().selected().and_then(Destination::new) {
                    Some(Destination::File) => InnerState::CsvPath {
                        picker: Default::default(),
                        sep,
                        quote,
                    },
                    Some(Destination::Clipboard) => InnerState::CsvExportClipboard { sep, quote },
                    None => InnerState::CsvDestination { picker, sep, quote },
                }
            }
            InnerState::CsvPath { picker, sep, quote } => InnerState::CsvExportFile {
                sep,
                quote,
                path: picker.input().value().into(),
            },
            InnerState::CsvExportFile { sep, quote, path } => {
                InnerState::CsvExportFile { sep, quote, path }
            }
            InnerState::CsvExportClipboard { sep, quote } => {
                InnerState::CsvExportClipboard { sep, quote }
            }
            InnerState::TsvDestination { picker } => {
                match picker.list().selected().and_then(Destination::new) {
                    Some(Destination::File) => InnerState::TsvPath {
                        picker: Default::default(),
                    },
                    Some(Destination::Clipboard) => InnerState::TsvExportClipbaord,
                    None => InnerState::TsvDestination { picker },
                }
            }
            InnerState::TsvPath { picker } => InnerState::TsvExportFile {
                path: picker.input().value().into(),
            },
            InnerState::TsvExportFile { path } => InnerState::TsvExportFile { path },
            InnerState::TsvExportClipbaord => InnerState::TsvExportClipbaord,
            InnerState::ParquetPath { picker } => InnerState::ParquetExportFile {
                path: picker.input().value().into(),
            },
            InnerState::ParquetExportFile { path } => InnerState::ParquetExportFile { path },
            InnerState::JsonDestination { picker } => {
                match picker.list().selected().and_then(Destination::new) {
                    Some(Destination::File) => InnerState::JsonPath {
                        picker: Default::default(),
                    },
                    Some(Destination::Clipboard) => InnerState::JsonExportClipboard,
                    None => InnerState::JsonDestination { picker },
                }
            }
            InnerState::JsonPath { picker } => InnerState::JsonExportFile {
                path: picker.input().value().into(),
            },
            InnerState::JsonExportFile { path } => InnerState::JsonLExportFile { path },
            InnerState::JsonExportClipboard => InnerState::JsonExportClipboard,
            InnerState::JsonLDestination { picker } => {
                match picker.list().selected().and_then(Destination::new) {
                    Some(Destination::File) => InnerState::JsonLPath {
                        picker: Default::default(),
                    },
                    Some(Destination::Clipboard) => InnerState::JsonLExportClipboard,
                    None => InnerState::JsonDestination { picker },
                }
            }
            InnerState::JsonLPath { picker } => InnerState::JsonLExportFile {
                path: picker.input().value().into(),
            },
            InnerState::JsonLExportFile { path } => InnerState::JsonLExportFile { path },
            InnerState::JsonLExportClipboard => InnerState::JsonLExportClipboard,
            InnerState::ArrowPath { picker } => InnerState::ArrowExportFile {
                path: picker.input().value().into(),
            },
            InnerState::ArrowExportFile { path } => InnerState::ArrowExportFile { path },
        };
        &self.state
    }

    pub fn select_previous(&mut self) {
        match &mut self.state {
            InnerState::None { picker } => {
                picker.list_mut().select_previous();
            }
            InnerState::CsvDestination {
                picker,
                sep: _,
                quote: _,
            } => {
                picker.list_mut().select_previous();
            }
            InnerState::TsvDestination { picker } => {
                picker.list_mut().select_previous();
            }
            InnerState::JsonDestination { picker } => {
                picker.list_mut().select_previous();
            }
            InnerState::JsonLDestination { picker } => {
                picker.list_mut().select_previous();
            }
            _ => (),
        };
    }

    pub fn select_next(&mut self) {
        match &mut self.state {
            InnerState::None { picker } => {
                picker.list_mut().select_next();
            }
            InnerState::CsvDestination {
                picker,
                sep: _,
                quote: _,
            } => {
                picker.list_mut().select_next();
            }
            InnerState::TsvDestination { picker } => {
                picker.list_mut().select_next();
            }
            InnerState::JsonDestination { picker } => {
                picker.list_mut().select_next();
            }
            InnerState::JsonLDestination { picker } => {
                picker.list_mut().select_next();
            }
            _ => (),
        };
    }

    pub fn handle(&mut self, event: KeyEvent) {
        match &mut self.state {
            InnerState::None { picker } => {
                picker.input_mut().handle(event);
            }
            InnerState::CsvSeparator { picker } => {
                picker.input_mut().handle(event);
            }
            InnerState::CsvQuote { picker, sep: _ } => {
                picker.input_mut().handle(event);
            }
            InnerState::CsvPath {
                picker,
                sep: _,
                quote: _,
            } => {
                picker.input_mut().handle(event);
            }
            InnerState::TsvPath { picker } => {
                picker.input_mut().handle(event);
            }
            InnerState::ParquetPath { picker } => {
                picker.input_mut().handle(event);
            }
            InnerState::JsonPath { picker } => {
                picker.input_mut().handle(event);
            }
            InnerState::JsonLPath { picker } => {
                picker.input_mut().handle(event);
            }
            InnerState::ArrowPath { picker } => {
                picker.input_mut().handle(event);
            }
            _ => (),
        }
    }
}

#[derive(Debug, Default)]
pub struct ExportWizard {}

impl StatefulWidget for ExportWizard {
    type State = ExportWizardState;

    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        state: &mut Self::State,
    ) {
        match &mut state.state {
            InnerState::None { picker } => {
                SearchPicker::default()
                    .title("Format")
                    .items(Format::iter().map(|f| Cow::Borrowed(f.into())))
                    .render(area, buf, picker);
            }
            InnerState::CsvSeparator { picker } => {
                TextPicker::default()
                    .title("Separator")
                    .render(area, buf, picker);
            }
            InnerState::CsvQuote { picker, sep: _ } => {
                TextPicker::default()
                    .title("Quote")
                    .render(area, buf, picker);
            }
            InnerState::CsvDestination {
                picker,
                sep: _,
                quote: _,
            } => {
                ListPicker::default()
                    .title("Destination")
                    .items(Destination::iter().map(|d| Cow::Borrowed(d.into())))
                    .render(area, buf, picker);
            }
            InnerState::CsvPath {
                picker,
                sep: _,
                quote: _,
            } => {
                TextPicker::default()
                    .title("Path")
                    .render(area, buf, picker);
            }
            InnerState::TsvDestination { picker } => {
                ListPicker::default()
                    .title("Destination")
                    .items(Destination::iter().map(|d| Cow::Borrowed(d.into())))
                    .render(area, buf, picker);
            }
            InnerState::TsvPath { picker } => {
                TextPicker::default()
                    .title("Path")
                    .render(area, buf, picker);
            }
            InnerState::ParquetPath { picker } => {
                TextPicker::default()
                    .title("Path")
                    .render(area, buf, picker);
            }
            InnerState::JsonDestination { picker } => {
                ListPicker::default()
                    .title("Destination")
                    .items(Destination::iter().map(|d| Cow::Borrowed(d.into())))
                    .render(area, buf, picker);
            }
            InnerState::JsonPath { picker } => {
                TextPicker::default()
                    .title("Path")
                    .render(area, buf, picker);
            }
            InnerState::JsonLDestination { picker } => {
                ListPicker::default()
                    .title("Destination")
                    .items(Destination::iter().map(|d| Cow::Borrowed(d.into())))
                    .render(area, buf, picker);
            }
            InnerState::JsonLPath { picker } => {
                TextPicker::default()
                    .title("Path")
                    .render(area, buf, picker);
            }
            InnerState::ArrowPath { picker } => {
                TextPicker::default()
                    .title("Path")
                    .render(area, buf, picker);
            }
            _ => (),
        }
    }
}

#[derive(Debug)]
pub enum InnerState {
    None {
        picker: SearchPickerState,
    },
    CsvSeparator {
        picker: TextPickerState,
    },
    CsvQuote {
        picker: TextPickerState,
        sep: char,
    },
    CsvDestination {
        picker: ListPickerState,
        sep: char,
        quote: char,
    },
    CsvPath {
        picker: TextPickerState,
        sep: char,
        quote: char,
    },
    CsvExportFile {
        sep: char,
        quote: char,
        path: PathBuf,
    },
    CsvExportClipboard {
        sep: char,
        quote: char,
    },
    TsvDestination {
        picker: ListPickerState,
    },
    TsvPath {
        picker: TextPickerState,
    },
    TsvExportFile {
        path: PathBuf,
    },
    TsvExportClipbaord,
    ParquetPath {
        picker: TextPickerState,
    },
    ParquetExportFile {
        path: PathBuf,
    },
    JsonDestination {
        picker: ListPickerState,
    },
    JsonPath {
        picker: TextPickerState,
    },
    JsonExportFile {
        path: PathBuf,
    },
    JsonExportClipboard,
    JsonLDestination {
        picker: ListPickerState,
    },
    JsonLPath {
        picker: TextPickerState,
    },
    JsonLExportFile {
        path: PathBuf,
    },
    JsonLExportClipboard,
    ArrowPath {
        picker: TextPickerState,
    },
    ArrowExportFile {
        path: PathBuf,
    },
}

impl Default for InnerState {
    fn default() -> Self {
        InnerState::None {
            picker: Default::default(),
        }
    }
}

#[derive(Debug, IntoStaticStr, EnumIter, PartialEq)]
enum Destination {
    File,
    Clipboard,
}

impl Destination {
    fn new(idx: usize) -> Option<Self> {
        match idx {
            0 => Some(Self::File),
            1 => Some(Self::Clipboard),
            _ => None,
        }
    }
}

#[derive(Debug, IntoStaticStr, EnumIter, PartialEq)]
enum Format {
    Csv,
    Tsv,
    Parquet,
    Json,
    JsonL,
    Arrow,
}

impl Format {
    fn new(idx: usize) -> Option<Self> {
        match idx {
            0 => Some(Self::Csv),
            1 => Some(Self::Tsv),
            2 => Some(Self::Parquet),
            3 => Some(Self::Json),
            4 => Some(Self::JsonL),
            5 => Some(Self::Arrow),
            _ => None,
        }
    }
}
