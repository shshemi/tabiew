use std::fmt::Display;

use strum::IntoEnumIterator;
use strum_macros::{EnumIter, IntoStaticStr};

use crate::tui::{component::Component, pickers::list_picker::ListPicker};

#[derive(Debug)]
pub struct OutputTargetPicker {
    list_picker: ListPicker<Target>,
}

impl OutputTargetPicker {
    pub fn selected_target(&self) -> Option<Target> {
        self.list_picker.selected_item().copied()
    }

    pub fn select_next(&mut self) {
        self.list_picker.list_mut().select_next();
    }

    pub fn select_previous(&mut self) {
        self.list_picker.list_mut().select_previous();
    }
}

impl Component for OutputTargetPicker {
    fn render(
        &mut self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        focus_state: crate::tui::component::FocusState,
    ) {
        self.list_picker.render(area, buf, focus_state);
    }

    fn handle(&mut self, event: crossterm::event::KeyEvent) -> bool {
        self.list_picker.handle(event)
    }
}

impl Default for OutputTargetPicker {
    fn default() -> Self {
        OutputTargetPicker {
            list_picker: ListPicker::new(Target::iter().to_owned().collect()),
        }
    }
}

// #[derive(Debug, Default)]
// pub struct OutputTargetPicker {}

// impl StatefulWidget for OutputTargetPicker {
//     type State = OutputTargetPickerState;

//     fn render(
//         self,
//         area: ratatui::prelude::Rect,
//         buf: &mut ratatui::prelude::Buffer,
//         state: &mut Self::State,
//     ) {
//         ListPicker::default()
//             .title("Output Target")
//             .items(Target::iter().map(Into::into).map(Cow::Borrowed))
//             .render(area, buf, &mut state.list_picker)
//     }
// }

#[derive(Debug, IntoStaticStr, EnumIter, Clone, Copy)]
pub enum Target {
    File,
    Clipboard,
}

impl Display for Target {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Into::<&str>::into(self))
    }
}
