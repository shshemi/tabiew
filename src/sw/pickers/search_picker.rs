use std::{
    borrow::Cow,
    fmt::Display,
    hash::{DefaultHasher, Hash, Hasher},
    marker::PhantomData,
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
pub struct SearchPickerState<T> {
    input: InputState,
    list: ListState,
    cache: CachedFilter<T>,
}

impl<T> SearchPickerState<T> {
    pub fn new(items: Vec<T>) -> Self {
        Self {
            input: InputState::default(),
            list: ListState::default().with_selected(Some(0)),
            cache: CachedFilter {
                items,
                indices: Default::default(),
                query_hash: 0,
            },
        }
    }

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
                .and_then(|idx| self.cache.indices.get(idx))
                .map(|(idx, _)| *idx)
        }
    }

    pub fn selected_item(&self) -> Option<&T> {
        self.selected().and_then(|idx| self.cache.items.get(idx))
    }

    pub fn into_items(self) -> Vec<T> {
        self.cache.items
    }

    pub fn len(&self) -> usize {
        if self.text().is_empty() {
            self.cache.items.len()
        } else {
            self.cache.indices.len()
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
}

impl<T> SearchPickerState<T>
where
    T: Display,
{
    pub fn selected_str(&self) -> Option<String> {
        self.selected_item().map(ToString::to_string)
    }
}

#[derive(Debug)]
pub struct SearchPicker<'a, T> {
    title: Cow<'a, str>,
    darken_bg: bool,
    marker: PhantomData<T>,
}

impl<'a, T> SearchPicker<'a, T> {
    pub fn title(mut self, title: impl Into<Cow<'a, str>>) -> Self {
        self.title = title.into();
        self
    }

    pub fn no_darken_bg(mut self) -> Self {
        self.darken_bg = false;
        self
    }
}

impl<'a, T> Default for SearchPicker<'a, T> {
    fn default() -> Self {
        Self {
            title: Cow::Borrowed(""),
            darken_bg: true,
            marker: PhantomData,
        }
    }
}

impl<'a, T> StatefulWidget for SearchPicker<'a, T>
where
    T: Display,
{
    type State = SearchPickerState<T>;

    fn render(
        self,
        _area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        state: &mut Self::State,
    ) {
        if self.darken_bg {
            buf.darken();
        }

        let strings = state
            .cache
            .items
            .iter()
            .map(ToString::to_string)
            .collect_vec();
        let items = if state.input.value().is_empty() {
            strings
                .iter()
                .map(|item| ListItem::new(item.as_str()))
                .collect_vec()
        } else {
            state
                .cache
                .query(state.input.value())
                .iter()
                .map(|(idx, matches)| match_item(&strings[*idx], matches))
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

#[derive(Debug)]
struct CachedFilter<T> {
    items: Vec<T>,
    indices: Vec<(usize, Vec<usize>)>,
    query_hash: u64,
}

impl<T> CachedFilter<T>
where
    T: Display,
{
    fn query(&mut self, query: &str) -> &[(usize, Vec<usize>)] {
        let mut hasher = DefaultHasher::new();
        query.hash(&mut hasher);
        let query_hash = hasher.finish();

        if self.query_hash != query_hash {
            self.indices.clear();
            self.indices.extend(
                self.items
                    .iter()
                    .enumerate()
                    .filter_map(|(idx, item)| {
                        let text = item.to_string();
                        subsequence_pos(&text, query).map(|pos| (idx, text.len(), pos))
                    })
                    .sorted_by_key(|(_, len, _)| *len)
                    .map(|(idx, _, pos)| (idx, pos)),
            );
            self.query_hash = query_hash;
        }
        &self.indices
    }
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

    fn typed<T>(state: &mut SearchPickerState<T>, text: &str) {
        for c in text.chars() {
            state.input_mut().insert(c);
        }
    }

    fn render_once<T: Display>(state: &mut SearchPickerState<T>) -> Buffer {
        let area = Rect::new(0, 0, 100, 30);
        let mut buf = Buffer::empty(area);
        SearchPicker::default().render(area, &mut buf, state);
        buf
    }

    fn content(buf: &Buffer) -> String {
        buf.content().iter().map(|c| c.symbol()).collect()
    }

    mod state {
        use super::*;

        #[test]
        fn unfiltered_selection_maps_directly_to_items() {
            let mut state = SearchPickerState::new(vec!["alpha", "beta", "gamma"]);
            state.select(Some(1));
            assert_eq!(state.selected(), Some(1));
            assert_eq!(state.selected_item(), Some(&"beta"));
            assert_eq!(state.selected_str().as_deref(), Some("beta"));
        }

        #[test]
        fn unfiltered_len_is_item_count() {
            let state = SearchPickerState::new(vec!["a", "b", "c"]);
            assert_eq!(state.len(), 3);
            assert!(!state.is_empty());
        }

        #[test]
        fn empty_state_is_empty() {
            let state = SearchPickerState::<&str>::new(vec![]);
            assert!(state.is_empty());
            assert_eq!(state.selected_item(), None);
        }

        #[test]
        fn cycle_up_wraps_to_last_item() {
            let mut state = SearchPickerState::new(vec!["a", "b", "c"]);
            state.select(Some(0));
            state.cycle_up();
            assert_eq!(state.selected(), Some(2));
        }

        #[test]
        fn cycle_down_wraps_to_first_item() {
            let mut state = SearchPickerState::new(vec!["a", "b", "c"]);
            state.select(Some(2));
            state.cycle_down();
            assert_eq!(state.selected(), Some(0));
        }

        #[test]
        fn select_first_and_last() {
            let mut state = SearchPickerState::new(vec!["a", "b", "c"]);
            state.select_last();
            assert_eq!(state.selected(), Some(2));
            state.select_first();
            assert_eq!(state.selected(), Some(0));
        }

        #[test]
        fn text_reflects_typed_input() {
            let mut state = SearchPickerState::new(vec!["a"]);
            typed(&mut state, "ab");
            assert_eq!(state.text(), "ab");
        }

        #[test]
        fn into_items_returns_original_items() {
            let state = SearchPickerState::new(vec!["a", "b"]);
            assert_eq!(state.into_items(), vec!["a", "b"]);
        }
    }

    mod filtering {
        use super::*;

        #[test]
        fn len_reflects_matches_after_render() {
            let mut state = SearchPickerState::new(vec!["alpha", "beta", "gamma"]);
            typed(&mut state, "mm");
            render_once(&mut state);

            assert_eq!(state.len(), 1);
        }

        #[test]
        fn selected_maps_through_filter_to_original_index() {
            let mut state = SearchPickerState::new(vec!["alpha", "beta", "gamma"]);
            typed(&mut state, "mm");
            render_once(&mut state);

            state.select(Some(0));
            assert_eq!(state.selected(), Some(2));
            assert_eq!(state.selected_item(), Some(&"gamma"));
        }

        #[test]
        fn subsequence_match_is_case_insensitive_and_non_contiguous() {
            let mut state = SearchPickerState::new(vec!["Alpha", "beta"]);
            typed(&mut state, "AP");
            render_once(&mut state);

            state.select(Some(0));
            assert_eq!(state.selected_item(), Some(&"Alpha"));
            assert_eq!(state.len(), 1);
        }

        #[test]
        fn no_match_leaves_zero_length() {
            let mut state = SearchPickerState::new(vec!["alpha", "beta"]);
            typed(&mut state, "zzz");
            render_once(&mut state);

            assert_eq!(state.len(), 0);
            assert!(state.is_empty());
        }

        #[test]
        fn shorter_matches_sort_first() {
            let mut state = SearchPickerState::new(vec!["aaaa", "aa", "aaa"]);
            typed(&mut state, "a");
            render_once(&mut state);

            state.select(Some(0));
            assert_eq!(state.selected_item(), Some(&"aa"));
        }

        #[test]
        fn refiltering_after_query_change_updates_results() {
            let mut state = SearchPickerState::new(vec!["alpha", "beta", "gamma"]);
            typed(&mut state, "mm");
            render_once(&mut state);
            assert_eq!(state.len(), 1);

            state.input_mut().delete_prev();
            state.input_mut().delete_prev();
            typed(&mut state, "et");
            render_once(&mut state);

            state.select(Some(0));
            assert_eq!(state.selected_item(), Some(&"beta"));
        }
    }

    mod widget {
        use super::*;

        #[test]
        fn renders_title_and_all_items_when_query_is_empty() {
            let mut state = SearchPickerState::new(vec!["alpha", "beta"]);
            let area = Rect::new(0, 0, 100, 30);
            let mut buf = Buffer::empty(area);
            SearchPicker::default()
                .title("Format")
                .render(area, &mut buf, &mut state);

            let content = content(&buf);
            assert!(content.contains("Format"));
            assert!(content.contains("alpha"));
            assert!(content.contains("beta"));
        }

        #[test]
        fn renders_only_matching_items_when_filtered() {
            let mut state = SearchPickerState::new(vec!["alpha", "beta"]);
            typed(&mut state, "mm");
            let buf = render_once(&mut state);

            let content = content(&buf);
            assert!(!content.contains("alpha"));
            assert!(!content.contains("beta"));
        }

        #[test]
        fn renders_typed_query_in_input_row() {
            let mut state = SearchPickerState::new(vec!["alpha"]);
            typed(&mut state, "alp");
            let buf = render_once(&mut state);

            assert!(content(&buf).contains("alp"));
        }

        #[test]
        fn height_grows_with_item_count() {
            let two = SearchPickerState::new(vec!["a", "b"]);
            let four = SearchPickerState::new(vec!["a", "b", "c", "d"]);
            let area = Rect::new(0, 0, 100, 30);

            assert_eq!(area.palette(two.len().saturating_add(4) as u16).height, 6);
            assert_eq!(area.palette(four.len().saturating_add(4) as u16).height, 8);
        }

        #[test]
        fn selects_first_item_when_nothing_is_selected() {
            let mut state = SearchPickerState::new(vec!["a", "b"]);
            state.select(None);
            render_once(&mut state);

            assert_eq!(state.selected(), Some(0));
        }

        #[test]
        fn darken_bg_scales_colors_outside_popup() {
            let area = Rect::new(0, 0, 100, 30);
            let mut buf = Buffer::empty(area);
            buf[(0, 0)].set_bg(Color::Rgb(100, 150, 200));
            buf[(0, 0)].set_fg(Color::Rgb(100, 150, 200));

            let mut state = SearchPickerState::new(vec!["a", "b"]);
            SearchPicker::default().render(area, &mut buf, &mut state);

            assert_eq!(buf[(0, 0)].bg, Color::Rgb(20, 30, 40));
            assert_eq!(buf[(0, 0)].fg, Color::Rgb(20, 30, 40));
        }

        #[test]
        fn no_darken_bg_leaves_colors_untouched() {
            let area = Rect::new(0, 0, 100, 30);
            let mut buf = Buffer::empty(area);
            buf[(0, 0)].set_bg(Color::Rgb(100, 150, 200));

            let mut state = SearchPickerState::new(vec!["a", "b"]);
            SearchPicker::default()
                .no_darken_bg()
                .render(area, &mut buf, &mut state);

            assert_eq!(buf[(0, 0)].bg, Color::Rgb(100, 150, 200));
        }

        #[test]
        fn input_and_list_are_separated_by_a_joined_divider() {
            let area = Rect::new(0, 0, 100, 30);
            let mut buf = Buffer::empty(area);
            let mut state = SearchPickerState::new(vec!["a", "b"]);
            SearchPicker::default().render(area, &mut buf, &mut state);

            let popup = area.palette(6);
            let divider_y = popup.y + 2;
            assert_eq!(buf[(popup.x, divider_y)].symbol(), "├");
            assert_eq!(buf[(popup.right() - 1, divider_y)].symbol(), "┤");
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
            let cells = cells("héllo", &[1]);

            assert_eq!(cells[1].0, "é");
            assert_eq!(cells[1].1, hit());
        }

        #[test]
        fn out_of_range_match_is_ignored() {
            assert!(cells("ab", &[5]).into_iter().all(|(_, fg)| fg == base()));
        }
    }
}
