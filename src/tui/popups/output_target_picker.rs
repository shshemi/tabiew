use std::borrow::Cow;

use ratatui::widgets::StatefulWidget;
use strum::IntoEnumIterator;
use strum_macros::{EnumIter, IntoStaticStr};

use crate::tui::pickers::list_picker::{ListPicker, ListPickerState};

#[derive(Debug, Default)]
pub struct OutputTargetPickerState {
    list_picker: ListPickerState,
}

impl OutputTargetPickerState {
    pub fn selected(&self) -> Option<Target> {
        self.list_picker.list().selected().and_then(Target::new)
    }

    pub fn select_next(&mut self) {
        self.list_picker.list_mut().select_next();
    }

    pub fn select_previous(&mut self) {
        self.list_picker.list_mut().select_previous();
    }
}

#[derive(Debug, Default)]
pub struct OutputTargetPicker {}

impl StatefulWidget for OutputTargetPicker {
    type State = OutputTargetPickerState;

    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        state: &mut Self::State,
    ) {
        ListPicker::default()
            .title("Output Target")
            .items(Target::iter().map(Into::into).map(Cow::Borrowed))
            .render(area, buf, &mut state.list_picker)
    }
}

#[derive(Debug, IntoStaticStr, EnumIter)]
pub enum Target {
    File,
    Clipboard,
}

impl Target {
    pub fn new(idx: usize) -> Option<Target> {
        match idx {
            0 => Some(Target::File),
            1 => Some(Target::Clipboard),
            _ => None,
        }
    }
}
