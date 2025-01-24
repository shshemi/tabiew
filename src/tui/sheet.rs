use std::marker::PhantomData;

use ratatui::{
    layout::Alignment,
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, Paragraph, StatefulWidget, Widget, Wrap},
};

use crate::tui::{
    utils::{line_count, Scroll},
    Styler,
};

#[derive(Debug)]
pub struct Sheet<Theme> {
    sections: Vec<SheetSection>,
    block: Option<Block<'static>>,
    _theme: PhantomData<Theme>,
}

impl<Theme> Sheet<Theme> {
    pub fn new() -> Self {
        Self {
            sections: Default::default(),
            block: None,
            _theme: Default::default(),
        }
    }

    pub fn with_sections(mut self, sections: Vec<SheetSection>) -> Self {
        self.sections = sections;
        self
    }

    pub fn with_block(mut self, block: Block<'static>) -> Self {
        self.block = Some(block);
        self
    }
}

#[derive(Debug)]
pub struct SheetSection {
    header: String,
    content: String,
}

impl SheetSection {
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
        for (idx, SheetSection { header, content }) in self.sections.iter().enumerate() {
            lines.push(Line::from(Span::styled(
                header.clone(),
                Theme::table_header_cell(idx),
            )));
            for line in content.lines() {
                lines.push(Line::from(Span::styled(line, Theme::sheet_value())));
            }
            lines.push(Line::raw("\n"));
        }

        state.scroll.adjust(
            lines
                .iter()
                .map(|line| line_count(&line.to_string(), area.width as usize))
                .sum(),
            area.height.saturating_sub(2),
        );

        Clear.render(area, buf);

        Paragraph::new(lines)
            .style(Theme::sheet_block())
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: true })
            .block(
                Block::new()
                    .style(Theme::sheet_block())
                    .border_style(Theme::sheet_block())
                    .border_type(BorderType::Rounded)
                    .borders(Borders::ALL),
            )
            .scroll((state.scroll.val_u16(), 0))
            .render(area, buf);
    }
}
