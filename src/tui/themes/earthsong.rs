
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Earthsong;

impl SixColorsTwoRowsStyler for Earthsong {
    const BACKGROUND: Color = Color::from_u32(0x00292520);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00494540);
    const FOREGROUND: Color = Color::from_u32(0x00e5c7a9);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00121418);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00ff645a),
        Color::from_u32(0x0098e036),
        Color::from_u32(0x00e0d561),
        Color::from_u32(0x005fdaff),
        Color::from_u32(0x00ff9269),
        Color::from_u32(0x0084f088),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00c94234),
        Color::from_u32(0x0085c54c),
        Color::from_u32(0x00f5ae2e),
        Color::from_u32(0x001398b9),
        Color::from_u32(0x00d0633d),
        Color::from_u32(0x00509552),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x002F2B26), Color::from_u32(0x0035312C)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00D6D7CC);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00C57E00);
}
