use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct MonokaiVivid;

impl SixColorsTwoRowsStyler for MonokaiVivid {
    const BACKGROUND: Color = Color::from_u32(0x00121212);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00323232);
    const FOREGROUND: Color = Color::from_u32(0x00f9f9f9);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00121212);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00f6669d),
        Color::from_u32(0x00b1e05f),
        Color::from_u32(0x00fff26d),
        Color::from_u32(0x000443ff),
        Color::from_u32(0x00f200f6),
        Color::from_u32(0x0051ceff),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00fa2934),
        Color::from_u32(0x0098e123),
        Color::from_u32(0x00fff30a),
        Color::from_u32(0x000443ff),
        Color::from_u32(0x00f800f8),
        Color::from_u32(0x0001b6ed),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00181818), Color::from_u32(0x001E1E1E)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00DB0000);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00CFC300);
}
