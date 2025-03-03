use ratatui::widgets::{Block, StatefulWidget};

use crate::config::theme;

use super::input::{Input, InputState};

#[derive(Debug, Default)]
pub struct SearchBarState {
    input: InputState,
}

impl SearchBarState {
    pub fn input(&mut self) -> &mut InputState {
        &mut self.input
    }
}

#[derive(Debug)]
pub struct SearchBar {
    selection: bool,
}

impl SearchBar {
    pub fn new() -> Self {
        Self { selection: false }
    }

    pub fn with_selection(self, selection: bool) -> Self {
        Self { selection }
    }
}

impl Default for SearchBar {
    fn default() -> Self {
        Self::new()
    }
}

impl StatefulWidget for SearchBar {
    type State = SearchBarState;

    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        state: &mut Self::State,
    ) {
        Input::new()
            .style(theme().text())
            .selection(self.selection)
            .block(
                Block::bordered()
                    .title_top("Fuzzy Search")
                    .border_type(ratatui::widgets::BorderType::Rounded)
                    .style(theme().block()),
            )
            .render(area, buf, &mut state.input);
    }
}
