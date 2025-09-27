
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct DarkModern;

impl SixColorsTwoRowsStyler for DarkModern {
    const BACKGROUND: Color = Color::from_u32(0x001f1f1f);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x003F3F3F);
    const FOREGROUND: Color = Color::from_u32(0x00cccccc);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00272727);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00dc5452),
        Color::from_u32(0x0023d18b),
        Color::from_u32(0x00f5f543),
        Color::from_u32(0x003b8eea),
        Color::from_u32(0x00d670d6),
        Color::from_u32(0x0029b8db),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00f74949),
        Color::from_u32(0x002ea043),
        Color::from_u32(0x009e6a03),
        Color::from_u32(0x000078d4),
        Color::from_u32(0x00d01273),
        Color::from_u32(0x001db4d6),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00252525), Color::from_u32(0x002B2B2B)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00DFDFDF);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00C71919);

    fn id(&self) -> &str {
        "dark_modern"
    }

    fn title(&self) -> &str {
        "DarkModern"
    }
}
