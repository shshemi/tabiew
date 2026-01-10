use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct XcodeLightHc;

impl SixColorsTwoRowsStyler for XcodeLightHc {
    const BACKGROUND: Color = Color::from_u32(0x00ffffff);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00FFFFFF);
    const FOREGROUND: Color = Color::from_u32(0x00000000);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00b4d8fd);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00ad1805),
        Color::from_u32(0x00174145),
        Color::from_u32(0x0078492a),
        Color::from_u32(0x00003f73),
        Color::from_u32(0x009c2191),
        Color::from_u32(0x00441ea1),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00ad1805),
        Color::from_u32(0x00355d61),
        Color::from_u32(0x0078492a),
        Color::from_u32(0x000058a1),
        Color::from_u32(0x009c2191),
        Color::from_u32(0x00703daa),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00FFFFFF), Color::from_u32(0x00FFFFFF)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00000000);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x007D0000);
}
