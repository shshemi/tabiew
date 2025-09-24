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
    misc::{globals::theme, type_ext::HasSubsequence},
    tui::{
        status_bar::{StatusBar, Tag},
        themes::styler::Styler,
        widgets::{
            block::Block,
            input::{Input, InputState},
        },
    },
};

#[derive(Debug)]
pub struct SearchPickerState {
    input: InputState,
    list: ListState,
    cached_filter: CachedFilter,
}

impl SearchPickerState {
    pub fn input(&self) -> &InputState {
        &self.input
    }

    pub fn input_mut(&mut self) -> &mut InputState {
        &mut self.input
    }

    pub fn list(&self) -> &ListState {
        &self.list
    }

    pub fn list_mut(&mut self) -> &mut ListState {
        &mut self.list
    }

    pub fn set_input(&mut self, text: String) {
        let mut input = InputState::default();
        for c in text.chars() {
            input.insert(c);
        }
        self.input = input;
    }

    pub fn selected(&self) -> Option<usize> {
        self.cached_filter
            .indices
            .get(self.list.selected().unwrap_or_default())
            .copied()
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
        mut self,
        _: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        state: &mut Self::State,
    ) {
        let [area] = Layout::horizontal([Constraint::Length(80)])
            .flex(Flex::Center)
            .areas(buf.area);
        let [_, area] =
            Layout::vertical([Constraint::Length(3), Constraint::Length(15)]).areas(area);

        Clear.render(area, buf);
        let [input_area, list_area] =
            Layout::vertical([Constraint::Length(3), Constraint::Fill(1)]).areas(area);

        self.input
            .block(self.txt_blk)
            .render(input_area, buf, &mut state.input);
        let list = List::new(
            state
                .cached_filter
                .query(state.input.value(), &self.items)
                .iter()
                .copied()
                .map(|idx| std::mem::take(&mut self.items[idx]))
                .map(|item| ListItem::new(item).style(theme().text())),
        )
        .highlight_style(theme().highlight())
        .block(
            Block::default()
                .borders(Borders::LEFT | Borders::BOTTOM | Borders::RIGHT)
                .bottom(if state.list.selected().is_some() {
                    StatusBar::new()
                        .mono_color()
                        .centered()
                        .tag(Tag::new(" Insert ", " Enter "))
                        .tag(Tag::new(" Cancel ", " Esc "))
                } else {
                    StatusBar::new()
                })
                .into_widget(),
        );
        if state.list.selected().is_none() && !list.is_empty() {
            state.list.select(Some(0));
        }
        StatefulWidget::render(list, list_area, buf, &mut state.list);
    }
}

#[derive(Debug)]
struct CachedFilter {
    indices: Vec<usize>,
    metric: Metric,
    query_hash: u64,
}

impl Default for CachedFilter {
    fn default() -> Self {
        Self::new(Metric::HasSubsequence)
    }
}

impl CachedFilter {
    pub fn new(metric: Metric) -> Self {
        Self {
            indices: Vec::new(),
            metric,
            query_hash: 0,
        }
    }

    pub fn query<T>(&mut self, query: &str, items: &[T]) -> &[usize]
    where
        T: AsRef<str>,
    {
        let mut hasher = DefaultHasher::new();
        query.hash(&mut hasher);
        let query_hash = hasher.finish();

        if self.query_hash != query_hash {
            self.indices.clear();
            self.indices
                .extend(items.iter().enumerate().filter_map(|(idx, item)| {
                    self.metric.validate(item.as_ref(), query).then_some(idx)
                }));
            self.query_hash = query_hash;
        }
        &self.indices
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Metric {
    Contains,
    HasSubsequence,
}

impl Metric {
    fn validate(&self, a: &str, b: &str) -> bool {
        match self {
            Metric::Contains => a.contains(b),
            Metric::HasSubsequence => a.has_subsequence(b),
        }
    }
}
