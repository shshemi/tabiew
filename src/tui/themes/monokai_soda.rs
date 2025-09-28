
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct MonokaiSoda;

impl SixColorsTwoRowsStyler for MonokaiSoda {
    const BACKGROUND: Color = Color::from_u32(0x001a1a1a);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x003A3A3A);
    const FOREGROUND: Color = Color::from_u32(0x00c4c5b5);
    const DARK_FOREGROUND: Color = Color::from_u32(0x001a1a1a);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00f4005f),
        Color::from_u32(0x0098e024),
        Color::from_u32(0x00e0d561),
        Color::from_u32(0x009d65ff),
        Color::from_u32(0x00f4005f),
        Color::from_u32(0x0058d1eb),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00f4005f),
        Color::from_u32(0x0098e024),
        Color::from_u32(0x00fa8419),
        Color::from_u32(0x009d65ff),
        Color::from_u32(0x00f4005f),
        Color::from_u32(0x0058d1eb),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00202020), Color::from_u32(0x00262626)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00D6D7CC);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00CA5400);
}
