use crossterm::event::KeyEvent;
use ratatui::{buffer::Buffer, layout::Rect};

use crate::handler::action::Action;

#[derive(Debug, Clone, Copy)]
pub enum FocusState {
    Focused,
    Unfocused,
}

impl FocusState {
    pub fn is_focused(&self) -> bool {
        matches!(self, FocusState::Focused)
    }
}

#[allow(unused_variables)]
pub trait Component {
    fn render(&mut self, area: Rect, buf: &mut Buffer, focus_state: FocusState);
    fn handle(&mut self, event: KeyEvent) -> bool {
        false
    }
    fn update(&mut self, action: &Action) {}
    fn tick(&mut self) {}
}
