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
            InnerState::None { picker } => {
                match picker.list().selected().and_then(OutputFormat::new) {
                    Some(OutputFormat::Csv) => InnerState::CsvSeparator {
                        picker: TextPickerState::default()
                            .with_max_len(1)
                            .with_value(",".to_owned()),
                    },
                    Some(OutputFormat::Tsv) => InnerState::TsvOutputTarget {
                        picker: Default::default(),
                    },
                    Some(OutputFormat::Parquet) => InnerState::ParquetPath {
                        picker: Default::default(),
                    },
                    Some(OutputFormat::Json) => InnerState::JsonOutputTarget {
                        picker: Default::default(),
                    },
                    Some(OutputFormat::JsonL) => InnerState::JsonLOutputTarget {
                        picker: Default::default(),
                    },
                    Some(OutputFormat::Arrow) => InnerState::ArrowPath {
                        picker: Default::default(),
                    },
                    None => InnerState::None { picker },
                }
            }
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
                    InnerState::CsvOutputTarget {
                        sep,
                        quote,
                        picker: Default::default(),
                    }
                } else {
                    InnerState::CsvSeparator { picker }
                }
            }
            InnerState::CsvOutputTarget { picker, sep, quote } => {
                match picker.list().selected().and_then(OutputTarget::new) {
                    Some(OutputTarget::File) => InnerState::CsvPath {
                        picker: Default::default(),
                        sep,
                        quote,
                    },
                    Some(OutputTarget::Clipboard) => InnerState::CsvExportClipboard { sep, quote },
                    None => InnerState::CsvOutputTarget { picker, sep, quote },
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
            InnerState::TsvOutputTarget { picker } => {
                match picker.list().selected().and_then(OutputTarget::new) {
                    Some(OutputTarget::File) => InnerState::TsvPath {
                        picker: Default::default(),
                    },
                    Some(OutputTarget::Clipboard) => InnerState::TsvExportClipbaord,
                    None => InnerState::TsvOutputTarget { picker },
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
            InnerState::JsonOutputTarget { picker } => {
                match picker.list().selected().and_then(OutputTarget::new) {
                    Some(OutputTarget::File) => InnerState::JsonPath {
                        picker: Default::default(),
                    },
                    Some(OutputTarget::Clipboard) => InnerState::JsonExportClipboard,
                    None => InnerState::JsonOutputTarget { picker },
                }
            }
            InnerState::JsonPath { picker } => InnerState::JsonExportFile {
                path: picker.input().value().into(),
            },
            InnerState::JsonExportFile { path } => InnerState::JsonLExportFile { path },
            InnerState::JsonExportClipboard => InnerState::JsonExportClipboard,
            InnerState::JsonLOutputTarget { picker } => {
                match picker.list().selected().and_then(OutputTarget::new) {
                    Some(OutputTarget::File) => InnerState::JsonLPath {
                        picker: Default::default(),
                    },
                    Some(OutputTarget::Clipboard) => InnerState::JsonLExportClipboard,
                    None => InnerState::JsonOutputTarget { picker },
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
            InnerState::CsvOutputTarget {
                picker,
                sep: _,
                quote: _,
            } => {
                picker.list_mut().select_previous();
            }
            InnerState::TsvOutputTarget { picker } => {
                picker.list_mut().select_previous();
            }
            InnerState::JsonOutputTarget { picker } => {
                picker.list_mut().select_previous();
            }
            InnerState::JsonLOutputTarget { picker } => {
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
            InnerState::CsvOutputTarget {
                picker,
                sep: _,
                quote: _,
            } => {
                picker.list_mut().select_next();
            }
            InnerState::TsvOutputTarget { picker } => {
                picker.list_mut().select_next();
            }
            InnerState::JsonOutputTarget { picker } => {
                picker.list_mut().select_next();
            }
            InnerState::JsonLOutputTarget { picker } => {
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
                    .items(OutputFormat::iter().map(|f| Cow::Borrowed(f.into())))
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
            InnerState::CsvOutputTarget {
                picker,
                sep: _,
                quote: _,
            } => {
                ListPicker::default()
                    .title("Output Target")
                    .items(OutputTarget::iter().map(|d| Cow::Borrowed(d.into())))
                    .render(area, buf, picker);
            }
            InnerState::CsvPath {
                picker,
                sep: _,
                quote: _,
            } => {
                TextPicker::default()
                    .title("File Path")
                    .render(area, buf, picker);
            }
            InnerState::TsvOutputTarget { picker } => {
                ListPicker::default()
                    .title("Ouput Target")
                    .items(OutputTarget::iter().map(|d| Cow::Borrowed(d.into())))
                    .render(area, buf, picker);
            }
            InnerState::TsvPath { picker } => {
                TextPicker::default()
                    .title("File Path")
                    .render(area, buf, picker);
            }
            InnerState::ParquetPath { picker } => {
                TextPicker::default()
                    .title("File Path")
                    .render(area, buf, picker);
            }
            InnerState::JsonOutputTarget { picker } => {
                ListPicker::default()
                    .title("Output Target")
                    .items(OutputTarget::iter().map(|d| Cow::Borrowed(d.into())))
                    .render(area, buf, picker);
            }
            InnerState::JsonPath { picker } => {
                TextPicker::default()
                    .title("File Path")
                    .render(area, buf, picker);
            }
            InnerState::JsonLOutputTarget { picker } => {
                ListPicker::default()
                    .title("Output Target")
                    .items(OutputTarget::iter().map(|d| Cow::Borrowed(d.into())))
                    .render(area, buf, picker);
            }
            InnerState::JsonLPath { picker } => {
                TextPicker::default()
                    .title("File Path")
                    .render(area, buf, picker);
            }
            InnerState::ArrowPath { picker } => {
                TextPicker::default()
                    .title("File Path")
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
    CsvOutputTarget {
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
    TsvOutputTarget {
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
    JsonOutputTarget {
        picker: ListPickerState,
    },
    JsonPath {
        picker: TextPickerState,
    },
    JsonExportFile {
        path: PathBuf,
    },
    JsonExportClipboard,
    JsonLOutputTarget {
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
enum OutputTarget {
    File,
    Clipboard,
}

impl OutputTarget {
    fn new(idx: usize) -> Option<Self> {
        match idx {
            0 => Some(Self::File),
            1 => Some(Self::Clipboard),
            _ => None,
        }
    }
}

#[derive(Debug, IntoStaticStr, EnumIter, PartialEq)]
enum OutputFormat {
    Csv,
    Tsv,
    Parquet,
    Json,
    JsonL,
    Arrow,
}

impl OutputFormat {
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
