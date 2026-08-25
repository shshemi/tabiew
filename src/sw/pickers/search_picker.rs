use std::{
    borrow::Cow,
    fmt::Display,
    hash::{DefaultHasher, Hash, Hasher},
};

use itertools::Itertools;
use ratatui::{
    layout::Constraint,
    text::{Line, Span},
    widgets::{Block, List, ListItem, ListState, StatefulWidget},
};
use crate::{
    misc::config::theme,
    sw::{
        app_default::AppDefault,
        buffer_ext::BufferExt,
        rect_ext::RectExt,
        widgets::{
            input::{Input, InputState},
            split::Split,
        },
    },
};

#[derive(Debug)]
pub struct SearchPickerState {
    input: InputState,
    list: ListState,
    indices: Vec<(usize, String, Vec<usize>)>,
    query_hash: u64,
    item_count: usize,
}

impl SearchPickerState {
    pub fn input(&self) -> &InputState {
        &self.input
    }

    pub fn input_mut(&mut self) -> &mut InputState {
        &mut self.input
    }

    pub fn text(&self) -> &str {
        self.input.value()
    }

    pub fn select(&mut self, idx: impl Into<Option<usize>>) {
        self.list.select(idx.into());
    }

    pub fn selected(&self) -> Option<usize> {
        if self.text().is_empty() {
            self.list.selected()
        } else {
            self.list
                .selected()
                .and_then(|idx| self.indices.get(idx))
                .map(|(idx, _, _)| *idx)
        }
    }

    pub fn len(&self) -> usize {
        if self.text().is_empty() {
            self.item_count
        } else {
            self.indices.len()
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn select_up(&mut self) {
        self.list.select_previous();
    }

    pub fn select_down(&mut self) {
        self.list.select_next();
    }

    pub fn select_first(&mut self) {
        self.list.select_first();
    }

    pub fn select_last(&mut self) {
        self.select(Some(self.len().saturating_sub(1)));
    }

    pub fn cycle_up(&mut self) {
        if self.list.selected() != Some(0) {
            self.select_up();
        } else {
            self.select_last();
        }
    }

    pub fn cycle_down(&mut self) {
        if self.list.selected() != Some(self.len().saturating_sub(1)) {
            self.select_down();
        } else {
            self.select_first();
        }
    }

    fn sync<T: Display>(&mut self, items: &[T]) {
        self.item_count = items.len();

        let query = self.input.value();
        if query.is_empty() {
            return;
        }

        let mut hasher = DefaultHasher::new();
        query.hash(&mut hasher);
        let query_hash = hasher.finish();

        if self.query_hash != query_hash {
            self.indices.clear();
            for (idx, item) in items.iter().enumerate() {
                let text = item.to_string();
                if let Some(pos) = subsequence_pos(&text, query) {
                    self.indices.push((idx, text, pos));
                }
            }
            self.indices
                .sort_by_key(|(_, text, _)| text.chars().count());
            self.query_hash = query_hash;
        }
    }
}

impl Default for SearchPickerState {
    fn default() -> Self {
        Self {
            input: InputState::default(),
            list: ListState::default().with_selected(Some(0)),
            indices: Vec::new(),
            query_hash: 0,
            item_count: 0,
        }
    }
}

#[derive(Debug)]
pub struct SearchPicker<'a, T> {
    items: &'a [T],
    title: Cow<'a, str>,
    darken_bg: bool,
}

impl<'a, T> SearchPicker<'a, T> {
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

impl<T> StatefulWidget for SearchPicker<'_, T>
where
    T: Display,
{
    type State = SearchPickerState;

    fn render(
        self,
        _area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        state: &mut Self::State,
    ) {
        state.sync(self.items);

        if self.darken_bg {
            buf.darken();
        }

        let items = if state.input.value().is_empty() {
            self.items
                .iter()
                .map(ToString::to_string)
                .map(ListItem::new)
                .collect_vec()
        } else {
            state
                .indices
                .iter()
                .map(|(_, text, matches)| match_item(text, matches))
                .collect_vec()
        };

        let area = buf
            .area
            .palette(items.len().saturating_add(4).min(25) as u16);
        buf.clear(area);

        let [input_area, list_area] = Split::vertical([Constraint::Length(1), Constraint::Fill(1)])
            .block(Block::app_default().title(self.title))
            .split(buf, area);

        Input::default().render(input_area, buf, &mut state.input);

        *state.list.offset_mut() = state
            .list
            .offset()
            .min(items.len().saturating_sub(list_area.height as usize));
        if state.list.selected().is_none() && !items.is_empty() {
            state.list.select(Some(0));
        }

        List::app_default()
            .items(items)
            .render(list_area, buf, &mut state.list);
    }
}

fn match_item<'a>(text: &'a str, matches: &[usize]) -> ListItem<'a> {
    let mut spans = text
        .char_indices()
        .map(|(i, c)| (i, i + c.len_utf8()))
        .map(|(start, end)| (&text[start..end], theme().text()))
        .collect_vec();
    for i in matches {
        if let Some((_, style)) = spans.get_mut(*i) {
            *style = theme().text_highlighted();
        }
    }

    ListItem::new(Line::from(
        spans
            .into_iter()
            .map(|(content, style)| Span::styled(content, style))
            .collect_vec(),
    ))
}

fn subsequence_pos(larger: &str, other: &str) -> Option<Vec<usize>> {
    let mut idxs = Vec::with_capacity(other.chars().count());
    let mut larger_iter = larger.chars().enumerate();
    for oc in other.chars() {
        if let Some((pos, _)) = larger_iter.find(|(_, lc)| lc.eq_ignore_ascii_case(&oc)) {
            idxs.push(pos);
        } else {
            return None;
        }
    }
    Some(idxs)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::{buffer::Buffer, layout::Rect, style::Color};

    fn typed(state: &mut SearchPickerState, text: &str) {
        for c in text.chars() {
            state.input_mut().insert(c);
        }
    }

    fn render<T: Display>(state: &mut SearchPickerState, picker: SearchPicker<T>) -> Buffer {
        let area = Rect::new(0, 0, 100, 30);
        let mut buf = Buffer::empty(area);
        picker.render(area, &mut buf, state);
        buf
    }

    fn synced<T: Display>(items: &[T]) -> SearchPickerState {
        let mut state = SearchPickerState::default();
        render(&mut state, SearchPicker::new(items));
        state
    }

    fn content(buf: &Buffer) -> String {
        buf.content().iter().map(|c| c.symbol()).collect()
    }

    mod state {
        use super::*;

        #[test]
        fn unfiltered_selection_is_the_item_index() {
            let mut state = synced(&["alpha", "beta", "gamma"]);
            state.select(Some(1));
            assert_eq!(state.selected(), Some(1));
        }

        #[test]
        fn rendering_adopts_the_item_count() {
            let state = synced(&["a", "b", "c"]);
            assert_eq!(state.len(), 3);
            assert!(!state.is_empty());
        }

        #[test]
        fn the_item_count_follows_the_rendered_list() {
            let mut state = SearchPickerState::default();
            render(&mut state, SearchPicker::new(&["a", "b", "c"]));
            assert_eq!(state.len(), 3);

            render(&mut state, SearchPicker::new(&["a"]));
            assert_eq!(state.len(), 1);
        }

        #[test]
        fn an_empty_item_list_leaves_the_state_empty() {
            let state = synced::<&str>(&[]);
            assert!(state.is_empty());
        }

        #[test]
        fn cycle_up_wraps_to_last_item() {
            let mut state = synced(&["a", "b", "c"]);
            state.select(Some(0));
            state.cycle_up();
            assert_eq!(state.selected(), Some(2));
        }

        #[test]
        fn cycle_down_wraps_to_first_item() {
            let mut state = synced(&["a", "b", "c"]);
            state.select(Some(2));
            state.cycle_down();
            assert_eq!(state.selected(), Some(0));
        }

        #[test]
        fn select_first_and_last() {
            let mut state = synced(&["a", "b", "c"]);
            state.select_last();
            assert_eq!(state.selected(), Some(2));
            state.select_first();
            assert_eq!(state.selected(), Some(0));
        }

        #[test]
        fn text_reflects_typed_input() {
            let mut state = SearchPickerState::default();
            typed(&mut state, "ab");
            assert_eq!(state.text(), "ab");
        }
    }

    mod filtering {
        use super::*;

        #[test]
        fn len_reflects_matches_after_render() {
            let items = ["alpha", "beta", "gamma"];
            let mut state = SearchPickerState::default();
            typed(&mut state, "mm");
            render(&mut state, SearchPicker::new(&items));

            assert_eq!(state.len(), 1);
        }

        #[test]
        fn selected_maps_through_filter_to_original_index() {
            let items = ["alpha", "beta", "gamma"];
            let mut state = SearchPickerState::default();
            typed(&mut state, "mm");
            render(&mut state, SearchPicker::new(&items));

            state.select(Some(0));
            assert_eq!(state.selected(), Some(2));
        }

        #[test]
        fn subsequence_match_is_case_insensitive_and_non_contiguous() {
            let items = ["Alpha", "beta"];
            let mut state = SearchPickerState::default();
            typed(&mut state, "AP");
            render(&mut state, SearchPicker::new(&items));

            state.select(Some(0));
            assert_eq!(state.selected(), Some(0));
            assert_eq!(state.len(), 1);
        }

        #[test]
        fn no_match_leaves_zero_length() {
            let items = ["alpha", "beta"];
            let mut state = SearchPickerState::default();
            typed(&mut state, "zzz");
            render(&mut state, SearchPicker::new(&items));

            assert_eq!(state.len(), 0);
            assert!(state.is_empty());
        }

        #[test]
        fn shorter_matches_sort_first() {
            let items = ["aaaa", "aa", "aaa"];
            let mut state = SearchPickerState::default();
            typed(&mut state, "a");
            render(&mut state, SearchPicker::new(&items));

            state.select(Some(0));
            assert_eq!(state.selected(), Some(1));
        }

        #[test]
        fn sorting_counts_characters_not_bytes() {
            // "aéé" is 5 bytes but 3 chars; "abcd" is 4 bytes and 4 chars
            let items = ["abcd", "aéé"];
            let mut state = SearchPickerState::default();
            typed(&mut state, "a");
            render(&mut state, SearchPicker::new(&items));

            state.select(Some(0));
            assert_eq!(state.selected(), Some(1));
        }

        #[test]
        fn sorting_counts_characters_not_display_columns() {
            // "a日日" is 3 chars but 5 columns; "abcd" is 4 chars and 4 columns
            let items = ["abcd", "a日日"];
            let mut state = SearchPickerState::default();
            typed(&mut state, "a");
            render(&mut state, SearchPicker::new(&items));

            state.select(Some(0));
            assert_eq!(state.selected(), Some(1));
        }

        #[test]
        fn an_unchanged_query_keeps_the_cached_matches() {
            let items = ["alpha", "beta"];
            let mut state = SearchPickerState::default();
            typed(&mut state, "a");
            render(&mut state, SearchPicker::new(&items));

            state.indices = vec![(0, "sentinel".to_owned(), Vec::new())];
            render(&mut state, SearchPicker::new(&items));

            assert_eq!(state.indices.len(), 1);
            assert_eq!(state.indices[0].1, "sentinel");
        }

        #[test]
        fn matched_text_is_cached_alongside_the_index() {
            let items = ["alpha", "beta"];
            let mut state = SearchPickerState::default();
            typed(&mut state, "et");
            render(&mut state, SearchPicker::new(&items));

            assert_eq!(state.indices.len(), 1);
            assert_eq!(state.indices[0].0, 1);
            assert_eq!(state.indices[0].1, "beta");
        }

        #[test]
        fn refiltering_after_query_change_updates_results() {
            let items = ["alpha", "beta", "gamma"];
            let mut state = SearchPickerState::default();
            typed(&mut state, "mm");
            render(&mut state, SearchPicker::new(&items));
            assert_eq!(state.len(), 1);

            state.input_mut().delete_prev();
            state.input_mut().delete_prev();
            typed(&mut state, "et");
            render(&mut state, SearchPicker::new(&items));

            state.select(Some(0));
            assert_eq!(state.selected(), Some(1));
        }
    }

    mod widget {
        use super::*;

        #[test]
        fn renders_title_and_all_items_when_query_is_empty() {
            let buf = render(
                &mut SearchPickerState::default(),
                SearchPicker::new(&["alpha", "beta"]).title("Format"),
            );

            let content = content(&buf);
            assert!(content.contains("Format"));
            assert!(content.contains("alpha"));
            assert!(content.contains("beta"));
        }

        #[test]
        fn renders_only_matching_items_when_filtered() {
            let items = ["alpha", "beta"];
            let mut state = SearchPickerState::default();
            typed(&mut state, "mm");
            let buf = render(&mut state, SearchPicker::new(&items));

            let content = content(&buf);
            assert!(!content.contains("alpha"));
            assert!(!content.contains("beta"));
        }

        #[test]
        fn renders_typed_query_in_input_row() {
            let items = ["alpha"];
            let mut state = SearchPickerState::default();
            typed(&mut state, "alp");
            let buf = render(&mut state, SearchPicker::new(&items));

            assert!(content(&buf).contains("alp"));
        }

        #[test]
        fn height_grows_with_item_count() {
            let area = Rect::new(0, 0, 100, 30);

            assert_eq!(area.palette(2u16.saturating_add(4)).height, 6);
            assert_eq!(area.palette(4u16.saturating_add(4)).height, 8);
        }

        #[test]
        fn selects_first_item_when_nothing_is_selected() {
            let items = ["a", "b"];
            let mut state = SearchPickerState::default();
            state.select(None);
            render(&mut state, SearchPicker::new(&items));

            assert_eq!(state.selected(), Some(0));
        }

        #[test]
        fn darken_bg_scales_colors_outside_popup() {
            let area = Rect::new(0, 0, 100, 30);
            let mut buf = Buffer::empty(area);
            buf[(0, 0)].set_bg(Color::Rgb(100, 150, 200));
            buf[(0, 0)].set_fg(Color::Rgb(100, 150, 200));

            SearchPicker::new(&["a", "b"]).render(
                area,
                &mut buf,
                &mut SearchPickerState::default(),
            );

            assert_eq!(buf[(0, 0)].bg, Color::Rgb(20, 30, 40));
            assert_eq!(buf[(0, 0)].fg, Color::Rgb(20, 30, 40));
        }

        #[test]
        fn no_darken_bg_leaves_colors_untouched() {
            let area = Rect::new(0, 0, 100, 30);
            let mut buf = Buffer::empty(area);
            buf[(0, 0)].set_bg(Color::Rgb(100, 150, 200));

            SearchPicker::new(&["a", "b"]).no_darken_bg().render(
                area,
                &mut buf,
                &mut SearchPickerState::default(),
            );

            assert_eq!(buf[(0, 0)].bg, Color::Rgb(100, 150, 200));
        }

        #[test]
        fn input_and_list_are_separated_by_a_joined_divider() {
            let area = Rect::new(0, 0, 100, 30);
            let mut buf = Buffer::empty(area);
            SearchPicker::new(&["a", "b"]).render(
                area,
                &mut buf,
                &mut SearchPickerState::default(),
            );

            let popup = area.palette(6);
            let divider_y = popup.y + 2;
            assert_eq!(buf[(popup.x, divider_y)].symbol(), "\u{251c}");
            assert_eq!(buf[(popup.right() - 1, divider_y)].symbol(), "\u{2524}");
        }
    }

    mod match_item {
        use super::*;

        fn cells(text: &str, matches: &[usize]) -> Vec<(String, Option<Color>)> {
            let area = Rect::new(0, 0, 12, 1);
            let mut buf = Buffer::empty(area);
            let mut list = ListState::default();
            StatefulWidget::render(
                List::new([super::super::match_item(text, matches)]),
                area,
                &mut buf,
                &mut list,
            );
            (0..text.chars().count() as u16)
                .map(|x| (buf[(x, 0)].symbol().to_owned(), Some(buf[(x, 0)].fg)))
                .collect()
        }

        fn base() -> Option<Color> {
            theme().text().fg
        }

        fn hit() -> Option<Color> {
            theme().text_highlighted().fg
        }

        #[test]
        fn renders_each_character_of_the_text() {
            assert_eq!(
                cells("abc", &[]).into_iter().map(|(c, _)| c).collect_vec(),
                vec!["a", "b", "c"]
            );
        }

        #[test]
        fn applies_match_style_to_marked_positions_only() {
            assert_eq!(
                cells("abc", &[1]),
                vec![
                    ("a".to_owned(), base()),
                    ("b".to_owned(), hit()),
                    ("c".to_owned(), base()),
                ]
            );
        }

        #[test]
        fn positions_are_char_indices_not_byte_offsets() {
            let cells = cells("h\u{e9}llo", &[1]);

            assert_eq!(cells[1].0, "\u{e9}");
            assert_eq!(cells[1].1, hit());
        }

        #[test]
        fn out_of_range_match_is_ignored() {
            assert!(cells("ab", &[5]).into_iter().all(|(_, fg)| fg == base()));
        }
    }
}
