
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct PoppingAndLocking;

impl SixColorsTwoRowsStyler for PoppingAndLocking {
    const BACKGROUND: Color = Color::from_u32(0x00181921);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00383941);
    const FOREGROUND: Color = Color::from_u32(0x00ebdbb2);
    const DARK_FOREGROUND: Color = Color::from_u32(0x001d2021);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00f42c3e),
        Color::from_u32(0x00b8bb26),
        Color::from_u32(0x00fabd2f),
        Color::from_u32(0x0099c6ca),
        Color::from_u32(0x00d3869b),
        Color::from_u32(0x007ec16e),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00cc241d),
        Color::from_u32(0x0098971a),
        Color::from_u32(0x00d79921),
        Color::from_u32(0x00458588),
        Color::from_u32(0x00b16286),
        Color::from_u32(0x00689d6a),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x001E1F27), Color::from_u32(0x0024252D)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00A7A7A7);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00A76900);
}
