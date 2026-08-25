use std::{borrow::Cow, fmt::Display};

use ratatui::widgets::{Block, List, ListItem, ListState, StatefulWidget};

use crate::sw::{app_default::AppDefault, buffer_ext::BufferExt, rect_ext::RectExt};

#[derive(Debug)]
pub struct ListPickerState {
    list: ListState,
    item_count: usize,
}

impl ListPickerState {
    pub fn select(&mut self, idx: impl Into<Option<usize>>) {
        self.list.select(idx.into());
    }

    pub fn selected(&self) -> Option<usize> {
        self.list.selected()
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
            self.select_last();
        }
    }

    pub fn cycle_down(&mut self) {
        if self.selected() != Some(self.item_count.saturating_sub(1)) {
            self.select_down();
        } else {
            self.select_first();
        }
    }

    pub fn select_first(&mut self) {
        self.list.select_first();
    }

    pub fn select_last(&mut self) {
        self.select(Some(self.item_count.saturating_sub(1)));
    }

    pub fn len(&self) -> usize {
        self.item_count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl Default for ListPickerState {
    fn default() -> Self {
        Self {
            list: ListState::default().with_selected(0.into()),
            item_count: 0,
        }
    }
}

#[derive(Debug)]
pub struct ListPicker<'a, T> {
    items: &'a [T],
    title: Cow<'a, str>,
    darken_bg: bool,
}

impl<'a, T> ListPicker<'a, T> {
    pub fn new(items: &'a [T]) -> Self {
        Self {
            items,
            title: Cow::Borrowed(""),
            darken_bg: true,
        }
    }

    pub fn title(mut self, title: impl Into<Cow<'a, str>>) -> Self {
        self.title = title.into();
        self
    }

    pub fn no_darken_bg(mut self) -> Self {
        self.darken_bg = false;
        self
    }
}

impl<T> StatefulWidget for ListPicker<'_, T>
where
    T: Display,
{
    type State = ListPickerState;

    fn render(
        self,
        _area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        state: &mut Self::State,
    ) {
        state.item_count = self.items.len();

        if self.darken_bg {
            buf.darken();
        }

        let area = buf
            .area
            .palette(self.items.len().saturating_add(2).min(25) as u16);
        buf.clear(area);

        List::app_default()
            .items(
                self.items
                    .iter()
                    .map(ToString::to_string)
                    .map(ListItem::from),
            )
            .block(Block::app_default().title(self.title))
            .render(area, buf, &mut state.list);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::{buffer::Buffer, layout::Rect, style::Color};

    fn render<T: Display>(state: &mut ListPickerState, picker: ListPicker<T>) -> Buffer {
        let area = Rect::new(0, 0, 100, 30);
        let mut buf = Buffer::empty(area);
        picker.render(area, &mut buf, state);
        buf
    }

    fn synced<T: Display>(items: &[T]) -> ListPickerState {
        let mut state = ListPickerState::default();
        render(&mut state, ListPicker::new(items));
        state
    }

    fn content(buf: &Buffer) -> String {
        buf.content().iter().map(|c| c.symbol()).collect()
    }

    mod state {
        use super::*;

        #[test]
        fn a_fresh_state_selects_the_first_item() {
            assert_eq!(ListPickerState::default().selected(), Some(0));
        }

        #[test]
        fn rendering_adopts_the_item_count() {
            let state = synced(&["a", "b", "c"]);
            assert_eq!(state.len(), 3);
            assert!(!state.is_empty());
        }

        #[test]
        fn an_empty_item_list_leaves_the_state_empty() {
            let state = synced::<&str>(&[]);
            assert!(state.is_empty());
        }

        #[test]
        fn selected_is_none_when_deselected() {
            let mut state = synced(&["a", "b"]);
            state.select(None);
            assert_eq!(state.selected(), None);
        }

        #[test]
        fn select_up_and_down_clamp_at_bounds() {
            let mut state = synced(&["a", "b", "c"]);
            state.select(Some(0));
            state.select_up();
            assert_eq!(state.selected(), Some(0));

            state.select(Some(0));
            state.select_down();
            assert_eq!(state.selected(), Some(1));
        }

        #[test]
        fn cycle_up_wraps_to_last_item() {
            let mut state = synced(&["a", "b", "c"]);
            state.select(Some(0));
            state.cycle_up();
            assert_eq!(state.selected(), Some(2));
        }

        #[test]
        fn cycle_up_mid_list_behaves_like_select_up() {
            let mut state = synced(&["a", "b", "c"]);
            state.select(Some(1));
            state.cycle_up();
            assert_eq!(state.selected(), Some(0));
        }

        #[test]
        fn cycle_down_wraps_to_first_item() {
            let mut state = synced(&["a", "b", "c"]);
            state.select(Some(2));
            state.cycle_down();
            assert_eq!(state.selected(), Some(0));
        }

        #[test]
        fn cycle_down_mid_list_behaves_like_select_down() {
            let mut state = synced(&["a", "b", "c"]);
            state.select(Some(0));
            state.cycle_down();
            assert_eq!(state.selected(), Some(1));
        }

        #[test]
        fn select_first_and_last() {
            let mut state = synced(&["a", "b", "c"]);
            state.select(Some(1));
            state.select_last();
            assert_eq!(state.selected(), Some(2));
            state.select_first();
            assert_eq!(state.selected(), Some(0));
        }

        #[test]
        fn a_shorter_item_list_shrinks_the_end() {
            let mut state = synced(&["a", "b", "c"]);
            render(&mut state, ListPicker::new(&["a"]));
            state.select_last();

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

            ListPicker::new(&["a", "b"]).render(area, &mut buf, &mut ListPickerState::default());

            assert_eq!(buf[(0, 0)].bg, Color::Rgb(20, 30, 40));
            assert_eq!(buf[(0, 0)].fg, Color::Rgb(20, 30, 40));
        }

        #[test]
        fn no_darken_bg_leaves_colors_untouched() {
            let area = Rect::new(0, 0, 100, 30);
            let mut buf = Buffer::empty(area);
            buf[(0, 0)].set_bg(Color::Rgb(100, 150, 200));

            ListPicker::new(&["a", "b"])
                .no_darken_bg()
                .render(area, &mut buf, &mut ListPickerState::default());

            assert_eq!(buf[(0, 0)].bg, Color::Rgb(100, 150, 200));
        }

        #[test]
        fn renders_title_and_items() {
            let buf = render(
                &mut ListPickerState::default(),
                ListPicker::new(&["alpha", "beta"]).title("Pick one"),
            );

            let content = content(&buf);
            assert!(content.contains("Pick one"));
            assert!(content.contains("alpha"));
            assert!(content.contains("beta"));
        }
    }
}
