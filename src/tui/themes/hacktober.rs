use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Hacktober;

impl SixColorsTwoRowsStyler for Hacktober {
    const BACKGROUND: Color = Color::from_u32(0x00141414);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00343434);
    const FOREGROUND: Color = Color::from_u32(0x00c9c9c9);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00191918);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00b33323),
        Color::from_u32(0x0042824a),
        Color::from_u32(0x00c75a22),
        Color::from_u32(0x005389c5),
        Color::from_u32(0x00e795a5),
        Color::from_u32(0x00ebc587),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00b34538),
        Color::from_u32(0x00587744),
        Color::from_u32(0x00d08949),
        Color::from_u32(0x00206ec5),
        Color::from_u32(0x00864651),
        Color::from_u32(0x00ac9166),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x001A1A1A), Color::from_u32(0x00202020)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00A9A9A9);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00A05919);
}
