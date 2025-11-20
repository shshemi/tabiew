use ratatui::{buffer::Buffer, crossterm::event::Event, layout::Rect};

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
    fn update(&mut self, action: &AppAction);
    fn handle(&mut self, event: Event);
    fn tick(&mut self);
}
