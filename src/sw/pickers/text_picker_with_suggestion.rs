use std::{borrow::Cow, marker::PhantomData};

use itertools::Itertools;
use ratatui::{
    layout::Constraint,
    widgets::{Block, List, ListItem, ListState, StatefulWidget},
};

use crate::sw::{
    app_default::AppDefault,
    buffer_ext::BufferExt,
    rect_ext::RectExt,
    widgets::{
        input::{Input, InputState},
        split::Split,
    },
};

pub trait Suggestion {
    fn title(&self) -> &str;
    fn apply_to(&self, input: &mut InputState);
}

pub trait Provider {
    type Suggestion: Suggestion;
    fn suggestions(&self, query: &str, cursor: usize) -> Vec<Self::Suggestion>;
}

impl<S> Provider for fn(&str, usize) -> Vec<S>
where
    S: Suggestion,
{
    type Suggestion = S;

    fn suggestions(&self, query: &str, cursor: usize) -> Vec<Self::Suggestion> {
        self(query, cursor)
    }
}

#[derive(Debug)]
pub struct TextPickerWithSuggestionState<P: Provider> {
    input: InputState,
    list: ListState,
    args: (String, usize),
    items: Vec<P::Suggestion>,
    provider: P,
}

impl<P> TextPickerWithSuggestionState<P>
where
    P: Provider,
{
    pub fn new(provider: P) -> Self {
        Self {
            input: InputState::default(),
            list: ListState::default().with_selected(0.into()),
            args: (String::default(), 0),
            items: provider.suggestions("", 0),
            provider,
        }
    }

    pub fn with_value(self, value: impl Into<String>) -> Self {
        Self {
            input: self.input.with_value(value.into()),
            ..self
        }
    }

    pub fn input(&self) -> &InputState {
        &self.input
    }

    pub fn input_mut(&mut self) -> &mut InputState {
        &mut self.input
    }

    pub fn value(&self) -> &str {
        self.input.value()
    }

    pub fn has_suggestions(&self) -> bool {
        !self.items.is_empty()
    }

    pub fn suggestions(&self) -> &[P::Suggestion] {
        &self.items
    }

    pub fn selected_suggestion(&self) -> Option<&P::Suggestion> {
        self.list.selected().and_then(|idx| self.items.get(idx))
    }

    pub fn clear_suggestion_selection(&mut self) {
        self.list = ListState::default().with_selected(0.into());
    }

    pub fn apply_selected_suggestion(&mut self) {
        if let Some(suggestion) = self.list.selected().and_then(|idx| self.items.get(idx)) {
            suggestion.apply_to(&mut self.input);
        }
    }

    pub fn refresh_suggestions(&mut self) {
        if self.args.0 != self.input.value() || self.args.1 != self.input.cursor() {
            self.args = (self.input.value().to_owned(), self.input.cursor());
            self.items = self.provider.suggestions(&self.args.0, self.args.1);
        }
    }

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

    pub fn select_first(&mut self) {
        self.list.select_first();
    }

    pub fn select_last(&mut self) {
        self.select(Some(self.items.len().saturating_sub(1)));
    }

    pub fn cycle_up(&mut self) {
        if self.list.selected() != Some(0) {
            self.select_up();
        } else {
            self.select_last();
        }
    }

    pub fn cycle_down(&mut self) {
        if self.list.selected() != Some(self.items.len().saturating_sub(1)) {
            self.select_down();
        } else {
            self.select_first();
        }
    }
}

#[derive(Debug)]
pub struct TextPickerWithSuggestion<'a, P> {
    title: Cow<'a, str>,
    darken_bg: bool,
    marker: PhantomData<P>,
}

impl<'a, P> TextPickerWithSuggestion<'a, P> {
    pub fn title(mut self, title: impl Into<Cow<'a, str>>) -> Self {
        self.title = title.into();
        self
    }

    pub fn no_darken_bg(mut self) -> Self {
        self.darken_bg = false;
        self
    }
}

impl<'a, P> Default for TextPickerWithSuggestion<'a, P> {
    fn default() -> Self {
        Self {
            title: Cow::Borrowed(""),
            darken_bg: true,
            marker: PhantomData,
        }
    }
}

impl<'a, P> StatefulWidget for TextPickerWithSuggestion<'a, P>
where
    P: Provider,
{
    type State = TextPickerWithSuggestionState<P>;

    fn render(
        self,
        _area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        state: &mut Self::State,
    ) {
        if self.darken_bg {
            buf.darken();
        }

        state.refresh_suggestions();

        let items = state
            .items
            .iter()
            .map(|suggestion| ListItem::new(suggestion.title()))
            .collect_vec();

        let area = buf
            .area
            .palette(items.len().saturating_add(4).min(25) as u16);
        buf.clear(area);

        let [input_area, list_area] = Split::vertical([Constraint::Length(1), Constraint::Fill(1)])
            .block(Block::app_default().title(self.title))
            .split(buf, area);

        Input::default().render(input_area, buf, &mut state.input);

        List::app_default()
            .items(items)
            .render(list_area, buf, &mut state.list);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::{buffer::Buffer, layout::Rect, style::Color};

    #[derive(Debug, PartialEq)]
    struct Word(String);

    impl Suggestion for Word {
        fn title(&self) -> &str {
            &self.0
        }

        fn apply_to(&self, input: &mut InputState) {
            input.set_value(self.0.clone());
        }
    }

    #[derive(Debug)]
    struct Words(Vec<&'static str>);

    impl Provider for Words {
        type Suggestion = Word;

        fn suggestions(&self, query: &str, _cursor: usize) -> Vec<Word> {
            self.0
                .iter()
                .filter(|word| word.starts_with(query))
                .map(|word| Word((*word).to_owned()))
                .collect()
        }
    }

    fn words() -> Words {
        Words(vec!["alpha", "alps", "beta"])
    }

    fn state() -> TextPickerWithSuggestionState<Words> {
        TextPickerWithSuggestionState::new(words())
    }

    fn typed(state: &mut TextPickerWithSuggestionState<Words>, text: &str) {
        for c in text.chars() {
            state.input_mut().insert(c);
        }
    }

    fn render_once(state: &mut TextPickerWithSuggestionState<Words>) -> Buffer {
        let area = Rect::new(0, 0, 100, 30);
        let mut buf = Buffer::empty(area);
        TextPickerWithSuggestion::default().render(area, &mut buf, state);
        buf
    }

    fn content(buf: &Buffer) -> String {
        buf.content().iter().map(|c| c.symbol()).collect()
    }

    mod state {
        use super::*;

        #[test]
        fn new_seeds_suggestions_from_the_empty_query() {
            let state = state();
            assert!(state.has_suggestions());
            assert_eq!(state.suggestions().len(), 3);
        }

        #[test]
        fn with_value_sets_the_input() {
            let state = state().with_value("alp");
            assert_eq!(state.value(), "alp");
        }

        #[test]
        fn refresh_narrows_suggestions_to_the_typed_prefix() {
            let mut state = state();
            typed(&mut state, "alp");
            state.refresh_suggestions();

            assert_eq!(
                state.suggestions().iter().map(Suggestion::title).collect_vec(),
                vec!["alpha", "alps"]
            );
        }

        #[test]
        fn refresh_is_a_no_op_when_input_is_unchanged() {
            let mut state = state();
            typed(&mut state, "beta");
            state.refresh_suggestions();
            state.refresh_suggestions();

            assert_eq!(state.suggestions().len(), 1);
        }

        #[test]
        fn refresh_reacts_to_cursor_movement_alone() {
            let mut state = state();
            typed(&mut state, "alp");
            state.refresh_suggestions();
            assert_eq!(state.suggestions().len(), 2);

            state.input_mut().goto_start();
            state.refresh_suggestions();

            assert_eq!(state.suggestions().len(), 2);
        }

        #[test]
        fn no_suggestions_when_nothing_matches() {
            let mut state = state();
            typed(&mut state, "zzz");
            state.refresh_suggestions();

            assert!(!state.has_suggestions());
            assert_eq!(state.selected_suggestion(), None);
        }

        #[test]
        fn selected_suggestion_follows_selection() {
            let mut state = state();
            state.select(Some(2));
            assert_eq!(state.selected_suggestion().map(Suggestion::title), Some("beta"));
        }

        #[test]
        fn apply_selected_suggestion_writes_into_the_input() {
            let mut state = state();
            state.select(Some(1));
            state.apply_selected_suggestion();

            assert_eq!(state.value(), "alps");
        }

        #[test]
        fn apply_selected_suggestion_is_a_no_op_without_a_selection() {
            let mut state = state();
            state.select(None);
            state.apply_selected_suggestion();

            assert_eq!(state.value(), "");
        }

        #[test]
        fn clear_suggestion_selection_resets_to_first() {
            let mut state = state();
            state.select(Some(2));
            state.clear_suggestion_selection();

            assert_eq!(state.selected(), Some(0));
        }

        #[test]
        fn cycle_up_wraps_to_last_suggestion() {
            let mut state = state();
            state.select(Some(0));
            state.cycle_up();

            assert_eq!(state.selected(), Some(2));
        }

        #[test]
        fn cycle_down_wraps_to_first_suggestion() {
            let mut state = state();
            state.select(Some(2));
            state.cycle_down();

            assert_eq!(state.selected(), Some(0));
        }

        #[test]
        fn select_first_and_last() {
            let mut state = state();
            state.select_last();
            assert_eq!(state.selected(), Some(2));
            state.select_first();
            assert_eq!(state.selected(), Some(0));
        }
    }

    mod widget {
        use super::*;

        #[test]
        fn renders_title_and_suggestions() {
            let area = Rect::new(0, 0, 100, 30);
            let mut buf = Buffer::empty(area);
            let mut state = state();
            TextPickerWithSuggestion::default()
                .title("Path")
                .render(area, &mut buf, &mut state);

            let content = content(&buf);
            assert!(content.contains("Path"));
            assert!(content.contains("alpha"));
            assert!(content.contains("beta"));
        }

        #[test]
        fn renders_typed_value_in_the_input_row() {
            let mut state = state();
            typed(&mut state, "alp");
            let buf = render_once(&mut state);

            assert!(content(&buf).contains("alp"));
        }

        #[test]
        fn render_refreshes_suggestions_from_the_input() {
            let mut state = state();
            typed(&mut state, "alp");
            let buf = render_once(&mut state);

            let content = content(&buf);
            assert!(content.contains("alps"));
            assert!(!content.contains("beta"));
        }

        #[test]
        fn input_and_suggestions_are_separated_by_a_joined_divider() {
            let area = Rect::new(0, 0, 100, 30);
            let mut buf = Buffer::empty(area);
            let mut state = state();
            TextPickerWithSuggestion::default().render(area, &mut buf, &mut state);

            let popup = area.palette(7);
            let divider_y = popup.y + 2;
            assert_eq!(buf[(popup.x, divider_y)].symbol(), "├");
            assert_eq!(buf[(popup.right() - 1, divider_y)].symbol(), "┤");
        }

        #[test]
        fn darken_bg_scales_colors_outside_popup() {
            let area = Rect::new(0, 0, 100, 30);
            let mut buf = Buffer::empty(area);
            buf[(0, 0)].set_bg(Color::Rgb(100, 150, 200));
            buf[(0, 0)].set_fg(Color::Rgb(100, 150, 200));

            let mut state = state();
            TextPickerWithSuggestion::default().render(area, &mut buf, &mut state);

            assert_eq!(buf[(0, 0)].bg, Color::Rgb(20, 30, 40));
            assert_eq!(buf[(0, 0)].fg, Color::Rgb(20, 30, 40));
        }

        #[test]
        fn no_darken_bg_leaves_colors_untouched() {
            let area = Rect::new(0, 0, 100, 30);
            let mut buf = Buffer::empty(area);
            buf[(0, 0)].set_bg(Color::Rgb(100, 150, 200));

            let mut state = state();
            TextPickerWithSuggestion::default()
                .no_darken_bg()
                .render(area, &mut buf, &mut state);

            assert_eq!(buf[(0, 0)].bg, Color::Rgb(100, 150, 200));
        }
    }
}
