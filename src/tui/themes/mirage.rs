
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Mirage;

impl SixColorsTwoRowsStyler for Mirage {
    const BACKGROUND: Color = Color::from_u32(0x001b2738);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x003B4758);
    const FOREGROUND: Color = Color::from_u32(0x00a6b2c0);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00011627);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00ff9999),
        Color::from_u32(0x0085cc95),
        Color::from_u32(0x00ffd700),
        Color::from_u32(0x007fb5ff),
        Color::from_u32(0x00ddb3ff),
        Color::from_u32(0x0085cc95),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00ff9999),
        Color::from_u32(0x0085cc95),
        Color::from_u32(0x00ffd700),
        Color::from_u32(0x007fb5ff),
        Color::from_u32(0x00ddb3ff),
        Color::from_u32(0x0021c7a8),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00212D3E), Color::from_u32(0x00273344)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00BD93DF);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00CFA700);
}
