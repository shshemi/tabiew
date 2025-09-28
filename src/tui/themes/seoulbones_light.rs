
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct SeoulbonesLight;

impl SixColorsTwoRowsStyler for SeoulbonesLight {
    const BACKGROUND: Color = Color::from_u32(0x00e2e2e2);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00FFFFFF);
    const FOREGROUND: Color = Color::from_u32(0x00555555);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00e2e2e2);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00be3c6d),
        Color::from_u32(0x00487249),
        Color::from_u32(0x00a76b48),
        Color::from_u32(0x00006f89),
        Color::from_u32(0x007f4c7e),
        Color::from_u32(0x00006f70),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00dc5284),
        Color::from_u32(0x00628562),
        Color::from_u32(0x00c48562),
        Color::from_u32(0x000084a3),
        Color::from_u32(0x00896788),
        Color::from_u32(0x00008586),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00E8E8E8), Color::from_u32(0x00EEEEEE)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00353535);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00AC2254);
}
