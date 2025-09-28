
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct WarmNeon;

impl SixColorsTwoRowsStyler for WarmNeon {
    const BACKGROUND: Color = Color::from_u32(0x00404040);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00606060);
    const FOREGROUND: Color = Color::from_u32(0x00afdab6);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00000000);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00e97071),
        Color::from_u32(0x009cc090),
        Color::from_u32(0x00ddda7a),
        Color::from_u32(0x007b91d6),
        Color::from_u32(0x00f674ba),
        Color::from_u32(0x005ed1e5),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00e24346),
        Color::from_u32(0x0039b13a),
        Color::from_u32(0x00dae145),
        Color::from_u32(0x004261c5),
        Color::from_u32(0x00f920fb),
        Color::from_u32(0x002abbd4),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00464646), Color::from_u32(0x004C4C4C)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x0010DF04);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00C900CB);
}
