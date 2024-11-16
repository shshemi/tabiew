use std::marker::PhantomData;

use ratatui::{
    layout::Alignment,
    text::{Line, Span},
    widgets::{Borders, Paragraph, StatefulWidget, Widget, Wrap},
};

use crate::tui::{
    utils::{line_count, Scroll},
    Styler,
};

#[derive(Debug)]
pub struct Sheet<Theme> {
    title: String,
    blocks: Vec<SheetBlock>,
    _theme: PhantomData<Theme>,
}

impl<Theme> Sheet<Theme> {
    pub fn new(title: String, blocks: Vec<SheetBlock>) -> Self {
        Self {
            title,
            blocks,
            _theme: Default::default(),
        }
    }
}

#[derive(Debug)]
pub struct SheetBlock {
    header: String,
    content: String,
}

impl SheetBlock {
    pub fn new(header: String, content: String) -> Self {
        Self { header, content }
    }
}

#[derive(Debug, Default)]
pub struct SheetState {
    scroll: Scroll,
}

impl SheetState {
    pub fn scroll_up(&mut self) {
        self.scroll.up();
    }

    pub fn scroll_down(&mut self) {
        self.scroll.down();
    }
}

impl<Theme> StatefulWidget for Sheet<Theme>
where
    Theme: Styler,
{
    type State = SheetState;

    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        state: &mut Self::State,
    ) {
        let mut lines = Vec::new();
        for (idx, SheetBlock { header, content }) in self.blocks.iter().enumerate() {
            lines.push(Line::from(Span::styled(
                header.clone(),
                Theme::table_header_cell(idx),
            )));
            for line in content.lines() {
                lines.push(Line::from(Span::styled(line, Theme::sheet_value())));
            }
        }

        state.scroll.adjust(
            lines
                .iter()
                .map(|line| line_count(&line.to_string(), area.width as usize))
                .sum(),
            area.height.saturating_sub(2),
        );

        let parag = Paragraph::new(lines)
            .block(
                ratatui::widgets::Block::new()
                    .title(self.title)
                    .borders(Borders::ALL),
            )
            .style(Theme::sheet_block())
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: true });
        parag.scroll((state.scroll.val_u16(), 0)).render(area, buf);
    }
}
