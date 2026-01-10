use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Breadog;

impl SixColorsTwoRowsStyler for Breadog {
    const BACKGROUND: Color = Color::from_u32(0x00f1ebe6);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00FFFFFF);
    const FOREGROUND: Color = Color::from_u32(0x00362c24);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00362c24);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00de1100),
        Color::from_u32(0x00008f40),
        Color::from_u32(0x00ae6000),
        Color::from_u32(0x000074e1),
        Color::from_u32(0x00c300bd),
        Color::from_u32(0x00008697),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00b10b00),
        Color::from_u32(0x00007232),
        Color::from_u32(0x008b4c00),
        Color::from_u32(0x00005cb4),
        Color::from_u32(0x009b0097),
        Color::from_u32(0x00006a78),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00F7F1EC), Color::from_u32(0x00FDF7F2)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00160C04);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00810000);
}
