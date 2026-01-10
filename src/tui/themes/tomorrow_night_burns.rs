use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct TomorrowNightBurns;

impl SixColorsTwoRowsStyler for TomorrowNightBurns {
    const BACKGROUND: Color = Color::from_u32(0x00151515);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00353535);
    const FOREGROUND: Color = Color::from_u32(0x00a1b0b8);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00252525);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00832e31),
        Color::from_u32(0x00a63c40),
        Color::from_u32(0x00d2494e),
        Color::from_u32(0x00fc595f),
        Color::from_u32(0x00df9395),
        Color::from_u32(0x00ba8586),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00832e31),
        Color::from_u32(0x00a63c40),
        Color::from_u32(0x00d3494e),
        Color::from_u32(0x00fc595f),
        Color::from_u32(0x00df9395),
        Color::from_u32(0x00ba8586),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x001B1B1B), Color::from_u32(0x00212121)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00DF241E);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00CC292F);
}
