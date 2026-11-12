use std::path::PathBuf;

use crate::handler::message::Message;

#[derive(Debug, Clone)]
pub enum Destination {
    File(PathBuf),
    Clipboard,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ExportFormat {
    Csv {
        separator: char,
        quote: char,
        header: bool,
    },
    Json,
    Jsonl,
    Parquet,
    Arrow,
    Avro,
    Markdown,
}

pub fn export(view: &str, format: ExportFormat, dest: Destination) {
    Message::AppExport(view.to_owned(), format, dest).enqueue();
    Message::PaneDismissModal.enqueue();
}

pub(crate) fn escape_cell(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('|', "\\|")
        .replace('\r', "")
        .replace('\n', "<br>")
}
