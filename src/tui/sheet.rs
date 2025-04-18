use ratatui::{
    layout::Alignment,
    style::{Modifier, Stylize},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, Paragraph, StatefulWidget, Widget, Wrap},
};

use crate::{
    misc::globals::theme,
    tui::utils::{Scroll, line_count},
};

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

#[derive(Debug)]
pub struct Sheet {
    sections: Vec<SheetSection>,
    block: Option<Block<'static>>,
}

impl Sheet {
    pub fn new() -> Self {
        Self {
            sections: Default::default(),
            block: None,
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

impl Default for Sheet {
    fn default() -> Self {
        Self::new()
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

impl StatefulWidget for Sheet {
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
                theme().header(idx),
            )));
            for line in content.lines() {
                lines.push(Line::from(Span::styled(line, theme().text())));
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
            .style(theme().text())
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: true })
            .block(
                Block::new()
                    .style(theme().text())
                    .border_style(theme().block())
                    .border_type(BorderType::Rounded)
                    .title_bottom(Line::from_iter([
                        Span::raw(" Scroll Up ").style(theme().block_tag()),
                        Span::raw(" Shift+K | Shift+\u{2191} ")
                            .style(theme().block_tag())
                            .add_modifier(Modifier::REVERSED),
                        Span::raw(" "),
                        Span::raw(" Scroll Down ").style(theme().block_tag()),
                        Span::raw(" Shift+J | Shift+\u{2193} ")
                            .style(theme().block_tag())
                            .add_modifier(Modifier::REVERSED),
                    ]))
                    .title_alignment(Alignment::Center)
                    .borders(Borders::ALL),
            )
            .scroll((state.scroll.val_u16(), 0))
            .render(area, buf);
    }
}
