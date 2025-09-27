
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct MonokaiProLightSun;

impl SixColorsTwoRowsStyler for MonokaiProLightSun {
    const BACKGROUND: Color = Color::from_u32(0x00f8efe7);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00FFFFFF);
    const FOREGROUND: Color = Color::from_u32(0x002c232e);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00f8efe7);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00ce4770),
        Color::from_u32(0x00218871),
        Color::from_u32(0x00b16803),
        Color::from_u32(0x00d4572b),
        Color::from_u32(0x006851a2),
        Color::from_u32(0x002473b6),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00ce4770),
        Color::from_u32(0x00218871),
        Color::from_u32(0x00b16803),
        Color::from_u32(0x00d4572b),
        Color::from_u32(0x006851a2),
        Color::from_u32(0x002473b6),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00FEF5ED), Color::from_u32(0x00FFFBF3)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x0052494D);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00A42700);

    fn id(&self) -> &str {
        "monokai_pro_light_sun"
    }

    fn title(&self) -> &str {
        "MonokaiProLightSun"
    }
}
