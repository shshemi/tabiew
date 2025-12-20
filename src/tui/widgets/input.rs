use crate::{misc::globals::theme, tui::component::Component};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    layout::Rect,
    style::Modifier,
    widgets::{Paragraph, Widget},
};

#[derive(Debug, Default, Clone, Copy)]
pub enum InputType {
    #[default]
    Any,
    Numeric,
    Alphabetic,
    MultiNumeric,
}

#[derive(Debug, Default)]
pub struct Input {
    input: tui_input::Input,
    input_type: InputType,
    max_len: Option<usize>,
    hint: String,
}

impl Input {
    pub fn with_max_len(self, max_len: impl Into<Option<usize>>) -> Self {
        Input {
            max_len: max_len.into(),
            ..self
        }
    }

    pub fn with_input_type(self, input_type: InputType) -> Self {
        Input { input_type, ..self }
    }

    pub fn with_value(self, value: String) -> Self {
        Self {
            input: self.input.with_value(value),
            ..self
        }
    }

    pub fn with_hint(self, value: String) -> Self {
        Self {
            hint: value,
            ..self
        }
    }

    pub fn delete_prev(&mut self) {
        self.input.handle(tui_input::InputRequest::DeletePrevChar);
    }

    pub fn delete_next(&mut self) {
        self.input.handle(tui_input::InputRequest::DeleteNextChar);
    }

    pub fn goto_prev(&mut self) {
        self.input.handle(tui_input::InputRequest::GoToPrevChar);
    }

    pub fn goto_next(&mut self) {
        self.input.handle(tui_input::InputRequest::GoToNextChar);
    }

    pub fn goto_start(&mut self) {
        self.input.handle(tui_input::InputRequest::GoToStart);
    }

    pub fn goto_end(&mut self) {
        self.input.handle(tui_input::InputRequest::GoToEnd);
    }

    pub fn insert(&mut self, c: char) {
        if let Some(max_len) = self.max_len {
            if self.value().chars().count() < max_len {
                self.input.handle(tui_input::InputRequest::InsertChar(c));
            }
        } else {
            self.input.handle(tui_input::InputRequest::InsertChar(c));
        }
    }

    pub fn goto_next_word(&mut self) {
        self.input.handle(tui_input::InputRequest::GoToNextWord);
    }

    pub fn goto_prev_word(&mut self) {
        self.input.handle(tui_input::InputRequest::GoToPrevWord);
    }

    pub fn delete_next_word(&mut self) {
        self.input.handle(tui_input::InputRequest::DeleteNextWord);
    }

    pub fn delete_prev_word(&mut self) {
        self.input.handle(tui_input::InputRequest::DeletePrevWord);
    }

    pub fn value(&self) -> &str {
        self.input.value()
    }
}

impl Component for Input {
    fn render(
        &mut self,
        area: Rect,
        buf: &mut ratatui::prelude::Buffer,
        focus_state: crate::tui::component::FocusState,
    ) {
        // styles
        let style = theme().text();
        let hint_style = theme().subtext();

        if self.input.value().is_empty() {
            // draw hint
            Paragraph::new(self.hint.as_str())
                .style(hint_style)
                .render(area, buf);
            // draw cursor
            if focus_state.is_focused() {
                buf.set_style(
                    Rect {
                        x: area.x,
                        y: area.y,
                        width: 1,
                        height: 1,
                    },
                    style.add_modifier(Modifier::REVERSED),
                );
            }
        } else {
            // draw text
            let scroll = self
                .input
                .visual_scroll(area.width.saturating_sub(1).into());
            Paragraph::new(self.input.value().chars().skip(scroll).collect::<String>())
                .style(style)
                .render(area, buf);
            // draw cursor
            if focus_state.is_focused() {
                buf.set_style(
                    Rect {
                        x: area.x + self.input.visual_cursor().saturating_sub(scroll) as u16,
                        y: area.y,
                        width: 1,
                        height: 1,
                    },
                    style.add_modifier(Modifier::REVERSED),
                );
            }
        }
    }

    fn handle(&mut self, event: KeyEvent) -> bool {
        match (event.code, event.modifiers) {
            (KeyCode::Backspace, KeyModifiers::ALT)
            | (KeyCode::Char('w'), KeyModifiers::CONTROL) => {
                self.delete_prev_word();
                true
            }
            (KeyCode::Backspace, KeyModifiers::NONE)
            | (KeyCode::Char('h'), KeyModifiers::CONTROL) => {
                self.delete_prev();
                true
            }
            (KeyCode::Left, KeyModifiers::ALT) | (KeyCode::Char('b'), KeyModifiers::ALT) => {
                self.goto_prev_word();
                true
            }
            (KeyCode::Right, KeyModifiers::ALT) | (KeyCode::Char('f'), KeyModifiers::ALT) => {
                self.goto_next_word();
                true
            }
            (KeyCode::Left, KeyModifiers::NONE) | (KeyCode::Char('b'), KeyModifiers::CONTROL) => {
                self.goto_prev();
                true
            }
            (KeyCode::Right, KeyModifiers::NONE) | (KeyCode::Char('f'), KeyModifiers::CONTROL) => {
                self.goto_next();
                true
            }
            (KeyCode::Home, KeyModifiers::NONE) | (KeyCode::Char('a'), KeyModifiers::CONTROL) => {
                self.goto_start();
                true
            }
            (KeyCode::End, KeyModifiers::NONE) | (KeyCode::Char('e'), KeyModifiers::CONTROL) => {
                self.goto_end();
                true
            }
            (KeyCode::Delete, KeyModifiers::NONE) | (KeyCode::Char('d'), KeyModifiers::CONTROL) => {
                self.delete_next();
                true
            }
            (KeyCode::Char('i'), KeyModifiers::CONTROL) => true,
            (KeyCode::Char(c), KeyModifiers::NONE) | (KeyCode::Char(c), KeyModifiers::SHIFT) => {
                match self.input_type {
                    InputType::Any => {
                        self.insert(c);
                    }
                    InputType::Numeric => {
                        if c.is_numeric() {
                            self.insert(c);
                        }
                    }
                    InputType::Alphabetic => {
                        if c.is_alphabetic() {
                            self.insert(c);
                        }
                    }
                    InputType::MultiNumeric => {
                        if c.is_numeric() || c == ' ' {
                            self.insert(c);
                        }
                    }
                }
                true
            }
            _ => false,
        }
    }
}

// #[derive(Debug)]
// pub struct Input<'a> {
//     block: Option<Block<'a>>,
//     hint: Cow<'a, str>,
//     selection: bool,
// }

// impl<'a> Input<'a> {
//     pub fn block(mut self, block: Block<'a>) -> Self {
//         self.block = Some(block);
//         self
//     }

//     pub fn hint(mut self, hint: impl Into<Cow<'a, str>>) -> Self {
//         self.hint = hint.into();
//         self
//     }
// }

// impl<'a> Default for Input<'a> {
//     fn default() -> Self {
//         Self {
//             block: None,
//             hint: Default::default(),
//             selection: true,
//         }
//     }
// }

// impl StatefulWidget for Input<'_> {
//     type State = InputState;

//     fn render(
//         self,
//         area: ratatui::prelude::Rect,
//         buf: &mut ratatui::prelude::Buffer,
//         state: &mut Self::State,
//     ) {
//         // draw block and update area
//         let area = if let Some(block) = self.block {
//             let new_area = block.inner(area);
//             block.render(area, buf);
//             new_area
//         } else {
//             area
//         };

//         if state.input.value().is_empty() {
//             // draw hint
//             Paragraph::new(self.hint)
//                 .style(self.hint_style)
//                 .render(area, buf);
//             // draw cursor
//             if self.selection {
//                 buf.set_style(
//                     Rect {
//                         x: area.x,
//                         y: area.y,
//                         width: 1,
//                         height: 1,
//                     },
//                     self.style.add_modifier(Modifier::REVERSED),
//                 );
//             }
//         } else {
//             // draw text
//             let scroll = state
//                 .input
//                 .visual_scroll(area.width.saturating_sub(1).into());
//             Paragraph::new(state.input.value().chars().skip(scroll).collect::<String>())
//                 .style(self.style)
//                 .render(area, buf);
//             // draw cursor
//             if self.selection {
//                 buf.set_style(
//                     Rect {
//                         x: area.x + state.input.visual_cursor().saturating_sub(scroll) as u16,
//                         y: area.y,
//                         width: 1,
//                         height: 1,
//                     },
//                     self.style.add_modifier(Modifier::REVERSED),
//                 );
//             }
//         }
//     }
// }
