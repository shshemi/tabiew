use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Kibble;

impl SixColorsTwoRowsStyler for Kibble {
    const BACKGROUND: Color = Color::from_u32(0x000e100a);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x002E302A);
    const FOREGROUND: Color = Color::from_u32(0x00f7f7f7);
    const DARK_FOREGROUND: Color = Color::from_u32(0x004d4d4d);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00f01578),
        Color::from_u32(0x006ce05c),
        Color::from_u32(0x00f3f79e),
        Color::from_u32(0x0097a4f7),
        Color::from_u32(0x00c495f0),
        Color::from_u32(0x0068f2e0),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00c70031),
        Color::from_u32(0x0029cf13),
        Color::from_u32(0x00d8e30e),
        Color::from_u32(0x003449d1),
        Color::from_u32(0x008400ff),
        Color::from_u32(0x000798ab),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00141610), Color::from_u32(0x001A1C16)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x007FBA7C);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00A8B300);
}
