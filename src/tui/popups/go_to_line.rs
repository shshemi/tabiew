use crossterm::event::KeyEvent;
use ratatui::{
    layout::{Constraint, Flex, Layout},
    widgets::{Clear, Widget},
};

use crate::tui::{
    component::Component,
    widgets::{
        block::Block,
        input::{Input, InputType},
    },
};

#[derive(Debug)]
pub struct GoToLine {
    rollback: usize,
    input: Input,
}

impl GoToLine {
    pub fn new(rollback: usize) -> Self {
        Self {
            input: Input::default().with_input_type(InputType::Numeric),
            rollback,
        }
    }

    pub fn with_value(self, value: usize) -> Self {
        Self {
            rollback: self.rollback,
            input: self.input.with_value(value.to_string()),
        }
    }

    pub fn handle(&mut self, event: KeyEvent) {
        self.input.handle(event);
    }

    pub fn rollback(&self) -> usize {
        self.rollback
    }

    pub fn value(&self) -> usize {
        self.input.value().parse().unwrap_or(1)
    }
}

impl Component for GoToLine {
    fn render(
        &mut self,
        _area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        focus_state: crate::tui::component::FocusState,
    ) {
        let [area, _] = Layout::horizontal([Constraint::Length(32), Constraint::Length(1)])
            .flex(Flex::End)
            .areas(buf.area);
        let [_, area] =
            Layout::vertical([Constraint::Length(1), Constraint::Length(3)]).areas(area);
        Clear.render(area, buf);
        let area = {
            let block = Block::default().title("Go to Line");
            let inner = block.inner(area);
            block.render(area, buf);
            inner
        };
        self.input.render(area, buf, focus_state);
    }
    fn handle(&mut self, event: KeyEvent) -> bool {
        self.input.handle(event)
    }
}

// #[derive(Debug, Default)]
// pub struct GoToLine {}

// impl StatefulWidget for GoToLine {
//     type State = GoToLineState;

//     fn render(
//         self,
//         _: ratatui::prelude::Rect,
//         buf: &mut ratatui::prelude::Buffer,
//         state: &mut Self::State,
//     ) {
//         let [area, _] = Layout::horizontal([Constraint::Length(32), Constraint::Length(1)])
//             .flex(Flex::End)
//             .areas(buf.area);
//         let [_, area] =
//             Layout::vertical([Constraint::Length(1), Constraint::Length(3)]).areas(area);
//         Clear.render(area, buf);

//         Input::default()
//             .block(Block::default().title("Go to Line"))
//             .render(area, buf, &mut state.input);
//     }
// }
