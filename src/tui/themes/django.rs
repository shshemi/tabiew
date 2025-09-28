
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Django;

impl SixColorsTwoRowsStyler for Django {
    const BACKGROUND: Color = Color::from_u32(0x000b2f20);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x002B4F40);
    const FOREGROUND: Color = Color::from_u32(0x00f8f8f8);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00000000);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00ff943b),
        Color::from_u32(0x0073da70),
        Color::from_u32(0x00ffff94),
        Color::from_u32(0x00568264),
        Color::from_u32(0x00ffffff),
        Color::from_u32(0x00cfffd1),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00fd6209),
        Color::from_u32(0x0041a83e),
        Color::from_u32(0x00ffe862),
        Color::from_u32(0x00245032),
        Color::from_u32(0x00f8f8f8),
        Color::from_u32(0x009df39f),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00113526), Color::from_u32(0x00173B2C)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00134422);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00CFB832);
}
