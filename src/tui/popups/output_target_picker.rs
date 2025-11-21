use strum::IntoEnumIterator;
use strum_macros::{EnumIter, IntoStaticStr};

use crate::tui::{component::Component, pickers::list_picker::ListPicker};

#[derive(Debug)]
pub struct OutputTargetPicker {
    list_picker: ListPicker,
}

impl OutputTargetPicker {
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
            list_picker: ListPicker::new(
                Target::iter()
                    .map(Into::<&str>::into)
                    .map(str::to_string)
                    .to_owned()
                    .collect(),
            ),
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
