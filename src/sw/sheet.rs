use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    text::Line,
    widgets::{Block, Paragraph, StatefulWidget, Widget, Wrap},
};

use crate::{
    misc::config::theme,
    sw::{
        app_default::AppDefault,
        buffer_ext::BufferExt,
        widgets::tag_line::{Tag, TagLine},
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

#[derive(Debug, Default)]
pub struct SheetState {
    scroll: usize,
}

impl SheetState {
    pub fn scroll(&self) -> usize {
        self.scroll
    }

    pub fn scroll_up(&mut self) {
        self.scroll = self.scroll.saturating_sub(1);
    }

    pub fn scroll_down(&mut self) {
        self.scroll = self.scroll.saturating_add(1);
    }
}

#[derive(Debug)]
pub struct Sheet<'a> {
    sections: &'a [SheetSection],
}

impl<'a> Sheet<'a> {
    pub fn new(sections: &'a [SheetSection]) -> Self {
        Self { sections }
    }

    pub fn text(&self) -> String {
        self.sections
            .iter()
            .map(|section| format!("{}\n{}", section.header, section.content))
            .collect::<Vec<_>>()
            .join("\n\n")
    }
}

impl StatefulWidget for Sheet<'_> {
    type State = SheetState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        buf.clear(area);

        let paragraph = paragraph(self.sections);
        let visible = area.height.saturating_sub(2) as usize;
        let max_scroll = paragraph.line_count(area.width).saturating_sub(visible);
        state.scroll = state.scroll.min(max_scroll);

        paragraph.scroll((state.scroll as u16, 0)).render(area, buf);
    }
}

fn paragraph(sections: &[SheetSection]) -> Paragraph<'_> {
    Paragraph::new(
        sections
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
        Block::app_default()
            .title_bottom(
                TagLine::mono_color()
                    .centered()
                    .tag(Tag::new(" Scroll Up ", " Shift+K | Shift+\u{2191} "))
                    .tag(Tag::new(" Scroll Down ", " Shift+J | Shift+\u{2193} "))
                    .tag(Tag::new(" Copy ", " C ")),
            )
            .title_alignment(Alignment::Center),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sections(count: usize) -> Vec<SheetSection> {
        (0..count)
            .map(|idx| SheetSection::new(format!("header{idx}"), format!("content{idx}")))
            .collect()
    }

    fn tall_sections(count: usize) -> Vec<SheetSection> {
        (0..count)
            .map(|idx| {
                SheetSection::new(
                    format!("header{idx}"),
                    (0..5)
                        .map(|l| format!("line{idx}x{l}"))
                        .collect::<Vec<_>>()
                        .join("\n"),
                )
            })
            .collect()
    }

    fn render(
        state: &mut SheetState,
        sections: &[SheetSection],
        width: u16,
        height: u16,
    ) -> Buffer {
        let area = Rect::new(0, 0, width, height);
        let mut buf = Buffer::empty(area);
        Sheet::new(sections).render(area, &mut buf, state);
        buf
    }

    fn content(buf: &Buffer) -> String {
        buf.content().iter().map(|c| c.symbol()).collect()
    }

    mod state {
        use super::*;

        #[test]
        fn a_fresh_state_starts_at_the_top() {
            assert_eq!(SheetState::default().scroll(), 0);
        }

        #[test]
        fn scroll_up_saturates_at_the_top() {
            let mut state = SheetState::default();
            state.scroll_up();

            assert_eq!(state.scroll(), 0);
        }

        #[test]
        fn scroll_down_advances_unbounded_until_a_render_clamps_it() {
            let mut state = SheetState::default();
            state.scroll_down();
            state.scroll_down();

            assert_eq!(state.scroll(), 2);
        }
    }

    mod widget {
        use super::*;

        #[test]
        fn text_joins_headers_and_contents() {
            let sections = sections(2);

            assert_eq!(
                Sheet::new(&sections).text(),
                "header0\ncontent0\n\nheader1\ncontent1"
            );
        }

        #[test]
        fn text_of_no_sections_is_empty() {
            assert_eq!(Sheet::new(&[]).text(), "");
        }

        #[test]
        fn renders_headers_and_contents() {
            let sections = sections(2);
            let buf = render(&mut SheetState::default(), &sections, 40, 20);

            let content = content(&buf);
            assert!(content.contains("header0"));
            assert!(content.contains("content0"));
            assert!(content.contains("header1"));
            assert!(content.contains("content1"));
        }

        #[test]
        fn renders_the_shortcut_hints() {
            let sections = sections(1);
            let buf = render(&mut SheetState::default(), &sections, 100, 20);

            let content = content(&buf);
            assert!(content.contains("Scroll Up"));
            assert!(content.contains("Scroll Down"));
            assert!(content.contains("Copy"));
        }

        #[test]
        fn a_narrow_area_crops_the_shortcut_hints() {
            let sections = sections(1);
            let buf = render(&mut SheetState::default(), &sections, 40, 20);

            assert!(!content(&buf).contains("Scroll Up"));
        }

        #[test]
        fn is_wrapped_in_a_border() {
            let sections = sections(1);
            let buf = render(&mut SheetState::default(), &sections, 40, 20);

            assert!(content(&buf).contains('╭'));
        }

        #[test]
        fn content_that_fits_is_never_scrolled() {
            let sections = sections(1);
            let mut state = SheetState::default();
            for _ in 0..10 {
                state.scroll_down();
            }
            render(&mut state, &sections, 40, 20);

            assert_eq!(state.scroll(), 0);
        }

        #[test]
        fn scrolling_stops_at_the_last_page() {
            let sections = tall_sections(6);
            let mut state = SheetState::default();
            for _ in 0..100 {
                state.scroll_down();
            }
            render(&mut state, &sections, 40, 10);
            let bottom = state.scroll();
            assert!(bottom > 0);

            for _ in 0..10 {
                state.scroll_down();
            }
            render(&mut state, &sections, 40, 10);

            assert_eq!(state.scroll(), bottom);
        }

        #[test]
        fn scrolling_down_moves_the_content_up() {
            let sections = tall_sections(6);
            let mut state = SheetState::default();
            let before = content(&render(&mut state, &sections, 40, 10));

            state.scroll_down();
            let after = content(&render(&mut state, &sections, 40, 10));

            assert_eq!(state.scroll(), 1);
            assert_ne!(before, after);
        }

        #[test]
        fn scroll_is_clamped_when_the_area_grows() {
            let sections = tall_sections(6);
            let mut state = SheetState::default();
            render(&mut state, &sections, 40, 10);
            for _ in 0..20 {
                state.scroll_down();
            }
            assert!(state.scroll() > 0);

            render(&mut state, &sections, 40, 60);

            assert_eq!(state.scroll(), 0);
        }

        #[test]
        fn no_sections_renders_without_panicking() {
            let mut state = SheetState::default();
            render(&mut state, &[], 40, 10);

            assert_eq!(state.scroll(), 0);
        }

        #[test]
        fn tiny_area_renders_without_panicking() {
            let sections = tall_sections(4);
            render(&mut SheetState::default(), &sections, 3, 1);
        }
    }
}
