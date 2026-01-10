use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Firewatch;

impl SixColorsTwoRowsStyler for Firewatch {
    const BACKGROUND: Color = Color::from_u32(0x001e2027);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x003E4047);
    const FOREGROUND: Color = Color::from_u32(0x009ba2b2);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00585f6d);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00d95360),
        Color::from_u32(0x005ab977),
        Color::from_u32(0x00dfb563),
        Color::from_u32(0x004c89c5),
        Color::from_u32(0x00d55119),
        Color::from_u32(0x0044a8b6),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00d95360),
        Color::from_u32(0x005ab977),
        Color::from_u32(0x00dfb563),
        Color::from_u32(0x004d89c4),
        Color::from_u32(0x00d55119),
        Color::from_u32(0x0044a8b6),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x0024262D), Color::from_u32(0x002A2C33)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00D6D7CC);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00AF8533);
}
