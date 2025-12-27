use crossterm::event::{KeyCode, KeyModifiers};
use ratatui::{
    layout::Alignment,
    text::Line,
    widgets::{Clear, Paragraph, Widget, Wrap},
};

use crate::{
    handler::message::Message,
    misc::config::theme,
    tui::{
        component::Component,
        status_bar::{StatusBar, Tag},
        utils::Scroll,
        widgets::block::Block,
    },
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
    scroll: Scroll,
    row: usize,
    sections: Vec<SheetSection>,
}

impl Sheet {
    pub fn new(row: usize, sections: Vec<SheetSection>) -> Self {
        Self {
            scroll: Default::default(),
            row,
            sections,
        }
    }

    pub fn scroll_up(&mut self) {
        self.scroll.up();
    }

    pub fn scroll_down(&mut self) {
        self.scroll.down();
    }

    pub fn row(&self) -> usize {
        self.row
    }

    pub fn set(&mut self, row: usize, sections: Vec<SheetSection>) {
        self.row = row;
        self.sections = sections;
    }
}

impl Component for Sheet {
    fn render(
        &mut self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        _focus_state: super::component::FocusState,
    ) {
        Clear.render(area, buf);

        let pg = Paragraph::new(
            self.sections
                .iter()
                .enumerate()
                .flat_map(|(idx, SheetSection { header, content })| {
                    std::iter::once(Line::raw(header).style(theme().header(idx)))
                        .chain(
                            content
                                .lines()
                                .map(|line| Line::raw(line).style(theme().text())),
                        )
                        .chain(std::iter::once(Line::raw("\n")))
                })
                .collect::<Vec<_>>(),
        )
        .style(theme().text())
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true })
        .block(
            Block::default()
                .bottom(
                    StatusBar::new()
                        .mono_color()
                        .centered()
                        .tag(Tag::new(" Scroll Up ", " Shift+K | Shift+\u{2191} "))
                        .tag(Tag::new(" Scroll Down ", " Shift+J | Shift+\u{2193} ")),
                )
                .title_alignment(Alignment::Center)
                .into_widget(),
        );

        self.scroll
            .adjust(pg.line_count(area.width), area.height.saturating_sub(2));

        pg.scroll((self.scroll.val_u16(), 0)).render(area, buf);
    }

    fn handle(&mut self, event: crossterm::event::KeyEvent) -> bool {
        match (event.code, event.modifiers) {
            (KeyCode::Char('K'), KeyModifiers::SHIFT) | (KeyCode::Up, KeyModifiers::SHIFT) => {
                self.scroll.up();
                true
            }
            (KeyCode::Char('J'), KeyModifiers::SHIFT) | (KeyCode::Down, KeyModifiers::SHIFT) => {
                self.scroll.down();
                true
            }
            (KeyCode::Esc, KeyModifiers::NONE) | (KeyCode::Char('q'), KeyModifiers::NONE) => {
                Message::PaneDismissModal.enqueue();
                true
            }

            _ => false,
        }
    }
}
