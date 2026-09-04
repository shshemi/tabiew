use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    layout::Rect,
    widgets::{Block, Widget},
};

use crate::{
    handler::message::Message,
    misc::buffer_ext::BufferExt,
    tui::{
        app_default::{AppDefault, AppTitle},
        component::Component,
        widgets::input::{Input, InputType},
    },
};

const WIDTH: u16 = 32;
const HEIGHT: u16 = 3;
const MAX_LEN: usize = (WIDTH - 2) as usize;

#[derive(Debug)]
pub struct GoToLine {
    rollback: usize,
    input: Input,
}

impl GoToLine {
    pub fn new(rollback: usize) -> Self {
        Self {
            input: Input::default()
                .with_input_type(InputType::Numeric)
                .with_max_len(MAX_LEN),
            rollback,
        }
    }

    pub fn with_value(self, value: usize) -> Self {
        Self {
            rollback: self.rollback,
            input: self.input.with_value(value.to_string()),
        }
    }

    fn value(&self) -> usize {
        self.input.value().parse().unwrap_or(1)
    }
}

impl Component for GoToLine {
    fn render(
        &mut self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        focus_state: crate::tui::component::FocusState,
    ) {
        let area = Rect {
            x: area.right().saturating_sub(WIDTH),
            y: area.y,
            width: WIDTH,
            height: HEIGHT,
        };
        buf.clear(area);
        let area = {
            let block = Block::app_default().app_title("Line");
            let inner = block.inner(area);
            block.render(area, buf);
            inner
        };
        self.input.render(area, buf, focus_state);
    }
    fn handle(&mut self, event: KeyEvent) -> bool {
        if self.input.handle(event) {
            Message::PaneTableSelect(self.value().saturating_sub(1)).enqueue();
            true
        } else {
            match (event.code, event.modifiers) {
                (KeyCode::Enter, KeyModifiers::NONE) => {
                    Message::PaneDismissModal.enqueue();
                    true
                }
                (KeyCode::Esc, KeyModifiers::NONE) => {
                    Message::PaneDismissModal.enqueue();
                    Message::PaneTableSelect(self.rollback).enqueue();
                    true
                }
                _ => false,
            }
        }
    }
}
