use std::{
    borrow::Cow,
    cmp::Ordering,
    ffi::OsStr,
    fs::{DirEntry, read_dir},
    path::{MAIN_SEPARATOR, Path, PathBuf},
};

use home::home_dir;
use itertools::Itertools;
use ratatui::{buffer::Buffer, layout::Rect, widgets::StatefulWidget};

use crate::sw::{
    pickers::text_picker_with_suggestion::{
        Provider, Suggestion, TextPickerWithSuggestion, TextPickerWithSuggestionState,
    },
    widgets::input::InputState,
};

#[derive(Debug)]
pub struct FilePickerState {
    picker: TextPickerWithSuggestionState<FileSuggestionProvider>,
}

impl FilePickerState {
    pub fn new() -> Self {
        Self {
            picker: TextPickerWithSuggestionState::new(FileSuggestionProvider).with_value(
                std::env::current_dir()
                    .ok()
                    .or_else(home_dir)
                    .map(|path| path_to_string(&path))
                    .unwrap_or_default(),
            ),
        }
    }

    pub fn picker(&self) -> &TextPickerWithSuggestionState<FileSuggestionProvider> {
        &self.picker
    }

    pub fn picker_mut(&mut self) -> &mut TextPickerWithSuggestionState<FileSuggestionProvider> {
        &mut self.picker
    }

    pub fn input(&self) -> &InputState {
        self.picker.input()
    }

    pub fn input_mut(&mut self) -> &mut InputState {
        self.picker.input_mut()
    }

    pub fn path(&self) -> PathBuf {
        self.picker.value().into()
    }
}

impl Default for FilePickerState {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub struct FilePicker<'a> {
    title: Cow<'a, str>,
}

impl<'a> FilePicker<'a> {
    pub fn title(mut self, title: impl Into<Cow<'a, str>>) -> Self {
        self.title = title.into();
        self
    }
}

impl Default for FilePicker<'_> {
    fn default() -> Self {
        Self {
            title: Cow::Borrowed("File Path"),
        }
    }
}

impl StatefulWidget for FilePicker<'_> {
    type State = FilePickerState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        TextPickerWithSuggestion::default()
            .title(self.title)
            .render(area, buf, &mut state.picker);
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct FileSuggestionProvider;

impl Provider for FileSuggestionProvider {
    type Suggestion = FileSuggestion;

    fn suggestions(&self, query: &str, _cursor: usize) -> Vec<FileSuggestion> {
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
                    .map(|s| {
                        if path.is_dir() {
                            format!("{s}/")
                        } else {
                            s.into()
                        }
                    })
                    .map(|title| FileSuggestion { title, path })
            })
            .collect_vec()
    }
}

#[derive(Debug)]
pub struct FileSuggestion {
    title: String,
    path: PathBuf,
}

impl Suggestion for FileSuggestion {
    fn title(&self) -> &str {
        &self.title
    }

    fn apply_to(&self, input: &mut InputState) {
        input.set_value(path_to_string(&self.path));
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use tempfile::{TempDir, tempdir};

    fn tree() -> TempDir {
        let dir = tempdir().unwrap();
        fs::create_dir(dir.path().join("zeta_dir")).unwrap();
        fs::create_dir(dir.path().join("beta_dir")).unwrap();
        File::create(dir.path().join("alpha.txt")).unwrap();
        File::create(dir.path().join("gamma.csv")).unwrap();
        dir
    }

    fn query_in(dir: &TempDir, suffix: &str) -> String {
        format!("{}/{}", dir.path().to_string_lossy(), suffix)
    }

    fn titles(query: &str) -> Vec<String> {
        FileSuggestionProvider
            .suggestions(query, 0)
            .into_iter()
            .map(|suggestion| suggestion.title)
            .collect()
    }

    mod suggesting {
        use super::*;

        #[test]
        fn a_trailing_slash_lists_the_directory() {
            let dir = tree();

            assert_eq!(
                titles(&query_in(&dir, "")),
                vec!["beta_dir/", "zeta_dir/", "alpha.txt", "gamma.csv"]
            );
        }

        #[test]
        fn directories_sort_before_files() {
            let dir = tree();
            let titles = titles(&query_in(&dir, ""));

            let last_dir = titles.iter().rposition(|t| t.ends_with('/')).unwrap();
            let first_file = titles.iter().position(|t| !t.ends_with('/')).unwrap();

            assert!(last_dir < first_file);
        }

        #[test]
        fn a_partial_name_filters_by_prefix() {
            let dir = tree();

            assert_eq!(titles(&query_in(&dir, "al")), vec!["alpha.txt"]);
        }

        #[test]
        fn the_prefix_filter_ignores_case() {
            let dir = tree();

            assert_eq!(titles(&query_in(&dir, "AL")), vec!["alpha.txt"]);
        }

        #[test]
        fn a_prefix_matching_nothing_suggests_nothing() {
            let dir = tree();

            assert!(titles(&query_in(&dir, "nope")).is_empty());
        }

        #[test]
        fn a_missing_directory_suggests_nothing() {
            assert!(titles("/definitely/not/a/real/path/x").is_empty());
        }

        #[test]
        fn a_bare_name_with_no_parent_suggests_nothing() {
            assert!(titles("").is_empty());
        }
    }

    mod applying {
        use super::*;

        #[test]
        fn choosing_a_file_writes_its_full_path() {
            let dir = tree();
            let chosen = FileSuggestionProvider
                .suggestions(&query_in(&dir, "al"), 0)
                .pop()
                .unwrap();
            let mut input = InputState::default();

            chosen.apply_to(&mut input);

            assert_eq!(
                input.value(),
                dir.path().join("alpha.txt").to_string_lossy()
            );
        }

        #[test]
        fn choosing_a_directory_appends_a_separator() {
            let dir = tree();
            let chosen = FileSuggestionProvider
                .suggestions(&query_in(&dir, "beta"), 0)
                .pop()
                .unwrap();
            let mut input = InputState::default();

            chosen.apply_to(&mut input);

            assert!(input.value().ends_with(MAIN_SEPARATOR));
        }
    }

    mod path_strings {
        use super::*;

        #[test]
        fn a_directory_gains_a_trailing_separator() {
            let dir = tree();

            assert!(path_to_string(dir.path()).ends_with(MAIN_SEPARATOR));
        }

        #[test]
        fn a_file_is_left_alone() {
            let dir = tree();
            let file = dir.path().join("alpha.txt");

            assert_eq!(path_to_string(&file), file.to_string_lossy());
        }

        #[test]
        fn a_directory_that_already_ends_in_a_separator_is_unchanged() {
            let dir = tree();
            let with_sep = PathBuf::from(format!("{}{}", dir.path().display(), MAIN_SEPARATOR));

            assert_eq!(
                path_to_string(&with_sep),
                format!("{}{}", dir.path().display(), MAIN_SEPARATOR)
            );
        }
    }

    mod state {
        use super::*;

        #[test]
        fn opens_on_an_existing_directory() {
            let state = FilePickerState::new();

            assert!(state.path().is_dir());
        }

        #[test]
        fn the_path_follows_the_input() {
            let mut state = FilePickerState::new();
            state.input_mut().set_value("/tmp/data.csv".to_owned());

            assert_eq!(state.path(), PathBuf::from("/tmp/data.csv"));
        }

        #[test]
        fn applying_a_suggestion_updates_the_path() {
            let dir = tree();
            let mut state = FilePickerState::new();
            state.input_mut().set_value(query_in(&dir, "al"));
            state.picker_mut().refresh_suggestions();
            state.picker_mut().select(Some(0));

            state.picker_mut().apply_selected_suggestion();

            assert_eq!(state.path(), dir.path().join("alpha.txt"));
        }

        #[test]
        fn suggestions_refresh_as_the_path_is_edited() {
            let dir = tree();
            let mut state = FilePickerState::new();
            state.input_mut().set_value(query_in(&dir, "al"));
            state.picker_mut().refresh_suggestions();

            assert_eq!(
                state
                    .picker()
                    .suggestions()
                    .iter()
                    .map(Suggestion::title)
                    .collect_vec(),
                vec!["alpha.txt"]
            );
        }
    }

    mod widget {
        use super::*;

        fn render(state: &mut FilePickerState, picker: FilePicker) -> Buffer {
            let area = Rect::new(0, 0, 100, 30);
            let mut buf = Buffer::empty(area);
            picker.render(area, &mut buf, state);
            buf
        }

        fn content(buf: &Buffer) -> String {
            buf.content().iter().map(|c| c.symbol()).collect()
        }

        #[test]
        fn defaults_to_a_file_path_title() {
            let buf = render(&mut FilePickerState::new(), FilePicker::default());

            assert!(content(&buf).contains("File Path"));
        }

        #[test]
        fn the_title_is_overridable() {
            let buf = render(
                &mut FilePickerState::new(),
                FilePicker::default().title("Export To"),
            );

            let content = content(&buf);
            assert!(content.contains("Export To"));
            assert!(!content.contains("File Path"));
        }

        #[test]
        fn renders_the_suggestions_for_the_typed_path() {
            let dir = tree();
            let mut state = FilePickerState::new();
            state.input_mut().set_value(query_in(&dir, "al"));
            let buf = render(&mut state, FilePicker::default());

            assert!(content(&buf).contains("alpha.txt"));
        }
    }
}
