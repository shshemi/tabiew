use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Teerb;

impl SixColorsTwoRowsStyler for Teerb {
    const BACKGROUND: Color = Color::from_u32(0x00262626);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00464646);
    const FOREGROUND: Color = Color::from_u32(0x00d0d0d0);
    const DARK_FOREGROUND: Color = Color::from_u32(0x001c1c1c);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00d68686),
        Color::from_u32(0x00aed686),
        Color::from_u32(0x00e4c9af),
        Color::from_u32(0x0086aed6),
        Color::from_u32(0x00d6aed6),
        Color::from_u32(0x00b1e7dd),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00d68686),
        Color::from_u32(0x00aed686),
        Color::from_u32(0x00d7af87),
        Color::from_u32(0x0086aed6),
        Color::from_u32(0x00d6aed6),
        Color::from_u32(0x008adbb4),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x002C2C2C), Color::from_u32(0x00323232)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00C4A98F);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00A77F57);
}
