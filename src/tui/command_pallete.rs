use std::{borrow::Cow, marker::PhantomData};

use ratatui::{
    layout::{Alignment, Constraint, Layout},
    style::{Modifier, Stylize},
    symbols::{
        border::{Set, ROUNDED},
        line::{VERTICAL_LEFT, VERTICAL_RIGHT},
    },
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, List, ListState, StatefulWidget, Widget},
};

use super::{
    input::{Input, InputState},
    Styler,
};

pub struct CommandPalleteState {
    input: InputState,
    list: ListState,
}

impl CommandPalleteState {
    pub fn new(cmd: String) -> Self {
        let mut input = InputState::default();
        for c in cmd.chars() {
            input.insert(c);
        }
        Self {
            input,
            list: ListState::default(),
        }
    }

    pub fn input(&mut self) -> &mut InputState {
        &mut self.input
    }

    pub fn list(&mut self) -> &mut ListState {
        &mut self.list
    }

    pub fn set_input(&mut self, text: String) {
        let mut input = InputState::default();
        for c in text.chars() {
            input.insert(c);
        }
        self.input = input;
    }
}

pub struct CommandPallete<Theme, Iter> {
    items: Iter,
    _theme: PhantomData<Theme>,
}

impl<Theme, Iter> CommandPallete<Theme, Iter> {
    pub fn new(items: Iter) -> Self {
        Self {
            items,
            _theme: Default::default(),
        }
    }
}

impl<'a, Theme, Iter> StatefulWidget for CommandPallete<Theme, Iter>
where
    Theme: Styler,
    Iter: IntoIterator,
    Iter::Item: Into<Cow<'a, str>>,
{
    type State = CommandPalleteState;

    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        state: &mut Self::State,
    ) {
        Clear.render(area, buf);
        let [input_area, list_area] =
            Layout::vertical([Constraint::Length(3), Constraint::Fill(1)]).areas(area);
        Input::<Theme>::new()
            .style(Theme::pallete_text())
            .selection(state.list.selected().is_none())
            .block(
                Block::bordered()
                    .title_top(Line::raw("Command Palelte"))
                    .title_alignment(Alignment::Center)
                    .border_set(Set {
                        bottom_left: VERTICAL_RIGHT,
                        bottom_right: VERTICAL_LEFT,
                        ..ROUNDED
                    })
                    .style(Theme::pallete()),
            )
            .render(input_area, buf, &mut state.input);
        StatefulWidget::render(
            List::new(
                self.items
                    .into_iter()
                    .map(|item| Line::styled(item.into(), Theme::pallete_text())),
            )
            .highlight_style(Theme::table_highlight())
            .block(
                Block::new()
                    .borders(Borders::LEFT | Borders::BOTTOM | Borders::RIGHT)
                    .border_type(BorderType::Rounded)
                    .style(Theme::pallete())
                    // .title_alignment(Alignment::Center)
                    .title_bottom(
                        if state.list.selected().is_some() {
                            Line::from_iter([
                                Span::raw("Insert using "),
                                Span::raw(" Enter ").add_modifier(Modifier::REVERSED),
                                Span::raw(", cancel using "),
                                Span::raw(" Esc ").add_modifier(Modifier::REVERSED),
                            ])
                        } else {
                            Line::from_iter([
                                Span::raw("Select up with "),
                                Span::raw(" \u{2191} ").add_modifier(Modifier::REVERSED),
                                Span::raw(" or "),
                                Span::raw(" Ctrl+J ").add_modifier(Modifier::REVERSED),
                                Span::raw(", select down with "),
                                Span::raw(" \u{2193} ").add_modifier(Modifier::REVERSED),
                                Span::raw(" or "),
                                Span::raw(" Ctrl+N ").add_modifier(Modifier::REVERSED),
                            ])
                        }
                        .alignment(Alignment::Center),
                    ),
            ),
            list_area,
            buf,
            &mut state.list,
        );
    }
}
