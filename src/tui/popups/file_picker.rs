use std::{
    cmp::Ordering,
    ffi::OsStr,
    fs::{DirEntry, read_dir},
    path::{MAIN_SEPARATOR, Path, PathBuf},
};

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use home::home_dir;
use itertools::Itertools;

use crate::{
    misc::config::config,
    tui::{
        component::Component,
        pickers::text_picker_with_suggestion::{self, TextPickerWithSuggestion},
        widgets::input::Input,
    },
};

#[derive(Debug)]
pub struct FilePicker {
    text_picker: TextPickerWithSuggestion<fn(&str, usize) -> Vec<FileSuggestion>>,
}

impl FilePicker {
    pub fn with_title(self, title: impl Into<String>) -> Self {
        Self {
            text_picker: self.text_picker.with_title(title),
        }
    }

    pub fn path(&self) -> PathBuf {
        self.text_picker.value().into()
    }
}

impl Component for FilePicker {
    fn render(
        &mut self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        focus_state: crate::tui::component::FocusState,
    ) {
        self.text_picker.render(area, buf, focus_state);
    }

    fn handle(&mut self, event: KeyEvent) -> bool {
        self.text_picker.handle(event)
            || match (event.code, event.modifiers) {
                (KeyCode::Enter, KeyModifiers::NONE) => {
                    self.text_picker.apply_selected_suggestion();
                    self.text_picker.clear_suggestion_selection();
                    self.path().is_dir()
                }
                (KeyCode::Tab, KeyModifiers::NONE) => {
                    self.text_picker.apply_selected_suggestion();
                    true
                }
                _ => false,
            }
    }
}

impl Default for FilePicker {
    fn default() -> Self {
        Self {
            text_picker: TextPickerWithSuggestion::new(
                suggestions as for<'a> fn(&'a str, usize) -> Vec<FileSuggestion>,
            )
            .with_title("File Path")
            .with_value(
                std::env::current_dir()
                    .ok()
                    .or(home_dir())
                    .map(|p| path_to_string(&p))
                    .unwrap_or_default(),
            ),
        }
    }
}

#[derive(Debug)]
struct FileSuggestion {
    title: String,
    path: PathBuf,
}

impl text_picker_with_suggestion::Suggestion for FileSuggestion {
    fn title(&self) -> &str {
        &self.title
    }

    fn apply_to(&self, input: &mut Input) {
        input.set_value(path_to_string(&self.path));
    }
}

fn suggestions(query: &str, _: usize) -> Vec<FileSuggestion> {
    let path = Path::new(query);
    let filter = query
        .split(std::path::MAIN_SEPARATOR)
        .next_back()
        .unwrap_or_default();
    let path = if path.is_dir() && query.ends_with(std::path::MAIN_SEPARATOR) {
        path
    } else if let Some(parent) = path.parent() {
        parent
    } else {
        return Default::default();
    };
    let Ok(read_dir) = read_dir(path) else {
        return Default::default();
    };
    read_dir
        .flat_map(|r| r.ok())
        .filter(|entry| {
            entry
                .file_name()
                .to_string_lossy()
                .to_lowercase()
                .starts_with(&filter.to_lowercase())
        })
        .sorted_by(cmp_dir_entry)
        .flat_map(|entry| {
            let path = entry.path();
            path.file_name()
                .map(OsStr::to_string_lossy)
                .map(|name| suggestion_title(&path, &name))
                .map(|title| FileSuggestion { title, path })
        })
        .collect_vec()
}

fn suggestion_title(path: &Path, name: &str) -> String {
    let is_dir = path.is_dir();
    let use_nerd = config().use_nerd_font();
    match (is_dir, use_nerd) {
        (true, true) => format!(" {}  {}{}", icon(path), name, MAIN_SEPARATOR),
        (true, false) => format!("{}{}", name, MAIN_SEPARATOR),
        (false, true) => format!(" {}  {}", icon(path), name),
        (false, false) => name.to_owned(),
    }
}

fn icon(path: &Path) -> &'static str {
    const FOLDER_ICON: &str = "\u{F07B}";
    const FILE_ICON: &str = "\u{F15B}";
    const TABLE_ICON: &str = "\u{F0CE}";
    const JSON_ICON: &str = "\u{E60B}";
    const DATABASE_ICON: &str = "\u{F1C0}";
    const EXCEL_ICON: &str = "\u{F1C3}";
    const HTML_ICON: &str = "\u{F121}";
    const MARKDOWN_ICON: &str = "\u{E609}";
    const TEXT_ICON: &str = "\u{F0F6}";

    if path.is_dir() {
        return FOLDER_ICON;
    }
    match path
        .extension()
        .and_then(OsStr::to_str)
        .map(str::to_lowercase)
        .as_deref()
    {
        Some("csv" | "tsv") => TABLE_ICON,
        Some("json" | "jsonl") => JSON_ICON,
        Some("parquet" | "pqt" | "arrow" | "avro" | "db" | "sqlite") => DATABASE_ICON,
        Some("xls" | "xlsx" | "xlsm" | "xlsb") => EXCEL_ICON,
        Some("html" | "htm") => HTML_ICON,
        Some("md" | "markdown") => MARKDOWN_ICON,
        Some("fwf" | "log" | "txt") => TEXT_ICON,
        _ => FILE_ICON,
    }
}

fn path_to_string(path: &Path) -> String {
    let s = path.to_string_lossy();
    if path.is_dir() && !s.ends_with(MAIN_SEPARATOR) {
        format!("{}{}", s, MAIN_SEPARATOR)
    } else {
        s.into_owned()
    }
}

fn cmp_dir_entry(a: &DirEntry, b: &DirEntry) -> Ordering {
    if let Ok(type_a) = a.file_type()
        && let Ok(type_b) = b.file_type()
    {
        if type_a.is_dir() && !type_b.is_dir() {
            Ordering::Less
        } else if !type_a.is_dir() && type_b.is_dir() {
            Ordering::Greater
        } else {
            a.file_name()
                .to_string_lossy()
                .cmp(&b.file_name().to_string_lossy())
        }
    } else {
        Ordering::Equal
    }
}
