use ratatui::style::{Color, Style, Stylize};

pub trait Styler {
    fn table_header() -> Style;
    fn table_header_cell(col: usize) -> Style;
    fn table_row(row: usize) -> Style;
    fn table_highlight() -> Style;
    fn table_cell(row: usize, col: usize) -> Style;
    fn status_bar() -> Style;
}

pub struct Theme;

impl Styler for Theme {
    fn table_header() -> Style {
        Style::default().bg(Color::from_u32(0x001c191d))
    }

    fn table_header_cell(col: usize) -> Style {
        Style::default()
            .fg(match col % 6 {
                0 => Color::from_u32(0x00ff6188),
                1 => Color::from_u32(0x00fc9867),
                2 => Color::from_u32(0x00ffd866),
                3 => Color::from_u32(0x00a9dc76),
                4 => Color::from_u32(0x0078dce8),
                5 => Color::from_u32(0x00ab9df2),
                _ => panic!("How!"),
            })
            .bold()
    }

    fn table_row(row: usize) -> Style {
        if row % 2 == 0 {
            Style::new().bg(Color::from_u32(0x00232024))
        } else {
            Style::new().bg(Color::from_u32(0x001c191d))
        }
    }

    fn table_highlight() -> Style {
        Style::new().bg(Color::from_u32(0x00c89f2d))
    }

    fn table_cell(_row: usize, _col: usize) -> Style {
        Style::default().fg(Color::White)
    }

    fn status_bar() -> Style {
        Style::default().bg(Color::from_u32(0x00007dd0)).fg(Color::White)
    }
}
