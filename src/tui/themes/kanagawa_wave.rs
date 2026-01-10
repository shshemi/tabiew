use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct KanagawaWave;

impl SixColorsTwoRowsStyler for KanagawaWave {
    const BACKGROUND: Color = Color::from_u32(0x001f1f28);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x003F3F48);
    const FOREGROUND: Color = Color::from_u32(0x00dcd7ba);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00090618);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00e82424),
        Color::from_u32(0x0098bb6c),
        Color::from_u32(0x00e6c384),
        Color::from_u32(0x007fb4ca),
        Color::from_u32(0x00938aa9),
        Color::from_u32(0x007aa89f),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00c34043),
        Color::from_u32(0x0076946a),
        Color::from_u32(0x00c0a36e),
        Color::from_u32(0x007e9cd8),
        Color::from_u32(0x00957fb8),
        Color::from_u32(0x006a9589),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x0025252E), Color::from_u32(0x002B2B34)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00A8A073);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00931013);
}
