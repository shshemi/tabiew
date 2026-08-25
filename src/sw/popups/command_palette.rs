use std::{borrow::Cow, fmt::Display, sync::LazyLock};

use ratatui::{buffer::Buffer, layout::Rect, widgets::StatefulWidget};
use strum::IntoEnumIterator;
use strum_macros::{EnumIter, IntoStaticStr};

use crate::sw::pickers::search_picker::{SearchPicker, SearchPickerState};

static COMMANDS: LazyLock<Vec<Command>> = LazyLock::new(|| Command::iter().collect());

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter, IntoStaticStr)]
pub enum Command {
    Cast,
    Edit,
    Export,
    Filter,
    FuzzySearch,
    Histogram,
    Import,
    Info,
    Order,
    Query,
    Quit,
    Register,
    ReloadConfig,
    ScatterPlot,
    Search,
    Schema,
    Select,
    Sort,
    ThemeSelector,
    ToggleBorders,
    ToggleRowNumbers,
}

impl Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Into::<&str>::into(self))
    }
}

#[derive(Debug, Default)]
pub struct CommandPaletteState {
    search: SearchPickerState,
}

impl CommandPaletteState {
    pub fn search(&self) -> &SearchPickerState {
        &self.search
    }

    pub fn search_mut(&mut self) -> &mut SearchPickerState {
        &mut self.search
    }

    pub fn text(&self) -> &str {
        self.search.text()
    }

    pub fn selected(&self) -> Option<Command> {
        self.search
            .selected()
            .and_then(|idx| COMMANDS.get(idx))
            .copied()
    }
}

#[derive(Debug, Default)]
pub struct CommandPalette<'a> {
    title: Cow<'a, str>,
}

impl<'a> CommandPalette<'a> {
    pub fn title(mut self, title: impl Into<Cow<'a, str>>) -> Self {
        self.title = title.into();
        self
    }
}

impl StatefulWidget for CommandPalette<'_> {
    type State = CommandPaletteState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        SearchPicker::new(&COMMANDS)
            .title(self.title)
            .render(area, buf, &mut state.search);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn typed(state: &mut CommandPaletteState, text: &str) {
        for c in text.chars() {
            state.search_mut().input_mut().insert(c);
        }
    }

    fn render(state: &mut CommandPaletteState, palette: CommandPalette) -> Buffer {
        let area = Rect::new(0, 0, 100, 30);
        let mut buf = Buffer::empty(area);
        palette.render(area, &mut buf, state);
        buf
    }

    fn content(buf: &Buffer) -> String {
        buf.content().iter().map(|c| c.symbol()).collect()
    }

    mod state {
        use super::*;

        #[test]
        fn opens_on_the_first_command() {
            assert_eq!(
                CommandPaletteState::default().selected(),
                COMMANDS.first().copied()
            );
        }

        #[test]
        fn each_index_maps_to_its_command() {
            let mut state = CommandPaletteState::default();

            for (idx, expected) in COMMANDS.iter().enumerate() {
                state.search_mut().select(Some(idx));
                assert_eq!(state.selected(), Some(*expected));
            }
        }

        #[test]
        fn no_selection_has_no_command() {
            let mut state = CommandPaletteState::default();
            state.search_mut().select(None);

            assert_eq!(state.selected(), None);
        }

        #[test]
        fn text_reflects_the_typed_query() {
            let mut state = CommandPaletteState::default();
            typed(&mut state, "quit");

            assert_eq!(state.text(), "quit");
        }

        #[test]
        fn filtering_maps_the_selection_back_to_the_command() {
            let mut state = CommandPaletteState::default();
            typed(&mut state, "quit");
            render(&mut state, CommandPalette::default());

            state.search_mut().select(Some(0));

            assert_eq!(state.selected(), Some(Command::Quit));
        }

        #[test]
        fn filtering_narrows_the_command_list() {
            let mut state = CommandPaletteState::default();
            typed(&mut state, "toggle");
            render(&mut state, CommandPalette::default());

            assert_eq!(state.search().len(), 2);
        }

        #[test]
        fn every_command_is_offered() {
            let mut state = CommandPaletteState::default();
            render(&mut state, CommandPalette::default());

            assert_eq!(state.search().len(), Command::iter().count());
        }
    }

    mod widget {
        use super::*;

        #[test]
        fn renders_command_names() {
            let mut state = CommandPaletteState::default();
            typed(&mut state, "quit");
            let buf = render(&mut state, CommandPalette::default());

            assert!(content(&buf).contains("Quit"));
        }

        #[test]
        fn renders_the_title() {
            let buf = render(
                &mut CommandPaletteState::default(),
                CommandPalette::default().title("Commands"),
            );

            assert!(content(&buf).contains("Commands"));
        }
    }
}
