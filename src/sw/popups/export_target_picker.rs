use std::{borrow::Cow, fmt::Display};

use ratatui::{buffer::Buffer, layout::Rect, widgets::StatefulWidget};
use strum_macros::IntoStaticStr;

use crate::sw::pickers::list_picker::{ListPicker, ListPickerState};

const TARGETS: [Target; 2] = [Target::File, Target::Clipboard];

#[derive(Debug, Clone, Copy, PartialEq, Eq, IntoStaticStr)]
pub enum Target {
    File,
    Clipboard,
}

impl Display for Target {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Into::<&str>::into(self))
    }
}

#[derive(Debug, Default)]
pub struct ExportTargetPickerState {
    list: ListPickerState,
}

impl ExportTargetPickerState {
    pub fn list(&self) -> &ListPickerState {
        &self.list
    }

    pub fn list_mut(&mut self) -> &mut ListPickerState {
        &mut self.list
    }

    pub fn selected(&self) -> Option<Target> {
        self.list
            .selected()
            .and_then(|idx| TARGETS.get(idx))
            .copied()
    }
}

#[derive(Debug)]
pub struct ExportTargetPicker<'a> {
    title: Cow<'a, str>,
}

impl<'a> ExportTargetPicker<'a> {
    pub fn title(mut self, title: impl Into<Cow<'a, str>>) -> Self {
        self.title = title.into();
        self
    }
}

impl Default for ExportTargetPicker<'_> {
    fn default() -> Self {
        Self {
            title: Cow::Borrowed("Export Target"),
        }
    }
}

impl StatefulWidget for ExportTargetPicker<'_> {
    type State = ExportTargetPickerState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        ListPicker::new(&TARGETS)
            .title(self.title)
            .render(area, buf, &mut state.list);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn render(state: &mut ExportTargetPickerState, picker: ExportTargetPicker) -> Buffer {
        let area = Rect::new(0, 0, 100, 30);
        let mut buf = Buffer::empty(area);
        picker.render(area, &mut buf, state);
        buf
    }

    fn content(buf: &Buffer) -> String {
        buf.content().iter().map(|c| c.symbol()).collect()
    }

    mod state {
        use super::*;

        #[test]
        fn defaults_to_file() {
            assert_eq!(
                ExportTargetPickerState::default().selected(),
                Some(Target::File)
            );
        }

        #[test]
        fn selecting_the_second_option_is_clipboard() {
            let mut state = ExportTargetPickerState::default();
            state.list_mut().select(Some(1));

            assert_eq!(state.selected(), Some(Target::Clipboard));
        }

        #[test]
        fn no_selection_has_no_target() {
            let mut state = ExportTargetPickerState::default();
            state.list_mut().select(None);

            assert_eq!(state.selected(), None);
        }

        #[test]
        fn an_out_of_range_selection_has_no_target() {
            let mut state = ExportTargetPickerState::default();
            state.list_mut().select(Some(9));

            assert_eq!(state.selected(), None);
        }

        #[test]
        fn cycling_moves_between_the_two_targets() {
            let mut state = ExportTargetPickerState::default();
            render(&mut state, ExportTargetPicker::default());

            state.list_mut().cycle_down();
            assert_eq!(state.selected(), Some(Target::Clipboard));

            state.list_mut().cycle_down();
            assert_eq!(state.selected(), Some(Target::File));
        }

        #[test]
        fn rendering_reports_two_targets() {
            let mut state = ExportTargetPickerState::default();
            render(&mut state, ExportTargetPicker::default());

            assert_eq!(state.list().len(), 2);
        }
    }

    mod widget {
        use super::*;

        #[test]
        fn renders_both_targets() {
            let buf = render(
                &mut ExportTargetPickerState::default(),
                ExportTargetPicker::default(),
            );

            let content = content(&buf);
            assert!(content.contains("File"));
            assert!(content.contains("Clipboard"));
        }

        #[test]
        fn defaults_to_an_export_target_title() {
            let buf = render(
                &mut ExportTargetPickerState::default(),
                ExportTargetPicker::default(),
            );

            assert!(content(&buf).contains("Export Target"));
        }

        #[test]
        fn the_title_is_overridable() {
            let buf = render(
                &mut ExportTargetPickerState::default(),
                ExportTargetPicker::default().title("Write To"),
            );

            let content = content(&buf);
            assert!(content.contains("Write To"));
            assert!(!content.contains("Export Target"));
        }
    }
}
