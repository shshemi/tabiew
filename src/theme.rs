use ratatui::style::{Color, Style, Stylize};

pub trait Styler {
    fn table_header() -> Style;
    fn table_header_cell(col: usize) -> Style;
    fn table_row(row: usize) -> Style;
    fn table_highlight() -> Style;
    fn table_cell(row: usize, col: usize) -> Style;
    fn status_bar_red() -> Style;
    fn status_bar_green() -> Style;
    fn status_bar_blue() -> Style;
    fn item_block() -> Style;
}

pub struct Monokai;
pub struct Argonaut;
pub struct Terminal;

impl Styler for Monokai {
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
            Style::new()
                .bg(Color::from_u32(0x00232024))
                .fg(Color::from_u32(0x00fffaf4))
        } else {
            Style::new()
                .bg(Color::from_u32(0x001c191d))
                .fg(Color::from_u32(0x00fffaf4))
        }
    }

    fn table_highlight() -> Style {
        Style::new()
            .bg(Color::from_u32(0x00c89f2d))
            .fg(Color::from_u32(0x00fffaf4))
    }

    fn table_cell(_row: usize, _col: usize) -> Style {
        Style::default().fg(Color::from_u32(0x00fffaf4))
    }

    fn status_bar_red() -> Style {
        Style::default()
            .bg(Color::from_u32(0x00d02d00))
            .fg(Color::from_u32(0x00fffaf4))
    }

    fn status_bar_green() -> Style {
        Style::default()
            .bg(Color::from_u32(0x00008f1f))
            .fg(Color::from_u32(0x00fffaf4))
    }

    fn status_bar_blue() -> Style {
        Style::default()
            .bg(Color::from_u32(0x00007dd0))
            .fg(Color::from_u32(0x00fffaf4))
    }

    fn item_block() -> Style {
        Style::new()
            .bg(Color::from_u32(0x001c191d))
            .fg(Color::from_u32(0x00c89f2d))
    }
}

impl Styler for Argonaut {
    fn table_header() -> Style {
        Style::default().bg(Color::from_u32(0x0001030b))
    }

    fn table_header_cell(col: usize) -> Style {
        Style::default()
            .fg(match col % 6 {
                0 => Color::from_u32(0x00ff000f),
                1 => Color::from_u32(0x00ffb900),
                2 => Color::from_u32(0x00ffd866),
                3 => Color::from_u32(0x008ce10b),
                4 => Color::from_u32(0x006d43a6),
                5 => Color::from_u32(0x0000d8eb),
                _ => panic!("How!"),
            })
            .bold()
    }

    fn table_row(row: usize) -> Style {
        if row % 2 == 0 {
            Style::new()
                .bg(Color::from_u32(0x0011131b))
                .fg(Color::from_u32(0x00fffaf4))
        } else {
            Style::new()
                .bg(Color::from_u32(0x0001030b))
                .fg(Color::from_u32(0x00fffaf4))
        }
    }

    fn table_highlight() -> Style {
        Style::new().bg(Color::from_u32(0x00002a3b))
    }

    fn table_cell(_row: usize, _col: usize) -> Style {
        Style::default().fg(Color::from_u32(0x00fffaf4))
    }

    fn status_bar_red() -> Style {
        Style::default()
            .bg(Color::from_u32(0x00dd0000))
            .fg(Color::from_u32(0x00fffaf4))
    }

    fn status_bar_green() -> Style {
        Style::default()
            .bg(Color::from_u32(0x005cb100))
            .fg(Color::from_u32(0x00fffaf4))
    }

    fn status_bar_blue() -> Style {
        Style::default()
            .bg(Color::from_u32(0x00006dd8))
            .fg(Color::from_u32(0x00fffaf4))
    }

    fn item_block() -> Style {
        Style::new()
            .bg(Color::from_u32(0x000e1019))
            .fg(Color::from_u32(0x00fffaf4))
    }
}

impl Styler for Terminal {
    fn table_header() -> Style {
        Style::default().bg(Color::Cyan).fg(Color::Black)
    }

    fn table_header_cell(_col: usize) -> Style {
        Style::default()
    }

    fn table_row(_row: usize) -> Style {
        Default::default()
    }

    fn table_highlight() -> Style {
        Style::default().bg(Color::Yellow).fg(Color::Black)
    }

    fn table_cell(_row: usize, _col: usize) -> Style {
        Style::default()
    }

    fn status_bar_red() -> Style {
        Style::default().bg(Color::Red).fg(Color::White)
    }

    fn status_bar_green() -> Style {
        Style::default().bg(Color::Green).fg(Color::White)
    }

    fn status_bar_blue() -> Style {
        Style::default().bg(Color::Blue).fg(Color::White)
    }

    fn item_block() -> Style {
        Style::default()
    }
}
