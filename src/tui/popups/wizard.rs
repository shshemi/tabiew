use crossterm::event::{KeyCode, KeyModifiers};

use crate::{handler::message::Message, tui::component::Component};

pub trait WizardState {
    fn next(self) -> Self;
    fn responder(&mut self) -> Option<&mut dyn Component>;
    fn finalize(&self) -> bool;
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
        if let Some(responder) = self.state.as_mut().and_then(|s| s.responder()) {
            responder.render(area, buf, focus_state);
        }
    }

    fn handle(&mut self, event: crossterm::event::KeyEvent) -> bool {
        if let Some(responder) = self.state.as_mut().and_then(|s| s.responder()) {
            responder.handle(event)
                || match (event.code, event.modifiers) {
                    (KeyCode::Esc, KeyModifiers::NONE) => {
                        Message::PaneDismissModal.enqueue();
                        true
                    }
                    (KeyCode::Enter, KeyModifiers::NONE) => {
                        self.state = self.state.take().map(|s| s.next());
                        if let Some(state) = self.state.as_ref()
                            && state.finalize()
                        {
                            Message::PaneDismissModal.enqueue();
                        }
                        true
                    }
                    _ => false,
                }
        } else {
            false
        }
    }
}
