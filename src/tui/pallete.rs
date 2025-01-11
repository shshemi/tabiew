use ratatui::{
    layout::{Constraint, Flex, Layout, Rect},
    style::Modifier,
    symbols::{self},
    text::Text,
    widgets::{
        Block, Borders, Clear, List, ListItem, ListState, Padding, Paragraph, StatefulWidget,
        Widget,
    },
};
use std::{borrow::Cow, marker::PhantomData};
use tui_input::{Input, InputRequest};

use super::Styler;

pub enum PalleteValue<'a> {
    Item(usize),
    Text(&'a str),
}
#[derive(Debug, Default)]
pub struct PalleteState {
    input: Input,
    list: ListState,
    scroll: usize,
}

impl PalleteState {
    pub fn delete_prev(&mut self) {
        self.list.select(None);
        self.input.handle(InputRequest::DeletePrevChar);
    }

    pub fn delete_next(&mut self) {
        self.list.select(None);
        self.input.handle(InputRequest::DeleteNextChar);
    }

    pub fn goto_prev(&mut self) {
        self.list.select(None);
        self.input.handle(InputRequest::GoToPrevChar);
    }

    pub fn goto_next(&mut self) {
        self.list.select(None);
        self.input.handle(InputRequest::GoToNextChar);
    }

    pub fn goto_start(&mut self) {
        self.list.select(None);
        self.input.handle(InputRequest::GoToStart);
    }

    pub fn goto_end(&mut self) {
        self.list.select(None);
        self.input.handle(InputRequest::GoToEnd);
    }

    pub fn goto_above(&mut self) {
        self.list.select(
            self.list
                .selected()
                .map(|idx| idx.saturating_sub(1))
                .unwrap_or(0)
                .into(),
        );
    }

    pub fn goto_below(&mut self) {
        self.list.select(
            self.list
                .selected()
                .map(|idx| idx.saturating_add(1))
                .unwrap_or(0)
                .into(),
        );
    }

    pub fn insert(&mut self, c: char) {
        self.list.select(None);
        self.input.handle(InputRequest::InsertChar(c));
    }

    pub fn value(&self) -> PalleteValue {
        if let Some(idx) = self.list.selected() {
            PalleteValue::Item(idx)
        } else {
            PalleteValue::Text(self.input.value())
        }
    }

    pub fn text(&self) -> &str {
        self.input.value()
    }
}

pub struct NoItems;

#[derive(Debug)]
pub struct Pallete<Theme, Items = NoItems> {
    width: u16,
    height: u16,
    hor_pad: u16,
    scroll_pad: u16,
    items: Items,
    _theme: PhantomData<Theme>,
}

impl<T> Pallete<T, NoItems> {
    pub fn new() -> Self {
        Pallete::<T, NoItems> {
            width: 60,
            height: 15,
            hor_pad: 1,
            scroll_pad: 4,
            items: NoItems,
            _theme: Default::default(),
        }
    }
}

impl<T, I> Pallete<T, I> {
    pub fn with_width(mut self, width: u16) -> Self {
        self.width = width;
        self
    }
    pub fn with_height(mut self, height: u16) -> Self {
        self.height = height;
        self
    }
    pub fn with_horizontal_pad(mut self, pad: u16) -> Self {
        self.hor_pad = pad;
        self
    }

    pub fn with_scroll_pad(mut self, pad: u16) -> Self {
        self.scroll_pad = pad;
        self
    }
}

impl<T> Pallete<T, NoItems> {
    pub fn with_items<'a, Items, Item>(self, items: Items) -> Pallete<T, Items>
    where
        Items: IntoIterator<Item = Item>,
        Item: Into<ListItem<'a>>,
    {
        Pallete::<T, Items> {
            width: self.width,
            height: self.height,
            hor_pad: self.hor_pad,
            scroll_pad: self.scroll_pad,
            items,
            _theme: self._theme,
        }
    }
}

impl<'a, Theme: Styler, Items, Item> StatefulWidget for Pallete<Theme, Items>
where
    Items: IntoIterator<Item = Item>,
    Item: Into<Cow<'a, str>>,
{
    type State = PalleteState;

    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        state: &mut Self::State,
    ) {
        let pal_area = pallete_area(area, self.width, self.height);

        let [first, second] =
            Layout::vertical([Constraint::Length(3), Constraint::Fill(1)]).areas(pal_area);

        // clean pallete area
        Widget::render(&Clear, pal_area, buf);

        // calculate scroll to stay between locks
        state.scroll = {
            let input_len = state.input.value().chars().count();
            let input_width = self.width - 2 - self.hor_pad * 2 - 1;
            let cursor = state.input.visual_cursor();
            let min_scroll = if input_len - cursor < self.scroll_pad as usize {
                cursor.saturating_sub(input_width as usize)
            } else {
                cursor
                    .saturating_add(self.scroll_pad as usize)
                    .saturating_sub(input_width as usize)
            };
            let max_scroll = cursor.saturating_sub(self.scroll_pad as usize);
            state.scroll.clamp(min_scroll, max_scroll)
        };

        // draw text input
        Paragraph::new(
            state
                .input
                .value()
                .chars()
                .skip(state.scroll)
                .collect::<String>(),
        )
        .block(
            Block::bordered()
                .border_set(symbols::border::Set {
                    bottom_left: symbols::line::VERTICAL_RIGHT,
                    bottom_right: symbols::line::VERTICAL_LEFT,
                    ..symbols::border::ROUNDED
                })
                .padding(Padding::horizontal(self.hor_pad))
                .style(Theme::pallete()),
        )
        .style(Theme::pallete_text())
        .render(first, buf);

        // draw cursor while editing
        // if user is scrolling the list, no cursor is needed
        if state.list.selected().is_none() {
            buf.set_style(
                Rect {
                    x: pal_area.x
                        + 1
                        + self.hor_pad
                        + (state.input.visual_cursor() - state.scroll) as u16,
                    y: pal_area.y + 1,
                    width: 1,
                    height: 1,
                },
                Theme::pallete_text().add_modifier(Modifier::REVERSED),
            );
        }

        // draw list
        StatefulWidget::render(
            List::new(pallete_items::<Theme>(self.items))
                .style(Theme::pallete_text())
                .highlight_style(Theme::pallete_hightlight())
                .block(
                    Block::new()
                        .borders(Borders::LEFT | Borders::BOTTOM | Borders::RIGHT)
                        .border_set(symbols::border::ROUNDED)
                        .padding(Padding::horizontal(self.hor_pad))
                        .style(Theme::pallete()),
                ),
            second,
            buf,
            &mut state.list,
        );
    }
}

fn pallete_items<'a, Theme: Styler>(
    iter: impl IntoIterator<Item = impl Into<Cow<'a, str>>>,
) -> impl Iterator<Item = ListItem<'a>> {
    iter.into_iter().map(|txt| ListItem::new(Text::raw(txt)))
}

fn pallete_area(area: Rect, width: u16, height: u16) -> Rect {
    let [_, lo] = Layout::vertical([Constraint::Length(2), Constraint::Length(height)])
        .flex(Flex::Start)
        .areas(area);
    let [lo] = Layout::horizontal([Constraint::Max(width)])
        .flex(Flex::Center)
        .areas(lo);
    lo
}
