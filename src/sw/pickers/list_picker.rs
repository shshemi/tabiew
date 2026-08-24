use std::{fmt::Display, marker::PhantomData};

use ratatui::{
    layout::{Constraint, Flex, Layout},
    widgets::{Clear, List, ListItem, ListState, StatefulWidget, Widget},
};

use crate::{
    misc::{color_ext::ColorExt, config::theme},
    tui::widgets::block::Block,
};

#[derive(Debug)]
pub struct ListPickerState<T> {
    title: String,
    list: ListState,
    items: Vec<T>,
    strings: Vec<String>,
    darken_bg: bool,
}

impl<T> ListPickerState<T>
where
    T: Display,
{
    pub fn new(items: Vec<T>) -> Self {
        Self {
            list: ListState::default().with_selected(0.into()),
            strings: items.iter().map(ToString::to_string).collect(),
            title: Default::default(),
            items,
            darken_bg: true,
        }
    }

    pub fn with_title(self, title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            ..self
        }
    }

    pub fn no_darken_bg(self) -> Self {
        Self {
            darken_bg: false,
            ..self
        }
    }

    pub fn select(&mut self, idx: impl Into<Option<usize>>) {
        self.list.select(idx.into());
    }

    pub fn selected(&self) -> Option<usize> {
        self.list.selected()
    }

    pub fn selected_item(&self) -> Option<&T> {
        self.selected().and_then(|i| self.items.get(i))
    }

    pub fn selected_str(&self) -> Option<&str> {
        self.selected()
            .and_then(|i| self.strings.get(i).map(String::as_str))
    }

    pub fn select_up(&mut self) {
        self.list.select_previous();
    }

    pub fn select_down(&mut self) {
        self.list.select_next();
    }

    pub fn cycle_up(&mut self) {
        if self.selected() != Some(0) {
            self.select_up();
        } else {
            self.select(Some(self.items.len().saturating_sub(1)));
        }
    }

    pub fn cycle_down(&mut self) {
        if self.selected() != Some(self.items.len().saturating_sub(1)) {
            self.select_down();
        } else {
            self.select_first();
        }
    }

    pub fn select_first(&mut self) {
        self.list.select_first();
    }

    pub fn select_last(&mut self) {
        self.select(Some(self.items.len().saturating_sub(1)));
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

#[derive(Debug)]
pub struct ListPicker<T>(PhantomData<T>);

impl<T> Default for ListPicker<T> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<T> StatefulWidget for ListPicker<T> {
    type State = ListPickerState<T>;

    fn render(
        self,
        _area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        state: &mut Self::State,
    ) {
        if state.darken_bg {
            for cell in buf.content.iter_mut() {
                cell.bg = cell.bg.darken();
                cell.fg = cell.fg.darken();
            }
        }

        let width = 80;
        let height = state.strings.len().saturating_add(2).min(25) as u16;

        let [area] = Layout::horizontal([Constraint::Length(width)])
            .flex(Flex::Center)
            .areas(buf.area);
        let [_, area] =
            Layout::vertical([Constraint::Length(3), Constraint::Length(height)]).areas(area);
        Clear.render(area, buf);

        StatefulWidget::render(
            List::default()
                .style(theme().text())
                .highlight_style(theme().row_highlighted())
                .items(state.strings.iter().map(|s| ListItem::from(s.as_str())))
                .block(Block::default().title(state.title.as_str()).into_widget()),
            area,
            buf,
            &mut state.list,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::{buffer::Buffer, layout::Rect, style::Color};

    mod state {
        use super::*;

        #[test]
        fn selected_item_and_str_track_selection() {
            let mut state = ListPickerState::new(vec!["a", "b", "c"]);
            state.select(Some(1));
            assert_eq!(state.selected(), Some(1));
            assert_eq!(state.selected_item(), Some(&"b"));
            assert_eq!(state.selected_str(), Some("b"));
        }

        #[test]
        fn selected_is_none_when_deselected() {
            let mut state = ListPickerState::new(vec!["a", "b"]);
            state.select(None);
            assert_eq!(state.selected(), None);
            assert_eq!(state.selected_item(), None);
        }

        #[test]
        fn len_and_is_empty() {
            let state = ListPickerState::new(vec!["a", "b", "c"]);
            assert_eq!(state.len(), 3);
            assert!(!state.is_empty());
            let empty = ListPickerState::<&str>::new(vec![]);
            assert!(empty.is_empty());
        }

        #[test]
        fn select_up_and_down_clamp_at_bounds() {
            let mut state = ListPickerState::new(vec!["a", "b", "c"]);
            state.select(Some(0));
            state.select_up();
            assert_eq!(state.selected(), Some(0));

            state.select(Some(0));
            state.select_down();
            assert_eq!(state.selected(), Some(1));
        }

        #[test]
        fn cycle_up_wraps_to_last_item() {
            let mut state = ListPickerState::new(vec!["a", "b", "c"]);
            state.select(Some(0));
            state.cycle_up();
            assert_eq!(state.selected(), Some(2));
        }

        #[test]
        fn cycle_up_mid_list_behaves_like_select_up() {
            let mut state = ListPickerState::new(vec!["a", "b", "c"]);
            state.select(Some(1));
            state.cycle_up();
            assert_eq!(state.selected(), Some(0));
        }

        #[test]
        fn cycle_down_wraps_to_first_item() {
            let mut state = ListPickerState::new(vec!["a", "b", "c"]);
            state.select(Some(2));
            state.cycle_down();
            assert_eq!(state.selected(), Some(0));
        }

        #[test]
        fn cycle_down_mid_list_behaves_like_select_down() {
            let mut state = ListPickerState::new(vec!["a", "b", "c"]);
            state.select(Some(0));
            state.cycle_down();
            assert_eq!(state.selected(), Some(1));
        }

        #[test]
        fn select_first_and_last() {
            let mut state = ListPickerState::new(vec!["a", "b", "c"]);
            state.select(Some(1));
            state.select_last();
            assert_eq!(state.selected(), Some(2));
            state.select_first();
            assert_eq!(state.selected(), Some(0));
        }
    }

    mod widget {
        use super::*;

        #[test]
        fn darken_bg_scales_colors_outside_popup() {
            let area = Rect::new(0, 0, 100, 30);
            let mut buf = Buffer::empty(area);
            buf[(0, 0)].set_bg(Color::Rgb(100, 150, 200));
            buf[(0, 0)].set_fg(Color::Rgb(100, 150, 200));

            let mut state = ListPickerState::new(vec!["a", "b"]);
            ListPicker::default().render(area, &mut buf, &mut state);

            assert_eq!(buf[(0, 0)].bg, Color::Rgb(20, 30, 40));
            assert_eq!(buf[(0, 0)].fg, Color::Rgb(20, 30, 40));
        }

        #[test]
        fn no_darken_bg_leaves_colors_untouched() {
            let area = Rect::new(0, 0, 100, 30);
            let mut buf = Buffer::empty(area);
            buf[(0, 0)].set_bg(Color::Rgb(100, 150, 200));

            let mut state = ListPickerState::new(vec!["a", "b"]).no_darken_bg();
            ListPicker::default().render(area, &mut buf, &mut state);

            assert_eq!(buf[(0, 0)].bg, Color::Rgb(100, 150, 200));
        }

        #[test]
        fn renders_title_and_items() {
            let area = Rect::new(0, 0, 100, 30);
            let mut buf = Buffer::empty(area);
            let mut state = ListPickerState::new(vec!["alpha", "beta"]).with_title("Pick one");
            ListPicker::default().render(area, &mut buf, &mut state);

            let content = buf
                .content()
                .iter()
                .map(|c| c.symbol())
                .collect::<String>();
            assert!(content.contains("Pick one"));
            assert!(content.contains("alpha"));
            assert!(content.contains("beta"));
        }
    }
}
