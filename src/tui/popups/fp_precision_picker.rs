use std::fmt::Display;

use crossterm::event::KeyCode;

use crate::{
    handler::message::Message,
    misc::{config::config, type_ext::UnwrapOrEnqueueError},
    tui::{component::Component, icons, pickers::list_picker::ListPicker},
};

#[derive(Debug)]
pub struct FpPrecisionPicker {
    list_picker: ListPicker<FpPrecision>,
    rollback: i8,
}

impl Component for FpPrecisionPicker {
    fn render(
        &mut self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        focus_state: crate::tui::component::FocusState,
    ) {
        self.list_picker.render(area, buf, focus_state);
        if let Some(precision) = self.list_picker.selected_item() {
            config().set_fp_precision(precision.value());
        }
    }

    fn handle(&mut self, event: crossterm::event::KeyEvent) -> bool {
        self.list_picker.handle(event)
            || match event.code {
                KeyCode::Esc => {
                    Message::AppDismissOverlay.enqueue();
                    config().set_fp_precision(self.rollback);
                    true
                }
                KeyCode::Enter => {
                    Message::AppDismissOverlay.enqueue();
                    config().store().unwrap_or_enqueue_error();
                    true
                }
                _ => false,
            }
    }
}

impl Default for FpPrecisionPicker {
    fn default() -> Self {
        let items = FpPrecision::all();
        let rollback = config()
            .fp_precision()
            .map_or(-1, |precision| precision as i8);
        let selected = items
            .iter()
            .position(|precision| precision.value() == rollback);
        let mut list_picker = ListPicker::new(items)
            .with_title(icons::PRECISION.into_title("Floating Point Precision"));
        list_picker.select(selected);
        Self {
            list_picker,
            rollback,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum FpPrecision {
    Unset,
    Digits(i8),
}

impl FpPrecision {
    fn all() -> Vec<Self> {
        std::iter::once(Self::Unset)
            .chain((0..=9).map(Self::Digits))
            .collect()
    }

    fn value(&self) -> i8 {
        match self {
            FpPrecision::Unset => -1,
            FpPrecision::Digits(digits) => *digits,
        }
    }
}

impl Display for FpPrecision {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FpPrecision::Unset => write!(f, "Unset"),
            FpPrecision::Digits(digits) => write!(f, "{digits}"),
        }
    }
}
