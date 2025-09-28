
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Matrix;

impl SixColorsTwoRowsStyler for Matrix {
    const BACKGROUND: Color = Color::from_u32(0x000f191c);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x002F393C);
    const FOREGROUND: Color = Color::from_u32(0x00426644);
    const DARK_FOREGROUND: Color = Color::from_u32(0x000f191c);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x002fc079),
        Color::from_u32(0x0090d762),
        Color::from_u32(0x00faff00),
        Color::from_u32(0x004f7e7e),
        Color::from_u32(0x0011ff25),
        Color::from_u32(0x00c1ff8a),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x0023755a),
        Color::from_u32(0x0082d967),
        Color::from_u32(0x00ffd700),
        Color::from_u32(0x003f5242),
        Color::from_u32(0x00409931),
        Color::from_u32(0x0050b45a),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00151F22), Color::from_u32(0x001B2528)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00182525);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00CFA700);
}
