use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Oxocarbon;

impl SixColorsTwoRowsStyler for Oxocarbon {
    const BACKGROUND: Color = Color::from_u32(0x00161616);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00363636);
    const FOREGROUND: Color = Color::from_u32(0x00f2f4f8);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00161616);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x0000dfdb),
        Color::from_u32(0x0000b4ff),
        Color::from_u32(0x00ff4297),
        Color::from_u32(0x0000c15a),
        Color::from_u32(0x00c693ff),
        Color::from_u32(0x00ff74b8),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x0000dfdb),
        Color::from_u32(0x0000b4ff),
        Color::from_u32(0x00ff4297),
        Color::from_u32(0x0000c15a),
        Color::from_u32(0x00c693ff),
        Color::from_u32(0x00ff74b8),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x001C1C1C), Color::from_u32(0x00222222)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00DFDFDF);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00CF1267);
}
