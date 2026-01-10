use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Glacier;

impl SixColorsTwoRowsStyler for Glacier {
    const BACKGROUND: Color = Color::from_u32(0x000c1115);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x002C3135);
    const FOREGROUND: Color = Color::from_u32(0x00ffffff);
    const DARK_FOREGROUND: Color = Color::from_u32(0x002e343c);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00bd0f2f),
        Color::from_u32(0x0049e998),
        Color::from_u32(0x00fddf6e),
        Color::from_u32(0x002a8bc1),
        Color::from_u32(0x00ea4727),
        Color::from_u32(0x00a0b6d3),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00bd0f2f),
        Color::from_u32(0x0035a770),
        Color::from_u32(0x00fb9435),
        Color::from_u32(0x001f5872),
        Color::from_u32(0x00bd2523),
        Color::from_u32(0x00778397),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x0012171B), Color::from_u32(0x00181D21)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x004C4C4C);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00CB6405);
}
