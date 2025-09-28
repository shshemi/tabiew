
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Medallion;

impl SixColorsTwoRowsStyler for Medallion {
    const BACKGROUND: Color = Color::from_u32(0x001d1908);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x003D3928);
    const FOREGROUND: Color = Color::from_u32(0x00cac296);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00000000);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00ff9149),
        Color::from_u32(0x00b2ca3b),
        Color::from_u32(0x00ffe54a),
        Color::from_u32(0x00acb8ff),
        Color::from_u32(0x00ffa0ff),
        Color::from_u32(0x00ffbc51),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00b64c00),
        Color::from_u32(0x007c8b16),
        Color::from_u32(0x00d3bd26),
        Color::from_u32(0x00616bb0),
        Color::from_u32(0x008c5a90),
        Color::from_u32(0x00916c25),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00231F0E), Color::from_u32(0x00292514)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00B39A10);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00A38D00);
}
