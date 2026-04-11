use ratatui::style::{Color, Modifier, Style};

use crate::tui::themes::styler::Styler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Terminal;

const COLORS: [Color; 6] = [
    Color::Red,
    Color::Yellow,
    Color::Green,
    Color::Cyan,
    Color::Blue,
    Color::Magenta,
];

impl Styler for Terminal {
    fn table_header(&self) -> Style {
        Style::default()
            .bg(Color::Reset)
            .add_modifier(Modifier::BOLD)
    }

    fn row(&self, _row: usize) -> Style {
        Style::default().bg(Color::Reset).fg(Color::Reset)
    }

    fn row_highlighted(&self) -> Style {
        Style::default()
            .bg(Color::Reset)
            .fg(Color::Reset)
            .add_modifier(Modifier::REVERSED)
    }

    fn header(&self, idx: usize) -> Style {
        Style::default().fg(COLORS[idx % COLORS.len()]).bold()
    }

    fn tag(&self, idx: usize) -> Style {
        Style::default()
            .bg(Color::Indexed(236))
            .fg(COLORS[idx % COLORS.len()])
    }

    fn block_tag(&self) -> Style {
        Style::default()
            .bg(Color::Reset)
            .fg(Color::Yellow)
            .add_modifier(Modifier::REVERSED)
    }

    fn block(&self) -> Style {
        Style::default().bg(Color::Reset).fg(Color::Yellow)
    }

    fn text(&self) -> Style {
        Style::default().bg(Color::Reset).fg(Color::Reset)
    }

    fn subtext(&self) -> Style {
        Style::default()
            .bg(Color::Reset)
            .fg(Color::Reset)
            .add_modifier(Modifier::DIM)
    }

    fn error(&self) -> Style {
        Style::default()
            .bg(Color::Red)
            .fg(Color::Reset)
            .add_modifier(Modifier::REVERSED)
    }

    fn graph(&self, idx: usize) -> Style {
        Style::default().fg(COLORS[idx % COLORS.len()]).bold()
    }

    fn text_highlighted(&self) -> Style {
        Style::default()
            .bg(Color::Reset)
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD)
    }

    fn gutter(&self, _: usize) -> Style {
        Style::default().bg(Color::Reset).fg(Color::Reset)
    }
}
