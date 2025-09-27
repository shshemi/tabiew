
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct GhosttyDefaultStyleDark;

impl SixColorsTwoRowsStyler for GhosttyDefaultStyleDark {
    const BACKGROUND: Color = Color::from_u32(0x00282c34);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00484C54);
    const FOREGROUND: Color = Color::from_u32(0x00ffffff);
    const DARK_FOREGROUND: Color = Color::from_u32(0x001d1f21);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00d54e53),
        Color::from_u32(0x00b9ca4b),
        Color::from_u32(0x00e7c547),
        Color::from_u32(0x007aa6da),
        Color::from_u32(0x00c397d8),
        Color::from_u32(0x0070c0b1),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00cc6566),
        Color::from_u32(0x00b6bd68),
        Color::from_u32(0x00f0c674),
        Color::from_u32(0x0082a2be),
        Color::from_u32(0x00b294bb),
        Color::from_u32(0x008abeb7),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x002E323A), Color::from_u32(0x00343840)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00DFDFDF);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00C09644);

    fn id(&self) -> &str {
        "ghostty_default_style_dark"
    }

    fn title(&self) -> &str {
        "GhosttyDefaultStyleDark"
    }
}
