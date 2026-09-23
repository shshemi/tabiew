use std::fmt::Display;

use crossterm::event::{KeyCode, KeyModifiers};
use strum::IntoEnumIterator;
use strum_macros::{EnumIter, IntoStaticStr};

use crate::{
    handler::message::Message,
    tui::{component::Component, icons, pickers::search_picker::SearchPicker},
};

#[derive(Debug)]
pub struct Exporter {
    picker: SearchPicker<Format>,
}

impl Component for Exporter {
    fn render(
        &mut self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        focus_state: crate::tui::component::FocusState,
    ) {
        self.picker.render(area, buf, focus_state);
    }

    fn handle(&mut self, event: crossterm::event::KeyEvent) -> bool {
        self.picker.handle(event)
            || match (event.code, event.modifiers) {
                (KeyCode::Esc, KeyModifiers::NONE) => {
                    Message::PaneDismissModal.enqueue();
                    Message::AppDismissOverlay.enqueue();
                    true
                }
                (KeyCode::Enter, KeyModifiers::NONE) => {
                    Message::PaneDismissModal.enqueue();
                    if let Some(format) = self.picker.selected_item() {
                        match format {
                            Format::Arrow => Message::PaneShowArrowExporter.enqueue(),
                            Format::Avro => Message::PaneShowAvroExporter.enqueue(),
                            Format::Csv => Message::PaneShowCsvExporter.enqueue(),
                            Format::Json => Message::PaneShowJsonExporter.enqueue(),
                            Format::JsonL => Message::PaneShowJsonlExporter.enqueue(),
                            Format::Markdown => Message::PaneShowMarkdownExporter.enqueue(),
                            Format::Parquet => Message::PaneShowParquetExporter.enqueue(),
                            Format::Tsv => Message::PaneShowTsvExporter.enqueue(),
                        }
                    }
                    true
                }
                _ => false,
            }
    }
}

impl Default for Exporter {
    fn default() -> Self {
        Self {
            picker: SearchPicker::new(Format::iter().collect())
                .with_title(icons::FORMAT.title("Format")),
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
    Markdown,
    Arrow,
    Avro,
}

impl Format {
    fn icon(&self) -> icons::Icon {
        match self {
            Format::Csv | Format::Tsv => icons::TABLE,
            Format::Json | Format::JsonL => icons::JSON,
            Format::Parquet | Format::Arrow | Format::Avro => icons::DATABASE,
            Format::Markdown => icons::MARKDOWN,
        }
    }
}

impl Display for Format {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.icon().item(Into::<&str>::into(self)))
    }
}
