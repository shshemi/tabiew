
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct HavnSkumring;

impl SixColorsTwoRowsStyler for HavnSkumring {
    const BACKGROUND: Color = Color::from_u32(0x00111522);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00313542);
    const FOREGROUND: Color = Color::from_u32(0x00d6dbeb);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00252c47);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00d17264),
        Color::from_u32(0x008c9e8f),
        Color::from_u32(0x00eac58c),
        Color::from_u32(0x005186cb),
        Color::from_u32(0x009b7cee),
        Color::from_u32(0x00d17ab6),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00ea563e),
        Color::from_u32(0x006ead7b),
        Color::from_u32(0x00f8b330),
        Color::from_u32(0x00596cf7),
        Color::from_u32(0x007c719e),
        Color::from_u32(0x00d588c1),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00171B28), Color::from_u32(0x001D212E)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00075A4F);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00C88300);
}
