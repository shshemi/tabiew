use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct TinaciousDesignDark;

impl SixColorsTwoRowsStyler for TinaciousDesignDark {
    const BACKGROUND: Color = Color::from_u32(0x001d1d26);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x003D3D46);
    const FOREGROUND: Color = Color::from_u32(0x00cbcbf0);
    const DARK_FOREGROUND: Color = Color::from_u32(0x001d1d26);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00ff2f92),
        Color::from_u32(0x0000d364),
        Color::from_u32(0x00ffd479),
        Color::from_u32(0x0000cbff),
        Color::from_u32(0x00d783ff),
        Color::from_u32(0x0000d5d4),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00ff3399),
        Color::from_u32(0x0000d364),
        Color::from_u32(0x00ffcc66),
        Color::from_u32(0x0000cbff),
        Color::from_u32(0x00cc66ff),
        Color::from_u32(0x0000ceca),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x0023232C), Color::from_u32(0x00292932)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00ABABD0);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00CF9C36);
}
