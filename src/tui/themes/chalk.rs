use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Chalk;

impl SixColorsTwoRowsStyler for Chalk {
    const BACKGROUND: Color = Color::from_u32(0x002b2d2e);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x004B4D4E);
    const FOREGROUND: Color = Color::from_u32(0x00d2d8d9);
    const DARK_FOREGROUND: Color = Color::from_u32(0x007d8b8f);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00f24840),
        Color::from_u32(0x0080c470),
        Color::from_u32(0x00ffeb62),
        Color::from_u32(0x004196ff),
        Color::from_u32(0x00fc5275),
        Color::from_u32(0x0053cdbd),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00b23a52),
        Color::from_u32(0x00789b6a),
        Color::from_u32(0x00b9ac4a),
        Color::from_u32(0x002a7fac),
        Color::from_u32(0x00bd4f5a),
        Color::from_u32(0x0044a799),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00313334), Color::from_u32(0x0037393A)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00506264);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x008D1F2A);
}
