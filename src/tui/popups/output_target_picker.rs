use std::borrow::Cow;

use ratatui::widgets::StatefulWidget;
use strum::IntoEnumIterator;
use strum_macros::{EnumIter, IntoStaticStr};

use crate::tui::pickers::list_picker::{ListPicker, ListPickerState};

#[derive(Debug, Default)]
pub struct OutputTargetState {
    list_picker: ListPickerState,
}

#[derive(Debug, Default)]
pub struct OutputTarget {}

impl StatefulWidget for OutputTarget {
    type State = OutputTargetState;

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
