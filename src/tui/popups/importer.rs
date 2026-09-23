use std::fmt::Display;

use crossterm::event::{KeyCode, KeyModifiers};
use strum::IntoEnumIterator;
use strum_macros::{EnumIter, IntoStaticStr};

use crate::{
    handler::message::Message,
    tui::{component::Component, icons, pickers::search_picker::SearchPicker},
};

#[derive(Debug)]
pub struct Importer {
    picker: SearchPicker<Format>,
}

impl Component for Importer {
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
                    Message::AppDismissOverlay.enqueue();
                    if let Some(format) = self.picker.selected_item() {
                        match format {
                            Format::Arrow => Message::AppShowArrowImporter.enqueue(),
                            Format::Avro => Message::AppShowAvroImporter.enqueue(),
                            Format::Csv => Message::AppShowCsvImporter.enqueue(),
                            Format::Excel => Message::AppShowExcelImporter.enqueue(),
                            Format::Fwf => Message::AppShowFwfImporter.enqueue(),
                            Format::Html => Message::AppShowHtmlImporter.enqueue(),
                            Format::Json => Message::AppShowJsonImporter.enqueue(),
                            Format::Jsonl => Message::AppShowJsonlImporter.enqueue(),
                            Format::Logfmt => Message::AppShowLogfmtImporter.enqueue(),
                            Format::Markdown => Message::AppShowMarkdownImporter.enqueue(),
                            Format::Parquet => Message::AppShowParquetImporter.enqueue(),
                            Format::Sqlite => Message::AppShowSqliteImporter.enqueue(),
                            Format::Tsv => Message::AppShowTsvImporter.enqueue(),
                        }
                    }
                    true
                }
                _ => false,
            }
    }
}

impl Default for Importer {
    fn default() -> Self {
        Self {
            picker: SearchPicker::new(Format::iter().collect())
                .with_title(icons::FORMAT.title("Format")),
        }
    }
}

#[derive(Debug, Clone, Copy, IntoStaticStr, EnumIter)]
pub enum Format {
    Csv,
    Tsv,
    Parquet,
    Jsonl,
    Json,
    Arrow,
    Avro,
    Fwf,
    Sqlite,
    Excel,
    Logfmt,
    Html,
    Markdown,
}

impl Format {
    fn icon(&self) -> icons::Icon {
        match self {
            Format::Csv | Format::Tsv => icons::TABLE,
            Format::Json | Format::Jsonl => icons::JSON,
            Format::Parquet | Format::Arrow | Format::Avro | Format::Sqlite => icons::DATABASE,
            Format::Excel => icons::EXCEL,
            Format::Html => icons::HTML,
            Format::Markdown => icons::MARKDOWN,
            Format::Fwf | Format::Logfmt => icons::TEXT,
        }
    }
}

impl Display for Format {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.icon().item(Into::<&str>::into(self)))
    }
}
