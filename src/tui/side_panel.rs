use ratatui::{
    layout::{Constraint, Rect},
    text::Span,
    widgets::{Block, BorderType, Cell, Clear, Row, StatefulWidget, Table, TableState, Widget},
};

use crate::misc::globals::theme;

#[derive(Debug)]
pub struct SidePanelState {
    list: TableState,
}

impl SidePanelState {
    pub fn new(selected: usize) -> SidePanelState {
        Self {
            list: TableState::default().with_selected(Some(selected)),
        }
    }
}

impl SidePanelState {
    pub fn list(&self) -> &TableState {
        &self.list
    }

    pub fn list_mut(&mut self) -> &mut TableState {
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
        let num_width = (self.items.len().ilog10() + 1) as u16;
        let text_width = self
            .items
            .iter()
            .map(|s| s.len() as u16)
            .max()
            .map(|w| w.clamp(34, area.width.saturating_div(2)))
            .unwrap_or(34);
        let width = num_width + text_width + 3;
        let area = Rect::new(area.x + area.width - width, area.y, width, area.height);

        Widget::render(Clear, area, buf);

        let mut block = Block::bordered()
            .border_type(BorderType::Rounded)
            .border_style(theme().block());
        if let Some(title) = self.title {
            block = block.title(title);
        }
        let rows = self.items.iter().enumerate().map(|(i, s)| {
            Row::new([
                Cell::new(
                    Span::raw(format!(" {:>width$}", i + 1, width = num_width as usize))
                        .style(theme().subtext()),
                ),
                Cell::new(Span::raw(s.as_str()).style(theme().text())),
            ])
        });
        StatefulWidget::render(
            Table::default()
                .rows(rows)
                // .items(self.items.iter().cloned())
                .style(theme().text())
                .row_highlight_style(theme().highlight())
                .widths([
                    Constraint::Length(num_width + 1),
                    Constraint::Length(text_width),
                ])
                .column_spacing(1)
                .block(block),
            area,
            buf,
            &mut state.list,
        );
    }
}
