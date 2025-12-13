use ratatui::style::{Color, Style, Stylize};
use std::fmt::Debug;

pub trait Styler: Debug {
    fn table_header(&self) -> Style;
    fn row(&self, row: usize) -> Style;
    fn row_highlighted(&self) -> Style;
    fn header(&self, col: usize) -> Style;
    fn tag(&self, col: usize) -> Style;
    fn block(&self) -> Style;
    fn block_tag(&self) -> Style;
    fn text(&self) -> Style;
    fn text_highlighted(&self) -> Style;
    fn subtext(&self) -> Style;
    fn error(&self) -> Style;
    fn graph(&self, idx: usize) -> Style;
    fn gutter(&self, idx: usize) -> Style;
}

pub trait SixColorsTwoRowsStyler {
    const BACKGROUND: Color;
    const LIGHT_BACKGROUND: Color;
    const FOREGROUND: Color;
    const DARK_FOREGROUND: Color;

    const COLORS: [Color; 6];
    const DARK_COLORS: [Color; 6];

    const ROW_BACKGROUNDS: [Color; 2];
    const HIGHLIGHT_BACKGROUND: Color;
    const HIGHLIGHT_FOREGROUND: Color;

    const STATUS_BAR_ERROR: Color;
}

impl<T> Styler for T
where
    T: SixColorsTwoRowsStyler + Debug,
{
    fn table_header(&self) -> Style {
        Style::default().bg(Self::BACKGROUND)
    }

    fn header(&self, idx: usize) -> Style {
        Style::default()
            .fg(Self::COLORS[idx % Self::COLORS.len()])
            .bold()
    }

    fn row(&self, idx: usize) -> Style {
        Style::new()
            .bg(Self::ROW_BACKGROUNDS[idx % Self::ROW_BACKGROUNDS.len()])
            .fg(Self::FOREGROUND)
    }

    fn row_highlighted(&self) -> Style {
        Style::new()
            .bg(Self::HIGHLIGHT_BACKGROUND)
            .fg(Self::HIGHLIGHT_FOREGROUND)
    }

    fn tag(&self, idx: usize) -> Style {
        Style::default()
            .bg(Self::DARK_COLORS[idx % Self::DARK_COLORS.len()])
            .fg(Self::LIGHT_BACKGROUND)
    }

    fn block(&self) -> Style {
        Style::default()
            .bg(Self::BACKGROUND)
            .fg(Self::HIGHLIGHT_BACKGROUND)
    }

    fn block_tag(&self) -> Style {
        Style::default()
            .bg(Self::HIGHLIGHT_BACKGROUND)
            .fg(Self::LIGHT_BACKGROUND)
    }

    fn text(&self) -> Style {
        Style::default().bg(Self::BACKGROUND).fg(Self::FOREGROUND)
    }

    fn text_highlighted(&self) -> Style {
        Style::default()
            .bg(Self::BACKGROUND)
            .fg(Self::HIGHLIGHT_BACKGROUND)
    }

    fn subtext(&self) -> Style {
        Style::default()
            .bg(Self::BACKGROUND)
            .fg(Self::DARK_FOREGROUND)
    }

    fn error(&self) -> Style {
        Style::default()
            .bg(Self::STATUS_BAR_ERROR)
            .fg(Self::FOREGROUND)
    }

    fn graph(&self, idx: usize) -> Style {
        Style::default()
            .fg(Self::DARK_COLORS[idx % Self::DARK_COLORS.len()])
            .bold()
    }

    fn gutter(&self, idx: usize) -> Style {
        Style::new()
            .bg(Self::ROW_BACKGROUNDS[idx % Self::ROW_BACKGROUNDS.len()])
            .fg(Self::DARK_FOREGROUND)
    }
}
