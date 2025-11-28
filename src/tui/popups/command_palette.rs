use crossterm::event::{KeyCode, KeyModifiers};
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
    handler::action::Action,
    misc::globals::theme,
    tui::{
        component::Component,
        status_bar::{StatusBar, Tag},
        widgets::{block::Block, input::Input},
    },
};

#[derive(Debug, Default)]
pub struct CommandPalette {
    input: Input,
    list: ListState,
    items: Vec<String>,
}

impl CommandPalette {
    pub fn new(cmd: String, items: Vec<String>) -> Self {
        let mut input = Input::default();
        for c in cmd.chars() {
            input.insert(c);
        }
        Self {
            input,
            list: ListState::default(),
            items,
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

impl Component for CommandPalette {
    fn render(
        &mut self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        focus_state: crate::tui::component::FocusState,
    ) {
        Clear.render(area, buf);
        let [input_area, list_area] =
            Layout::vertical([Constraint::Length(3), Constraint::Fill(1)]).areas(area);
        let input_area = {
            let block = Block::default().border_set(Set {
                bottom_left: VERTICAL_RIGHT,
                bottom_right: VERTICAL_LEFT,
                ..ROUNDED
            });
            let inner = block.inner(input_area);
            block.render(input_area, buf);
            inner
        };
        self.input.render(input_area, buf, focus_state);

        StatefulWidget::render(
            List::new(
                self.items
                    .iter()
                    .map(|item| Line::styled(item.as_str(), theme().text())),
            )
            .highlight_style(theme().row_highlighted())
            .block(
                Block::default()
                    .borders(Borders::LEFT | Borders::BOTTOM | Borders::RIGHT)
                    .bottom(if self.list.selected().is_some() {
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
            &mut self.list,
        );
    }
    fn handle(&mut self, event: crossterm::event::KeyEvent) -> bool {
        self.input.handle(event)
            || match (event.code, event.modifiers) {
                (KeyCode::Enter, KeyModifiers::NONE) => {
                    Action::AppDismissOverlay.enqueue();
                    true
                }
                (KeyCode::Esc, KeyModifiers::NONE) => {
                    Action::AppDismissOverlay.enqueue();
                    true
                }
                _ => false,
            }
    }
}

// pub struct CommandPalette<Iter> {
//     items: Iter,
// }

// impl<Iter> CommandPalette<Iter> {
//     pub fn new(items: Iter) -> Self {
//         Self { items }
//     }
// }

// impl<'a, Iter> StatefulWidget for CommandPalette<Iter>
// where
//     Iter: IntoIterator,
//     Iter::Item: Into<Cow<'a, str>>,
// {
//     type State = CommandPaletteState;

//     fn render(
//         self,
//         area: ratatui::prelude::Rect,
//         buf: &mut ratatui::prelude::Buffer,
//         state: &mut Self::State,
//     ) {
//         Clear.render(area, buf);
//         let [input_area, list_area] =
//             Layout::vertical([Constraint::Length(3), Constraint::Fill(1)]).areas(area);
//         let input_area = {
//             let block = Block::default().border_set(Set {
//                 bottom_left: VERTICAL_RIGHT,
//                 bottom_right: VERTICAL_LEFT,
//                 ..ROUNDED
//             });
//             let inner = block.inner(list_area);
//             block.render(list_area, buf);
//             inner
//         };
//         self.input.render();
//         StatefulWidget::render(
//             List::new(
//                 self.items
//                     .into_iter()
//                     .map(|item| Line::styled(item.into(), theme().text())),
//             )
//             .highlight_style(theme().row_highlighted())
//             .block(
//                 Block::default()
//                     .borders(Borders::LEFT | Borders::BOTTOM | Borders::RIGHT)
//                     .bottom(if state.list.selected().is_some() {
//                         StatusBar::new()
//                             .mono_color()
//                             .centered()
//                             .tag(Tag::new(" Insert ", " Enter "))
//                             .tag(Tag::new(" Cancel ", " Esc "))
//                     } else {
//                         StatusBar::new()
//                     })
//                     .into_widget(),
//             ),
//             list_area,
//             buf,
//             &mut state.list,
//         );
//     }
// }
