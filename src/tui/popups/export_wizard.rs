use std::{borrow::Cow, path::PathBuf};

use crossterm::event::KeyEvent;
use ratatui::widgets::StatefulWidget;
use strum::IntoEnumIterator;
use strum_macros::{EnumIter, IntoStaticStr};

use crate::tui::pickers::{
    search_picker::{SearchPicker, SearchPickerState},
    text_picker::{TextPicker, TextPickerState},
};

#[derive(Debug, Default)]
pub struct ExportWizardState {
    export_type: SearchPickerState,
    separator: Option<TextPickerState>,
    quote: Option<TextPickerState>,
    path: Option<TextPickerState>,
}

impl ExportWizardState {
    pub fn next_step(&mut self) -> bool {
        if self.format() == Some(Format::Csv) && self.separator.is_none() {
            self.separator = Some(
                TextPickerState::default()
                    .with_value(",".to_owned())
                    .with_max_len(1),
            );
            false
        } else if self.format() == Some(Format::Csv) && self.quote.is_none() {
            self.quote = Some(
                TextPickerState::default()
                    .with_value("\"".to_owned())
                    .with_max_len(1),
            );
            false
        } else if self.path().is_none() {
            self.path = Some(TextPickerState::default());
            false
        } else {
            true
        }
    }

    pub fn select_previous(&mut self) {
        if self.path.is_none() && self.separator.is_none() && self.quote.is_none() {
            self.export_type.list_mut().select_previous();
        }
    }

    pub fn select_next(&mut self) {
        if self.path.is_none() && self.separator.is_none() && self.quote.is_none() {
            self.export_type.list_mut().select_next();
        }
    }

    pub fn handle(&mut self, event: KeyEvent) {
        if let Some(state) = &mut self.path {
            state.input_mut().handle(event);
        } else if let Some(state) = &mut self.quote {
            state.input_mut().handle(event);
        } else if let Some(state) = &mut self.separator {
            state.input_mut().handle(event);
        } else {
            self.export_type.input_mut().handle(event);
        }
    }

    pub fn format(&self) -> Option<Format> {
        self.export_type.selected().and_then(Format::new)
    }

    pub fn separator(&self) -> Option<char> {
        self.separator
            .as_ref()
            .and_then(|s| s.input().value().chars().next())
    }

    pub fn quote(&self) -> Option<char> {
        self.quote
            .as_ref()
            .and_then(|s| s.input().value().chars().next())
    }

    pub fn path(&self) -> Option<PathBuf> {
        self.path
            .as_ref()
            .map(|s| s.input().value())
            .map(Into::into)
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
        if let Some(state) = &mut state.path {
            TextPicker::default().title("Path").render(area, buf, state);
        } else if let Some(state) = &mut state.quote {
            TextPicker::default()
                .title("Quote")
                .render(area, buf, state);
        } else if let Some(state) = &mut state.separator {
            TextPicker::default()
                .title("Separator")
                .render(area, buf, state);
        } else {
            SearchPicker::default()
                .title("Format")
                .items(Format::iter().map(|e| Cow::Borrowed(e.into())))
                .render(area, buf, &mut state.export_type);
        }
    }
}

#[derive(Debug, IntoStaticStr, EnumIter, PartialEq)]
pub enum Format {
    Csv,
    Tsv,
    Parquet,
    Json,
    JsonL,
    Arrow,
}

impl Format {
    pub fn new(idx: usize) -> Option<Self> {
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
