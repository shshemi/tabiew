
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Hurtado;

impl SixColorsTwoRowsStyler for Hurtado {
    const BACKGROUND: Color = Color::from_u32(0x00000000);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00202020);
    const FOREGROUND: Color = Color::from_u32(0x00dbdbdb);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00575757);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00d51d00),
        Color::from_u32(0x00a5df55),
        Color::from_u32(0x00fbe84a),
        Color::from_u32(0x0089beff),
        Color::from_u32(0x00c001c1),
        Color::from_u32(0x0086eafe),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00ff1b00),
        Color::from_u32(0x00a5e055),
        Color::from_u32(0x00fbe74a),
        Color::from_u32(0x00496487),
        Color::from_u32(0x00fd5ff1),
        Color::from_u32(0x0086e9fe),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00060606), Color::from_u32(0x000C0C0C)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x009B9B9B);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00CF0000);
}
