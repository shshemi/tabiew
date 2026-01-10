use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Wombat;

impl SixColorsTwoRowsStyler for Wombat {
    const BACKGROUND: Color = Color::from_u32(0x00171717);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00373737);
    const FOREGROUND: Color = Color::from_u32(0x00dedacf);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00000000);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00f58c80),
        Color::from_u32(0x00ddf88f),
        Color::from_u32(0x00eee5b2),
        Color::from_u32(0x00a5c7ff),
        Color::from_u32(0x00ddaaff),
        Color::from_u32(0x00b7fff9),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00ff615a),
        Color::from_u32(0x00b1e969),
        Color::from_u32(0x00ebd99c),
        Color::from_u32(0x005da9f6),
        Color::from_u32(0x00e86aff),
        Color::from_u32(0x0082fff7),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x001D1D1D), Color::from_u32(0x00232323)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x009B9B9B);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00CF312A);
}
