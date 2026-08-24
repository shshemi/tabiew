use std::{
    borrow::Cow,
    time::{Duration, Instant},
};

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{Block, Paragraph, Widget, Wrap},
};

use crate::{
    misc::config::theme,
    sw::{app_default::AppDefault, buffer_ext::BufferExt, rect_ext::RectExt},
};

const TIMEOUT: Duration = Duration::from_secs(3);

#[derive(Debug)]
pub struct ToastState {
    start: Instant,
}

impl ToastState {
    pub fn new() -> Self {
        Self {
            start: Instant::now(),
        }
    }

    pub fn is_finished(&self) -> bool {
        self.start.elapsed() > TIMEOUT
    }
}

impl Default for ToastState {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Default)]
pub struct Toast<'a> {
    message: Cow<'a, str>,
}

impl<'a> Toast<'a> {
    pub fn message(mut self, message: impl Into<Cow<'a, str>>) -> Self {
        self.message = message.into();
        self
    }
}

impl Widget for Toast<'_> {
    fn render(self, _area: Rect, buf: &mut Buffer) {
        let paragraph = paragraph(&self.message);
        let area = buf.area.toast(&paragraph);

        buf.clear(area);
        paragraph.render(area, buf);
    }
}

fn paragraph(message: &str) -> Paragraph<'_> {
    Paragraph::new(message)
        .style(theme().text())
        .left_aligned()
        .block(Block::app_default())
        .wrap(Wrap { trim: true })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn render(message: &str, width: u16, height: u16) -> Buffer {
        let area = Rect::new(0, 0, width, height);
        let mut buf = Buffer::empty(area);
        Toast::default().message(message).render(area, &mut buf);
        buf
    }

    fn content(buf: &Buffer) -> String {
        buf.content().iter().map(|c| c.symbol()).collect()
    }

    fn row(buf: &Buffer, y: u16) -> String {
        (0..buf.area.width).map(|x| buf[(x, y)].symbol()).collect()
    }

    fn column_of(buf: &Buffer, y: u16, symbol: &str) -> Option<u16> {
        (0..buf.area.width).find(|x| buf[(*x, y)].symbol() == symbol)
    }

    mod state {
        use super::*;

        #[test]
        fn a_fresh_toast_is_not_finished() {
            assert!(!ToastState::new().is_finished());
        }

        #[test]
        fn a_toast_is_finished_once_the_timeout_elapses() {
            let mut state = ToastState::new();
            state.start -= TIMEOUT + Duration::from_secs(1);

            assert!(state.is_finished());
        }

        #[test]
        fn a_toast_is_not_finished_right_before_the_timeout() {
            let mut state = ToastState::new();
            state.start -= TIMEOUT - Duration::from_millis(500);

            assert!(!state.is_finished());
        }
    }

    mod widget {
        use super::*;

        #[test]
        fn renders_the_message() {
            let buf = render("saved", 100, 30);

            assert!(content(&buf).contains("saved"));
        }

        #[test]
        fn is_placed_where_rect_ext_puts_it() {
            let buf = render("saved", 100, 30);
            let expected = Rect::new(0, 0, 100, 30).toast(&paragraph("saved"));

            assert_eq!(column_of(&buf, expected.y, "╭"), Some(expected.x));
            assert_eq!(column_of(&buf, expected.y, "╮"), Some(expected.right() - 1));
            assert_eq!(
                column_of(&buf, expected.bottom() - 1, "╰"),
                Some(expected.x)
            );
        }

        #[test]
        fn leaves_the_rows_below_it_untouched() {
            let buf = render("saved", 100, 30);
            let expected = Rect::new(0, 0, 100, 30).toast(&paragraph("saved"));

            for y in expected.bottom()..30 {
                assert!(row(&buf, y).trim().is_empty(), "row {y} should be empty");
            }
        }

        #[test]
        fn is_wrapped_in_a_border() {
            let buf = render("saved", 100, 30);
            let content = content(&buf);

            assert!(content.contains('╭'));
            assert!(content.contains('╮'));
            assert!(content.contains('╰'));
            assert!(content.contains('╯'));
        }

        #[test]
        fn grows_taller_as_the_message_wraps() {
            let short_buf = render("saved", 100, 30);
            let long_buf = render(&"w ".repeat(80), 100, 30);

            let box_height = |buf: &Buffer| {
                (0..30)
                    .filter(|y| {
                        let row = row(buf, *y);
                        row.contains('│') || row.contains('╭') || row.contains('╰')
                    })
                    .count()
            };

            assert!(box_height(&long_buf) > box_height(&short_buf));
        }

        #[test]
        fn narrow_buffer_renders_without_panicking() {
            render("saved", 2, 6);
        }
    }
}
