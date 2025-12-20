use crossterm::event::{KeyCode, KeyModifiers};

use crate::{handler::message::Message, tui::component::Component};

pub trait WizardState {
    fn next(self) -> Self;
    fn responder(&mut self) -> &mut dyn Component;
}

#[derive(Debug)]
pub struct Wizard<W: WizardState> {
    state: Option<W>,
}

impl<W> Wizard<W>
where
    W: WizardState,
{
    pub fn new(state: W) -> Self {
        Self { state: Some(state) }
    }
}

impl<W> Component for Wizard<W>
where
    W: WizardState,
{
    fn render(
        &mut self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        focus_state: crate::tui::component::FocusState,
    ) {
        if let Some(responder) = self.state.as_mut().map(|s| s.responder()) {
            responder.render(area, buf, focus_state);
        }
    }

    fn handle(&mut self, event: crossterm::event::KeyEvent) -> bool {
        self.state
            .as_mut()
            .map(|s| s.responder().handle(event))
            .unwrap_or(false)
            || match (event.code, event.modifiers) {
                (KeyCode::Esc, KeyModifiers::NONE) => {
                    Message::PaneDismissModal.enqueue();
                    Message::AppDismissOverlay.enqueue();
                    true
                }
                (KeyCode::Enter, KeyModifiers::NONE) => {
                    self.state = self.state.take().map(|s| s.next());
                    true
                }
                _ => false,
            }
    }
}

impl<State> Default for Wizard<State>
where
    State: Default + WizardState,
{
    fn default() -> Self {
        Self::new(Default::default())
    }
}
