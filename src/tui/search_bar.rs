use std::marker::PhantomData;

use ratatui::widgets::{Block, StatefulWidget};

use super::{
    input::{Input, InputState},
    Styler,
};

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
pub struct SearchBar<Theme> {
    selection: bool,
    _theme: PhantomData<Theme>,
}

impl<Theme> SearchBar<Theme> {
    pub fn new() -> Self {
        Self {
            selection: false,
            _theme: Default::default(),
        }
    }

    pub fn with_selection(self, selection: bool) -> Self {
        Self {
            selection,
            _theme: PhantomData,
        }
    }
}

impl<Theme: Styler> StatefulWidget for SearchBar<Theme> {
    type State = SearchBarState;

    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        state: &mut Self::State,
    ) {
        Input::<Theme>::new()
            .style(Theme::pallete_text())
            .selection(self.selection)
            .block(
                Block::bordered()
                    .title_top("Fuzzy Search")
                    .border_type(ratatui::widgets::BorderType::Rounded)
                    .style(Theme::pallete()),
            )
            .render(area, buf, &mut state.input);
    }
}
