use std::{
    borrow::Cow,
    fmt::Debug,
    hash::{DefaultHasher, Hash, Hasher},
};

use ratatui::{
    layout::{Constraint, Flex, Layout},
    symbols::{
        border::{ROUNDED, Set},
        line::{VERTICAL_LEFT, VERTICAL_RIGHT},
    },
    widgets::{Borders, Clear, List, ListItem, ListState, StatefulWidget, Widget, block::Title},
};

use crate::{
    misc::globals::theme,
    tui::widgets::{
        block::Block,
        highlighted_line::HighlightedLine,
        input::{Input, Input},
    },
};

#[derive(Debug)]
pub struct SearchPickerState {
    input: Input,
    list: ListState,
    cached_filter: CachedFilter,
}

impl SearchPickerState {
    pub fn input(&self) -> &Input {
        &self.input
    }

    pub fn input_mut(&mut self) -> &mut Input {
        &mut self.input
    }

    pub fn list(&self) -> &ListState {
        &self.list
    }

    pub fn list_mut(&mut self) -> &mut ListState {
        &mut self.list
    }

    pub fn set_input(&mut self, text: String) {
        let mut input = Input::default();
        for c in text.chars() {
            input.insert(c);
        }
        self.input = input;
    }

    pub fn selected(&self) -> Option<usize> {
        self.cached_filter
            .indices
            .get(self.list.selected().unwrap_or_default())
            .map(|(i, _)| *i)
    }
}

impl Default for SearchPickerState {
    fn default() -> Self {
        Self {
            input: Default::default(),
            list: ListState::default().with_selected(Some(0)),
            cached_filter: Default::default(),
        }
    }
}

#[derive(Debug)]
pub struct SearchPicker<'a> {
    txt_blk: Block<'a>,
    items: Vec<Cow<'a, str>>,
    input: Input<'a>,
}

impl<'a> SearchPicker<'a> {
    pub fn title(mut self, title: impl Into<Title<'a>>) -> Self {
        self.txt_blk = self.txt_blk.title(title);
        self
    }

    pub fn items<T>(mut self, items: T) -> Self
    where
        T: IntoIterator,
        T::Item: Into<Cow<'a, str>>,
    {
        self.items = items.into_iter().map(Into::into).collect();
        self
    }
}

impl<'a> Default for SearchPicker<'a> {
    fn default() -> Self {
        Self {
            txt_blk: Block::default().border_set(Set {
                bottom_left: VERTICAL_RIGHT,
                bottom_right: VERTICAL_LEFT,
                ..ROUNDED
            }),
            items: Default::default(),
            input: Input::default(),
        }
    }
}

impl<'a> StatefulWidget for SearchPicker<'a> {
    type State = SearchPickerState;

    fn render(
        self,
        _: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        state: &mut Self::State,
    ) {
        let list = List::new(
            state
                .cached_filter
                .query(state.input.value(), &self.items)
                .iter()
                .map(|(idx, hl)| (&self.items[*idx], hl))
                .map(|(item, hl)| {
                    ListItem::new(
                        HighlightedLine::default()
                            .text(item.as_ref())
                            .highlights(hl.iter().copied())
                            .text_style(theme().text())
                            .highlight_style(theme().text_highlighted()),
                    )
                    .style(theme().text())
                }),
        )
        .highlight_style(theme().row_highlighted())
        .block(
            Block::default()
                .borders(Borders::LEFT | Borders::BOTTOM | Borders::RIGHT)
                .into_widget(),
        );

        let width = 80;
        let height = list.len().saturating_add(4).min(25) as u16;

        let [area] = Layout::horizontal([Constraint::Length(width)])
            .flex(Flex::Center)
            .areas(buf.area);
        let [_, area] =
            Layout::vertical([Constraint::Length(3), Constraint::Length(height)]).areas(area);

        Clear.render(area, buf);
        let [input_area, list_area] =
            Layout::vertical([Constraint::Length(3), Constraint::Fill(1)]).areas(area);

        self.input
            .block(self.txt_blk)
            .render(input_area, buf, &mut state.input);

        *state.list.offset_mut() = state
            .list()
            .offset()
            .min(list.len().saturating_sub(list_area.height as usize));
        if state.list.selected().is_none() && !list.is_empty() {
            state.list.select(Some(0));
        }
        StatefulWidget::render(list, list_area, buf, &mut state.list);
    }
}

#[derive(Debug, Default)]
struct CachedFilter {
    indices: Vec<(usize, Vec<usize>)>,
    query_hash: u64,
}

impl CachedFilter {
    pub fn query<T>(&mut self, query: &str, items: &[T]) -> &[(usize, Vec<usize>)]
    where
        T: AsRef<str>,
    {
        let mut hasher = DefaultHasher::new();
        query.hash(&mut hasher);
        let query_hash = hasher.finish();

        if self.query_hash != query_hash {
            self.indices.clear();
            self.indices.extend(
                items.iter().enumerate().filter_map(|(idx, item)| {
                    subsequence_pos(item.as_ref(), query).map(|v| (idx, v))
                }),
            );
            self.query_hash = query_hash;
        }
        &self.indices
    }
}

fn subsequence_pos(larger: &str, other: &str) -> Option<Vec<usize>> {
    if other.is_empty() {
        return Some(Vec::new());
    }

    let mut want_iter = other.chars();
    let mut want = want_iter.next().unwrap();
    let mut idxs = Vec::with_capacity(other.chars().count());

    for (char_pos, c) in larger.chars().enumerate() {
        if c.eq_ignore_ascii_case(&want) {
            idxs.push(char_pos);
            if let Some(nxt) = want_iter.next() {
                want = nxt;
            } else {
                return Some(idxs);
            }
        }
    }
    None
}
