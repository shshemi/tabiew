
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct MonokaiProLight;

impl SixColorsTwoRowsStyler for MonokaiProLight {
    const BACKGROUND: Color = Color::from_u32(0x00faf4f2);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00FFFFFF);
    const FOREGROUND: Color = Color::from_u32(0x0029242a);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00faf4f2);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00e14775),
        Color::from_u32(0x00269d69),
        Color::from_u32(0x00cc7a0a),
        Color::from_u32(0x00e16032),
        Color::from_u32(0x007058be),
        Color::from_u32(0x001c8ca8),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00e14775),
        Color::from_u32(0x00269d69),
        Color::from_u32(0x00cc7a0a),
        Color::from_u32(0x00e16032),
        Color::from_u32(0x007058be),
        Color::from_u32(0x001c8ca8),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00FFFAF8), Color::from_u32(0x00FFFFFE)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00504B4E);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00B13002);
}
