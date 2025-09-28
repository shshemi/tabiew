
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct TomorrowNight;

impl SixColorsTwoRowsStyler for TomorrowNight {
    const BACKGROUND: Color = Color::from_u32(0x001d1f21);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x003D3F41);
    const FOREGROUND: Color = Color::from_u32(0x00c5c8c6);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00000000);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00cc6666),
        Color::from_u32(0x00b5bd68),
        Color::from_u32(0x00f0c674),
        Color::from_u32(0x0081a2be),
        Color::from_u32(0x00b294bb),
        Color::from_u32(0x008abeb7),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00cc6666),
        Color::from_u32(0x00b5bd68),
        Color::from_u32(0x00f0c674),
        Color::from_u32(0x0081a2be),
        Color::from_u32(0x00b294bb),
        Color::from_u32(0x008abeb7),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00232527), Color::from_u32(0x00292B2D)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00A5A8A6);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00C09644);
}
