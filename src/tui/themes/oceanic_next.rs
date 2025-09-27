
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct OceanicNext;

impl SixColorsTwoRowsStyler for OceanicNext {
    const BACKGROUND: Color = Color::from_u32(0x00162c35);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00364C55);
    const FOREGROUND: Color = Color::from_u32(0x00c0c5ce);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00162c35);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00ec5f67),
        Color::from_u32(0x0099c794),
        Color::from_u32(0x00fac863),
        Color::from_u32(0x006699cc),
        Color::from_u32(0x00c594c5),
        Color::from_u32(0x005fb3b3),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00ec5f67),
        Color::from_u32(0x0099c794),
        Color::from_u32(0x00fac863),
        Color::from_u32(0x006699cc),
        Color::from_u32(0x00c594c5),
        Color::from_u32(0x005fb3b3),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x001C323B), Color::from_u32(0x00223841)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00A0A5AE);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00CA9833);

    fn id(&self) -> &str {
        "oceanic_next"
    }

    fn title(&self) -> &str {
        "OceanicNext"
    }
}
