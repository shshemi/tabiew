use crossterm::event::{KeyCode, KeyModifiers};
use ratatui::{
    layout::Alignment,
    text::Line,
    widgets::{Clear, Paragraph, StatefulWidget, Widget, Wrap},
};

use crate::{
    misc::globals::theme,
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
    sections: Vec<SheetSection>,
}

impl Sheet {
    pub fn new(sections: Vec<SheetSection>) -> Self {
        Self {
            scroll: Default::default(),
            sections,
        }
    }

    pub fn scroll_up(&mut self) {
        self.scroll.up();
    }

    pub fn scroll_down(&mut self) {
        self.scroll.down();
    }

    pub fn set_sections(&mut self, sections: Vec<SheetSection>) {
        self.sections = sections;
    }
}

impl Component for Sheet {
    fn render(
        &mut self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        focus_state: super::component::FocusState,
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
        match event.code {
            KeyCode::Char('K') => {
                self.scroll.up();
                true
            }
            KeyCode::Char('J') => {
                self.scroll.down();
                true
            }
            KeyCode::Up if event.modifiers == KeyModifiers::SHIFT => {
                self.scroll.up();
                true
            }
            KeyCode::Down if event.modifiers == KeyModifiers::SHIFT => {
                self.scroll.down();
                true
            }
            _ => false,
        }
    }
}

// impl StatefulWidget for Sheet {
//     type State = SheetState;

//     fn render(
//         self,
//         area: ratatui::prelude::Rect,
//         buf: &mut ratatui::prelude::Buffer,
//         state: &mut Self::State,
//     ) {
//         Clear.render(area, buf);

//         let pg = Paragraph::new(
//             self.sections
//                 .iter()
//                 .enumerate()
//                 .flat_map(|(idx, SheetSection { header, content })| {
//                     std::iter::once(Line::raw(header).style(theme().header(idx)))
//                         .chain(
//                             content
//                                 .lines()
//                                 .map(|line| Line::raw(line).style(theme().text())),
//                         )
//                         .chain(std::iter::once(Line::raw("\n")))
//                 })
//                 .collect::<Vec<_>>(),
//         )
//         .style(theme().text())
//         .alignment(Alignment::Left)
//         .wrap(Wrap { trim: true })
//         .block(
//             Block::default()
//                 .bottom(
//                     StatusBar::new()
//                         .mono_color()
//                         .centered()
//                         .tag(Tag::new(" Scroll Up ", " Shift+K | Shift+\u{2191} "))
//                         .tag(Tag::new(" Scroll Down ", " Shift+J | Shift+\u{2193} ")),
//                 )
//                 .title_alignment(Alignment::Center)
//                 .into_widget(),
//         );

//         state
//             .scroll
//             .adjust(pg.line_count(area.width), area.height.saturating_sub(2));

//         pg.scroll((state.scroll.val_u16(), 0)).render(area, buf);
//     }
// }
