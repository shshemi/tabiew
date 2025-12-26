use ratatui::style::{Color, Style};

use crate::tui::themes::styler::Styler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Terminal;

impl Styler for Terminal {
    fn table_header(&self) -> Style {
        Style::default().bg(Color::DarkGray).fg(Color::White)
    }

    fn row(&self, _row: usize) -> Style {
        Style::default().bg(Color::Black).fg(Color::White)
    }

    fn row_highlighted(&self) -> Style {
        Style::default().bg(Color::LightYellow).fg(Color::Black)
    }

    fn header(&self, _col: usize) -> Style {
        Style::default().fg(Color::White)
    }

    fn tag(&self, idx: usize) -> Style {
        Style::default()
            .bg([
                Color::Red,
                Color::Magenta,
                Color::Blue,
                Color::Cyan,
                Color::Green,
            ][idx % 5])
            .fg(Color::Gray)
    }

    fn block_tag(&self) -> Style {
        Style::default().bg(Color::LightYellow).fg(Color::Gray)
    }

    fn block(&self) -> Style {
        Style::default().bg(Color::Black).fg(Color::White)
    }

    fn text(&self) -> Style {
        Style::default().bg(Color::Black).fg(Color::White)
    }

    fn subtext(&self) -> Style {
        Style::default().bg(Color::Black).fg(Color::DarkGray)
    }

    fn error(&self) -> Style {
        Style::default().bg(Color::Red).fg(Color::White)
    }

    fn graph(&self, _idx: usize) -> Style {
        Style::default().fg(Color::White)
    }

    fn text_highlighted(&self) -> Style {
        Style::default().fg(Color::Yellow)
    }

    fn gutter(&self, _: usize) -> Style {
        Style::default().bg(Color::Black).fg(Color::White)
    }
}
