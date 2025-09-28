
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct IcOrangePpl;

impl SixColorsTwoRowsStyler for IcOrangePpl {
    const BACKGROUND: Color = Color::from_u32(0x00262626);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00464646);
    const FOREGROUND: Color = Color::from_u32(0x00ffcb83);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00000000);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00ff8c68),
        Color::from_u32(0x00f6ff40),
        Color::from_u32(0x00ffe36e),
        Color::from_u32(0x00ffbe55),
        Color::from_u32(0x00fc874f),
        Color::from_u32(0x00c69752),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00c13900),
        Color::from_u32(0x00a4a900),
        Color::from_u32(0x00caaf00),
        Color::from_u32(0x00bd6d00),
        Color::from_u32(0x00fc5e00),
        Color::from_u32(0x00f79500),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x002C2C2C), Color::from_u32(0x00323232)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00DC3300);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00CC2E00);
}
