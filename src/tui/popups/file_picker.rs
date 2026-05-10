use std::{
    borrow::Cow,
    cmp::Ordering,
    ffi::OsStr,
    fs::{DirEntry, read_dir},
    path::{MAIN_SEPARATOR, Path, PathBuf},
};

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use home::home_dir;
use itertools::Itertools;

use crate::tui::{
    component::Component,
    pickers::text_picker_with_suggestion::{self, TextPickerWithSuggestion},
    widgets::input::Input,
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
                    self.text_picker
                        .selected_suggestion()
                        .map(|sug| sug.is_dir())
                        .unwrap_or_default()
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
                "File Path",
                suggestions as for<'a> fn(&'a str, usize) -> Vec<FileSuggestion>,
            )
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

impl FileSuggestion {
    fn is_dir(&self) -> bool {
        self.path.is_dir()
    }
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
    let filter = query.split('/').next_back().unwrap_or_default();
    let path = if path.is_dir() && query.ends_with('/') {
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
        .filter(|entry| entry.file_name().to_string_lossy().starts_with(filter))
        .sorted_by(cmp_dir_entry)
        .flat_map(|entry| {
            let path = entry.path();
            path.file_name()
                .map(OsStr::to_string_lossy)
                .map(Cow::into_owned)
                .map(|title| FileSuggestion { title, path })
        })
        .collect_vec()
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
    let Ok(type_a) = a.file_type() else {
        return Ordering::Equal;
    };
    let Ok(type_b) = a.file_type() else {
        return Ordering::Equal;
    };

    if type_a.is_dir() && !type_b.is_dir() {
        return Ordering::Less;
    }

    if !type_a.is_dir() && type_b.is_dir() {
        return Ordering::Greater;
    }

    b.file_name()
        .to_string_lossy()
        .cmp(&a.file_name().to_string_lossy())
}
