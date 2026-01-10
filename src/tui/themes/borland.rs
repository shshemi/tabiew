use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Borland;

impl SixColorsTwoRowsStyler for Borland {
    const BACKGROUND: Color = Color::from_u32(0x000000a4);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x002020C4);
    const FOREGROUND: Color = Color::from_u32(0x00ffff4e);
    const DARK_FOREGROUND: Color = Color::from_u32(0x004f4f4f);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00ffb6b0),
        Color::from_u32(0x00ceffac),
        Color::from_u32(0x00ffffcc),
        Color::from_u32(0x00b5dcff),
        Color::from_u32(0x00ff9cfe),
        Color::from_u32(0x00dfdffe),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00ff6c60),
        Color::from_u32(0x00a8ff60),
        Color::from_u32(0x00ffffb6),
        Color::from_u32(0x0096cbfe),
        Color::from_u32(0x00ff73fd),
        Color::from_u32(0x00c6c5fe),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x000606AA), Color::from_u32(0x000C0CB0)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00DF8540);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00CF3C30);
}
