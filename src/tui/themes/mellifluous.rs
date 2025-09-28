
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Mellifluous;

impl SixColorsTwoRowsStyler for Mellifluous {
    const BACKGROUND: Color = Color::from_u32(0x001a1a1a);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x003A3A3A);
    const FOREGROUND: Color = Color::from_u32(0x00dadada);
    const DARK_FOREGROUND: Color = Color::from_u32(0x001a1a1a);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00c95954),
        Color::from_u32(0x00828040),
        Color::from_u32(0x00a6794c),
        Color::from_u32(0x005a6599),
        Color::from_u32(0x009c6995),
        Color::from_u32(0x0074a39e),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00d29393),
        Color::from_u32(0x00b3b393),
        Color::from_u32(0x00cbaa89),
        Color::from_u32(0x00a8a1be),
        Color::from_u32(0x00b39fb0),
        Color::from_u32(0x00c0af8c),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00202020), Color::from_u32(0x00262626)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x009F8D7E);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00A26363);
}
