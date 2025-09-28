
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct CobaltNeon;

impl SixColorsTwoRowsStyler for CobaltNeon {
    const BACKGROUND: Color = Color::from_u32(0x00142838);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00344858);
    const FOREGROUND: Color = Color::from_u32(0x008ff586);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00142631);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00d4312e),
        Color::from_u32(0x008ff586),
        Color::from_u32(0x00e9f06d),
        Color::from_u32(0x003c7dd2),
        Color::from_u32(0x008230a7),
        Color::from_u32(0x006cbc67),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00ff2320),
        Color::from_u32(0x003ba5ff),
        Color::from_u32(0x00e9e75c),
        Color::from_u32(0x008ff586),
        Color::from_u32(0x00781aa0),
        Color::from_u32(0x008ff586),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x001A2E3E), Color::from_u32(0x00203444)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00A4004F);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00CF0000);
}
