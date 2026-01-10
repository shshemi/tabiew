use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Smyck;

impl SixColorsTwoRowsStyler for Smyck {
    const BACKGROUND: Color = Color::from_u32(0x001b1b1b);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x003B3B3B);
    const FOREGROUND: Color = Color::from_u32(0x00f7f7f7);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00000000);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00d6837c),
        Color::from_u32(0x00c4f137),
        Color::from_u32(0x00fee14d),
        Color::from_u32(0x008dcff0),
        Color::from_u32(0x00f79aff),
        Color::from_u32(0x006ad9cf),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00b84131),
        Color::from_u32(0x007da900),
        Color::from_u32(0x00c4a500),
        Color::from_u32(0x0062a3c4),
        Color::from_u32(0x00ba8acc),
        Color::from_u32(0x00207383),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00212121), Color::from_u32(0x00272727)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x009B9B9B);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00947500);
}
