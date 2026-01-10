use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Fideloper;

impl SixColorsTwoRowsStyler for Fideloper {
    const BACKGROUND: Color = Color::from_u32(0x00292f33);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00494F53);
    const FOREGROUND: Color = Color::from_u32(0x00dbdae0);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00292f33);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00d4605a),
        Color::from_u32(0x00d4605a),
        Color::from_u32(0x00a86671),
        Color::from_u32(0x007c85c4),
        Color::from_u32(0x005c5db2),
        Color::from_u32(0x00819090),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00cb1e2d),
        Color::from_u32(0x00edb8ac),
        Color::from_u32(0x00b7ab9b),
        Color::from_u32(0x002e78c2),
        Color::from_u32(0x00c0236f),
        Color::from_u32(0x00309186),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x002F3539), Color::from_u32(0x00353B3F)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00B4403A);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00BD887C);
}
