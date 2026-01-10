use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct KanagawaDragon;

impl SixColorsTwoRowsStyler for KanagawaDragon {
    const BACKGROUND: Color = Color::from_u32(0x00181616);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00383636);
    const FOREGROUND: Color = Color::from_u32(0x00c8c093);
    const DARK_FOREGROUND: Color = Color::from_u32(0x000d0c0c);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00e46876),
        Color::from_u32(0x0087a987),
        Color::from_u32(0x00e6c384),
        Color::from_u32(0x007fb4ca),
        Color::from_u32(0x00938aa9),
        Color::from_u32(0x007aa89f),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00c4746e),
        Color::from_u32(0x008a9a7b),
        Color::from_u32(0x00c4b28a),
        Color::from_u32(0x008ba4b0),
        Color::from_u32(0x00a292a3),
        Color::from_u32(0x008ea4a2),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x001E1C1C), Color::from_u32(0x00242222)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00A5A9A5);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x0094443E);
}
