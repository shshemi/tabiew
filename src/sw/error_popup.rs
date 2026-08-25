use std::borrow::Cow;

use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    widgets::{Block, Paragraph, Widget, Wrap},
};

use crate::{
    misc::config::theme,
    sw::{app_default::AppDefault, buffer_ext::BufferExt, rect_ext::RectExt},
};

#[derive(Debug, Default)]
pub struct ErrorPopup<'a> {
    message: Cow<'a, str>,
}

impl<'a> ErrorPopup<'a> {
    pub fn message(mut self, message: impl Into<Cow<'a, str>>) -> Self {
        self.message = message.into();
        self
    }
}

impl Widget for ErrorPopup<'_> {
    fn render(self, _area: Rect, buf: &mut Buffer) {
        buf.darken();

        let paragraph = paragraph(&self.message);
        let area = buf.area.popup(&paragraph);

        buf.clear(area);
        paragraph.render(area, buf);
    }
}

fn paragraph(message: &str) -> Paragraph<'_> {
    Paragraph::new(message)
        .left_aligned()
        .block(
            Block::app_default()
                .title(" Error ")
                .title_alignment(Alignment::Center)
                .style(theme().error()),
        )
        .wrap(Wrap { trim: true })
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::style::Color;

    fn render(message: &str, width: u16, height: u16) -> Buffer {
        let area = Rect::new(0, 0, width, height);
        let mut buf = Buffer::empty(area);
        ErrorPopup::default()
            .message(message)
            .render(area, &mut buf);
        buf
    }

    fn content(buf: &Buffer) -> String {
        buf.content().iter().map(|c| c.symbol()).collect()
    }

    fn column_of(buf: &Buffer, y: u16, symbol: &str) -> Option<u16> {
        (0..buf.area.width).find(|x| buf[(*x, y)].symbol() == symbol)
    }

    #[test]
    fn renders_the_message() {
        let buf = render("file not found", 100, 30);

        assert!(content(&buf).contains("file not found"));
    }

    #[test]
    fn is_titled_error() {
        let buf = render("file not found", 100, 30);

        assert!(content(&buf).contains("Error"));
    }

    #[test]
    fn a_message_narrower_than_the_title_crops_it() {
        let buf = render("x", 100, 30);

        assert!(!content(&buf).contains("Error"));
    }

    #[test]
    fn is_wrapped_in_a_border() {
        let buf = render("boom", 100, 30);
        let content = content(&buf);

        assert!(content.contains('╭'));
        assert!(content.contains('╮'));
        assert!(content.contains('╰'));
        assert!(content.contains('╯'));
    }

    #[test]
    fn is_placed_where_rect_ext_puts_it() {
        let buf = render("boom", 100, 30);
        let expected = Rect::new(0, 0, 100, 30).popup(&paragraph("boom"));

        assert_eq!(column_of(&buf, expected.y, "╭"), Some(expected.x));
        assert_eq!(column_of(&buf, expected.y, "╮"), Some(expected.right() - 1));
        assert_eq!(
            column_of(&buf, expected.bottom() - 1, "╰"),
            Some(expected.x)
        );
    }

    #[test]
    fn darkens_the_background_behind_it() {
        let area = Rect::new(0, 0, 100, 30);
        let mut buf = Buffer::empty(area);
        buf[(0, 0)].set_bg(Color::Rgb(100, 150, 200));
        buf[(0, 0)].set_fg(Color::Rgb(100, 150, 200));

        ErrorPopup::default().message("boom").render(area, &mut buf);

        assert_eq!(buf[(0, 0)].bg, Color::Rgb(20, 30, 40));
        assert_eq!(buf[(0, 0)].fg, Color::Rgb(20, 30, 40));
    }

    #[test]
    fn long_messages_wrap_within_the_capped_width() {
        let buf = render(&"w ".repeat(80), 100, 30);
        let y = (0..30)
            .find(|y| column_of(&buf, *y, "╭").is_some())
            .unwrap();
        let left = column_of(&buf, y, "╭").unwrap();
        let right = column_of(&buf, y, "╮").unwrap();

        assert_eq!(right - left + 1, 64);
    }

    #[test]
    fn narrow_buffer_renders_without_panicking() {
        render("boom", 2, 4);
    }
}
