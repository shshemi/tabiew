use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Zenburn;

impl SixColorsTwoRowsStyler for Zenburn {
    const BACKGROUND: Color = Color::from_u32(0x003f3f3f);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x005F5F5F);
    const FOREGROUND: Color = Color::from_u32(0x00dcdccc);
    const DARK_FOREGROUND: Color = Color::from_u32(0x004d4d4d);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00dca3a3),
        Color::from_u32(0x00c3bf9f),
        Color::from_u32(0x00e0cf9f),
        Color::from_u32(0x0094bff3),
        Color::from_u32(0x00ec93d3),
        Color::from_u32(0x0093e0e3),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00705050),
        Color::from_u32(0x0060b48a),
        Color::from_u32(0x00f0dfaf),
        Color::from_u32(0x00506070),
        Color::from_u32(0x00dc8cc3),
        Color::from_u32(0x008cd0d3),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00454545), Color::from_u32(0x004B4B4B)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x0053433A);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00C0AF7F);
}
