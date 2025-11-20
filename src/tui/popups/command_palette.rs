use std::borrow::Cow;

use ratatui::{
    layout::{Constraint, Layout},
    symbols::{
        border::{ROUNDED, Set},
        line::{VERTICAL_LEFT, VERTICAL_RIGHT},
    },
    text::Line,
    widgets::{Borders, Clear, List, ListState, StatefulWidget, Widget},
};

use crate::{
    misc::globals::theme,
    tui::{
        status_bar::{StatusBar, Tag},
        widgets::{
            block::Block,
            input::{Input, Input},
        },
    },
};

#[derive(Debug, Default)]
pub struct CommandPaletteState {
    input: Input,
    list: ListState,
}

impl CommandPaletteState {
    pub fn new(cmd: String) -> Self {
        let mut input = Input::default();
        for c in cmd.chars() {
            input.insert(c);
        }
        Self {
            input,
            list: ListState::default(),
        }
    }

    pub fn input(&mut self) -> &mut Input {
        &mut self.input
    }

    pub fn list(&mut self) -> &mut ListState {
        &mut self.list
    }

    pub fn set_input(&mut self, text: String) {
        let mut input = Input::default();
        for c in text.chars() {
            input.insert(c);
        }
        self.input = input;
    }
}

pub struct CommandPalette<Iter> {
    items: Iter,
}

impl<Iter> CommandPalette<Iter> {
    pub fn new(items: Iter) -> Self {
        Self { items }
    }
}

impl<'a, Iter> StatefulWidget for CommandPalette<Iter>
where
    Iter: IntoIterator,
    Iter::Item: Into<Cow<'a, str>>,
{
    type State = CommandPaletteState;

    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        state: &mut Self::State,
    ) {
        Clear.render(area, buf);
        let [input_area, list_area] =
            Layout::vertical([Constraint::Length(3), Constraint::Fill(1)]).areas(area);
        Input::default()
            .with_show_cursor(state.list.selected().is_none())
            .hint("Execute a command")
            .block(Block::default().border_set(Set {
                bottom_left: VERTICAL_RIGHT,
                bottom_right: VERTICAL_LEFT,
                ..ROUNDED
            }))
            .render(input_area, buf, &mut state.input);
        StatefulWidget::render(
            List::new(
                self.items
                    .into_iter()
                    .map(|item| Line::styled(item.into(), theme().text())),
            )
            .highlight_style(theme().row_highlighted())
            .block(
                Block::default()
                    .borders(Borders::LEFT | Borders::BOTTOM | Borders::RIGHT)
                    .bottom(if state.list.selected().is_some() {
                        StatusBar::new()
                            .mono_color()
                            .centered()
                            .tag(Tag::new(" Insert ", " Enter "))
                            .tag(Tag::new(" Cancel ", " Esc "))
                    } else {
                        StatusBar::new()
                    })
                    .into_widget(),
            ),
            list_area,
            buf,
            &mut state.list,
        );
    }
}
