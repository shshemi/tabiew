use std::marker::PhantomData;

use ratatui::widgets::{Block, Clear, StatefulWidget, Widget};

use super::{
    input::{Input, InputState},
    Styler,
};

pub struct CommandPalleteState {
    input: InputState,
}

impl CommandPalleteState {
    pub fn new(cmd: String) -> Self {
        let mut input = InputState::default();
        for c in cmd.chars() {
            input.insert(c);
        }
        Self { input }
    }
    pub fn input(&mut self) -> &mut InputState {
        &mut self.input
    }
}

pub struct CommandPallete<Theme> {
    selection: bool,
    _theme: PhantomData<Theme>,
}

impl<Theme> CommandPallete<Theme> {
    pub fn new() -> Self {
        Self {
            selection: true,
            _theme: Default::default(),
        }
    }
}

impl<Theme: Styler> StatefulWidget for CommandPallete<Theme> {
    type State = CommandPalleteState;

    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        state: &mut Self::State,
    ) {
        Clear.render(area, buf);
        Input::<Theme>::new()
            .style(Theme::status_bar_search())
            .selection(self.selection)
            .block(
                Block::bordered()
                    .title_top(" Command ")
                    .border_type(ratatui::widgets::BorderType::Rounded)
                    .border_style(Theme::pallete()),
            )
            .render(area, buf, &mut state.input);
    }
}
