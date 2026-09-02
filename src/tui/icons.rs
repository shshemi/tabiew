use std::fmt::Display;

use crate::misc::config::config;

#[derive(Debug, Clone, Copy)]
pub struct Icon(&'static str);

impl Icon {
    pub fn into_str(self, fallback: &'static str) -> &'static str {
        if config().use_nerd_font() {
            self.0
        } else {
            fallback
        }
    }

    pub fn into_title(self, text: impl Display) -> String {
        if config().use_nerd_font() {
            format!("{}  {text}", self.0)
        } else {
            text.to_string()
        }
    }

    pub fn into_item(self, text: impl Display) -> String {
        if config().use_nerd_font() {
            format!(" {}  {text}", self.0)
        } else {
            text.to_string()
        }
    }
}

pub const FILE: Icon = Icon("\u{F15B}");
pub const FORMAT: Icon = Icon("\u{F016}");
pub const FOLDER: Icon = Icon("\u{F07B}");
pub const FOLDER_OPEN: Icon = Icon("\u{F07C}");
pub const TABLE: Icon = Icon("\u{F0CE}");
pub const JSON: Icon = Icon("\u{E60B}");
pub const DATABASE: Icon = Icon("\u{F1C0}");
pub const EXCEL: Icon = Icon("\u{F1C3}");
pub const HTML: Icon = Icon("\u{F121}");
pub const MARKDOWN: Icon = Icon("\u{EB1D}");
pub const TEXT: Icon = Icon("\u{F0F6}");
pub const TERMINAL: Icon = Icon("\u{F120}");
pub const GLOBE: Icon = Icon("\u{F0AC}");
pub const CHECK: Icon = Icon("\u{F00C}");
pub const CROSS: Icon = Icon("\u{F00D}");
pub const IMPORT: Icon = Icon("\u{F090}");
pub const EXPORT: Icon = Icon("\u{F08B}");
pub const CLIPBOARD: Icon = Icon("\u{F0EA}");
pub const HEADER: Icon = Icon("\u{F1DC}");
pub const SEPARATOR: Icon = Icon("\u{F142}");
pub const QUOTE: Icon = Icon("\u{F10D}");
pub const LOCK: Icon = Icon("\u{F023}");
pub const WIDTH: Icon = Icon("\u{F07E}");
pub const EXPAND: Icon = Icon("\u{F0B2}");
pub const HEIGHT: Icon = Icon("\u{F07D}");
pub const SEARCH: Icon = Icon("\u{F002}");
pub const FUZZY_SEARCH: Icon = Icon("\u{F00E}");
pub const FILTER: Icon = Icon("\u{F0B0}");
pub const SORT: Icon = Icon("\u{F0DC}");
pub const COLUMN: Icon = Icon("\u{F0DB}");
pub const CAST: Icon = Icon("\u{F0EC}");
pub const CHART: Icon = Icon("\u{F080}");
pub const SCATTER: Icon = Icon("\u{F192}");
pub const TAG: Icon = Icon("\u{F02B}");
pub const PRECISION: Icon = Icon("\u{F1EC}");
pub const PALETTE: Icon = Icon("\u{F1FC}");
pub const PENCIL: Icon = Icon("\u{F040}");
pub const REFRESH: Icon = Icon("\u{F021}");
pub const POWER: Icon = Icon("\u{F011}");
pub const SCHEMA: Icon = Icon("\u{F0E8}");
pub const INFO: Icon = Icon("\u{F05A}");
pub const FONT: Icon = Icon("\u{F031}");
pub const ROW_NUMBERS: Icon = Icon("\u{F0CB}");
pub const BORDERS: Icon = Icon("\u{F096}");
pub const WEDGE: Icon = Icon("\u{E0B0}");
pub const TRIANGLE: &str = "\u{25B6}";

pub fn extension(ext: Option<&str>) -> Icon {
    match ext {
        Some("csv" | "tsv") => TABLE,
        Some("json" | "jsonl") => JSON,
        Some("parquet" | "pqt" | "arrow" | "avro" | "db" | "sqlite") => DATABASE,
        Some("xls" | "xlsx" | "xlsm" | "xlsb") => EXCEL,
        Some("html" | "htm") => HTML,
        Some("md" | "markdown") => MARKDOWN,
        Some("fwf" | "log" | "logfmt" | "txt") => TEXT,
        _ => FILE,
    }
}
