use std::{borrow::Cow, fmt::Display};

use ratatui::{buffer::Buffer, layout::Rect, widgets::StatefulWidget};
use strum_macros::IntoStaticStr;

use crate::sw::pickers::list_picker::{ListPicker, ListPickerState};

const OPTIONS: [YesNoValue; 2] = [YesNoValue::Yes, YesNoValue::No];

#[derive(Debug, Clone, Copy, PartialEq, Eq, IntoStaticStr)]
pub enum YesNoValue {
    Yes,
    No,
}

impl Display for YesNoValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Into::<&str>::into(self))
    }
}

#[derive(Debug, Default)]
pub struct YesNoPickerState {
    list: ListPickerState,
}

impl YesNoPickerState {
    pub fn list(&self) -> &ListPickerState {
        &self.list
    }

    pub fn list_mut(&mut self) -> &mut ListPickerState {
        &mut self.list
    }

    pub fn value(&self) -> Option<bool> {
        self.selected().map(|yes_no| yes_no == YesNoValue::Yes)
    }

    pub fn selected(&self) -> Option<YesNoValue> {
        self.list
            .selected()
            .and_then(|idx| OPTIONS.get(idx))
            .copied()
    }
}

#[derive(Debug, Default)]
pub struct YesNoPicker<'a> {
    title: Cow<'a, str>,
}

impl<'a> YesNoPicker<'a> {
    pub fn title(mut self, title: impl Into<Cow<'a, str>>) -> Self {
        self.title = title.into();
        self
    }
}

impl StatefulWidget for YesNoPicker<'_> {
    type State = YesNoPickerState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        ListPicker::new(&OPTIONS)
            .title(self.title)
            .render(area, buf, &mut state.list);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn render(state: &mut YesNoPickerState, picker: YesNoPicker) -> Buffer {
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
        fn defaults_to_yes() {
            let state = YesNoPickerState::default();

            assert_eq!(state.selected(), Some(YesNoValue::Yes));
            assert_eq!(state.value(), Some(true));
        }

        #[test]
        fn selecting_the_second_option_is_no() {
            let mut state = YesNoPickerState::default();
            state.list_mut().select(Some(1));

            assert_eq!(state.selected(), Some(YesNoValue::No));
            assert_eq!(state.value(), Some(false));
        }

        #[test]
        fn no_selection_has_no_value() {
            let mut state = YesNoPickerState::default();
            state.list_mut().select(None);

            assert_eq!(state.selected(), None);
            assert_eq!(state.value(), None);
        }

        #[test]
        fn an_out_of_range_selection_has_no_value() {
            let mut state = YesNoPickerState::default();
            state.list_mut().select(Some(7));

            assert_eq!(state.value(), None);
        }

        #[test]
        fn cycling_moves_between_the_two_options() {
            let mut state = YesNoPickerState::default();
            render(&mut state, YesNoPicker::default());

            state.list_mut().cycle_down();
            assert_eq!(state.value(), Some(false));

            state.list_mut().cycle_down();
            assert_eq!(state.value(), Some(true));

            state.list_mut().cycle_up();
            assert_eq!(state.value(), Some(false));
        }

        #[test]
        fn rendering_reports_two_options() {
            let mut state = YesNoPickerState::default();
            render(&mut state, YesNoPicker::default());

            assert_eq!(state.list().len(), 2);
        }
    }

    mod widget {
        use super::*;

        #[test]
        fn renders_both_options() {
            let buf = render(&mut YesNoPickerState::default(), YesNoPicker::default());

            let content = content(&buf);
            assert!(content.contains("Yes"));
            assert!(content.contains("No"));
        }

        #[test]
        fn renders_the_title() {
            let buf = render(
                &mut YesNoPickerState::default(),
                YesNoPicker::default().title("Has Header"),
            );

            assert!(content(&buf).contains("Has Header"));
        }
    }
}
