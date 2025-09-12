use ratatui::{
    layout::{Constraint, Flex, Layout},
    text::Line,
    widgets::{Clear, List, ListItem, ListState, StatefulWidget, Widget, block::Title},
};

use crate::{misc::globals::theme, tui::widgets::block::Block};

#[derive(Debug)]
pub struct ListPickerState {
    list: ListState,
}

impl ListPickerState {
    pub fn list(&self) -> &ListState {
        &self.list
    }

    pub fn list_mut(&mut self) -> &mut ListState {
        &mut self.list
    }
}

impl Default for ListPickerState {
    fn default() -> Self {
        Self {
            list: ListState::default().with_selected(0.into()),
        }
    }
}

#[derive(Debug, Default)]
pub struct ListPicker<'a> {
    items: Vec<ListItem<'a>>,
    block: Block<'a>,
    width: u16,
}

impl<'a> ListPicker<'a> {
    pub fn title<T: Into<Title<'a>>>(mut self, title: T) -> Self {
        self.block = self.block.title(title);
        self
    }

    pub fn bottom<T: Into<Line<'a>>>(mut self, title: T) -> Self {
        self.block = self.block.bottom(title);
        self
    }

    pub fn items<T>(mut self, items: T) -> Self
    where
        T: IntoIterator,
        T::Item: Into<ListItem<'a>>,
    {
        self.items = items
            .into_iter()
            .map(Into::into)
            .inspect(|d| self.width = self.width.max(d.width() as u16))
            .collect();
        self
    }
}

impl<'a> StatefulWidget for ListPicker<'a> {
    type State = ListPickerState;

    fn render(
        self,
        _: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        state: &mut Self::State,
    ) {
        let width = (self.width + 2).clamp(80, 80);

        let [area] = Layout::horizontal([Constraint::Length(width)])
            .flex(Flex::Center)
            .areas(buf.area);
        let [_, area] =
            Layout::vertical([Constraint::Length(3), Constraint::Length(15)]).areas(area);
        Clear.render(area, buf);

        StatefulWidget::render(
            List::default()
                .style(theme().text())
                .highlight_style(theme().highlight())
                .items(self.items)
                .block(self.block.into_widget()),
            area,
            buf,
            &mut state.list,
        );
    }
}
