use ratatui::{
    layout::Rect,
    widgets::{Block, BorderType, Clear, List, ListState, StatefulWidget, Widget},
};

use crate::misc::globals::theme;

#[derive(Debug)]
pub struct SidePanelState {
    list: ListState,
}

impl SidePanelState {
    pub fn new(selected: usize) -> SidePanelState {
        Self {
            list: ListState::default().with_selected(Some(selected)),
        }
    }
}

impl SidePanelState {
    pub fn list(&self) -> &ListState {
        &self.list
    }

    pub fn list_mut(&mut self) -> &mut ListState {
        &mut self.list
    }
}

#[derive(Debug)]
pub struct SidePanel<'a, 'b> {
    items: &'a [String],
    title: Option<&'b str>,
}

impl<'a, 'b> SidePanel<'a, 'b> {
    pub fn new(items: &'a [String]) -> Self {
        Self { items, title: None }
    }

    pub fn title(mut self, title: impl Into<Option<&'b str>>) -> Self {
        self.title = title.into();
        self
    }
}

impl StatefulWidget for SidePanel<'_, '_> {
    type State = SidePanelState;

    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        state: &mut Self::State,
    ) {
        let width = self
            .items
            .iter()
            .map(|s| s.len() as u16)
            .max()
            .map(|max| max + 2)
            .map(|w| w.clamp(34, area.width.saturating_div(2)))
            .unwrap_or(34);
        let area = Rect::new(area.x + area.width - width, area.y, width, area.height);

        Widget::render(Clear, area, buf);

        let mut block = Block::bordered()
            .border_type(BorderType::Rounded)
            .border_style(theme().block());
        if let Some(title) = self.title {
            block = block.title(title);
        }
        StatefulWidget::render(
            List::default()
                .items(self.items.iter().cloned())
                .style(theme().text())
                .highlight_style(theme().highlight())
                .block(block),
            area,
            buf,
            &mut state.list,
        );
    }
}
