use crossterm::event::KeyEvent;
use ratatui::{buffer::Buffer, layout::Rect};

use crate::handler::action::AppAction;

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

pub trait Component {
    fn render(&mut self, area: Rect, buf: &mut Buffer, focus_state: FocusState);
    fn update(&mut self, _action: &AppAction) {}
    fn handle(&mut self, _event: KeyEvent) -> bool {
        false
    }
    fn tick(&mut self) {}
}
