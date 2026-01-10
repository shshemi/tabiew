use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct FarmhouseDark;

impl SixColorsTwoRowsStyler for FarmhouseDark {
    const BACKGROUND: Color = Color::from_u32(0x001d2027);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x003D4047);
    const FOREGROUND: Color = Color::from_u32(0x00e8e4e1);
    const DARK_FOREGROUND: Color = Color::from_u32(0x001d2027);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00eb0009),
        Color::from_u32(0x007ac100),
        Color::from_u32(0x00ea9a00),
        Color::from_u32(0x00006efe),
        Color::from_u32(0x00bf3b7f),
        Color::from_u32(0x0019e062),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00ba0004),
        Color::from_u32(0x00549d00),
        Color::from_u32(0x00c87300),
        Color::from_u32(0x000049e6),
        Color::from_u32(0x009f1b61),
        Color::from_u32(0x001fb65c),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x0023262D), Color::from_u32(0x00292C33)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00004EDE);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00984300);
}
