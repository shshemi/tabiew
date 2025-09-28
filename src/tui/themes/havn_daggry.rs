
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct HavnDaggry;

impl SixColorsTwoRowsStyler for HavnDaggry {
    const BACKGROUND: Color = Color::from_u32(0x00f8f9fb);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00FFFFFF);
    const FOREGROUND: Color = Color::from_u32(0x003b4a7a);
    const DARK_FOREGROUND: Color = Color::from_u32(0x001f2842);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00cc4a35),
        Color::from_u32(0x00719679),
        Color::from_u32(0x00feb234),
        Color::from_u32(0x006089c0),
        Color::from_u32(0x007d7396),
        Color::from_u32(0x00aa869d),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00985248),
        Color::from_u32(0x00577159),
        Color::from_u32(0x00be6b00),
        Color::from_u32(0x003a577d),
        Color::from_u32(0x007c5c97),
        Color::from_u32(0x00925780),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00FEFFFF), Color::from_u32(0x00FFFFFF)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00024C2F);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x008E3B00);
}
