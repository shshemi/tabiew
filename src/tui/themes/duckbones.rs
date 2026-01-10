use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Duckbones;

impl SixColorsTwoRowsStyler for Duckbones {
    const BACKGROUND: Color = Color::from_u32(0x000e101a);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x002E303A);
    const FOREGROUND: Color = Color::from_u32(0x00ebefc0);
    const DARK_FOREGROUND: Color = Color::from_u32(0x000e101a);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00ff4821),
        Color::from_u32(0x0058db9e),
        Color::from_u32(0x00f6a100),
        Color::from_u32(0x0000b4e0),
        Color::from_u32(0x00b3a1e6),
        Color::from_u32(0x0000b4e0),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00e03600),
        Color::from_u32(0x005dcd97),
        Color::from_u32(0x00e39500),
        Color::from_u32(0x0000a3cb),
        Color::from_u32(0x00795ccc),
        Color::from_u32(0x0000a3cb),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00141620), Color::from_u32(0x001A1C26)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00CDD2A2);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00B36500);
}
