use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Vaughn;

impl SixColorsTwoRowsStyler for Vaughn {
    const BACKGROUND: Color = Color::from_u32(0x0025234f);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x0045436F);
    const FOREGROUND: Color = Color::from_u32(0x00dcdccc);
    const DARK_FOREGROUND: Color = Color::from_u32(0x0025234f);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00dca3a3),
        Color::from_u32(0x0060b48a),
        Color::from_u32(0x00f0dfaf),
        Color::from_u32(0x005555ff),
        Color::from_u32(0x00ec93d3),
        Color::from_u32(0x0093e0e3),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00705050),
        Color::from_u32(0x0060b48a),
        Color::from_u32(0x00dfaf8f),
        Color::from_u32(0x005555ff),
        Color::from_u32(0x00f08cc3),
        Color::from_u32(0x008cd0d3),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x002B2955), Color::from_u32(0x00312F5B)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00DF3535);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00C05C93);
}
