use crossterm::event::KeyEvent;
use ratatui::{buffer::Buffer, layout::Rect};

pub enum AppAction {}

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
    #[allow(unused_variables)]
    fn update(&mut self, action: &AppAction) {}
    #[allow(unused_variables)]
    fn handle(&mut self, event: KeyEvent) -> bool {
        false
    }
    fn tick(&mut self) {}
}
