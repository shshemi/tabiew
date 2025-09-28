
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Ardoise;

impl SixColorsTwoRowsStyler for Ardoise {
    const BACKGROUND: Color = Color::from_u32(0x001e1e1e);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x003E3E3E);
    const FOREGROUND: Color = Color::from_u32(0x00eaeaea);
    const DARK_FOREGROUND: Color = Color::from_u32(0x002c2c2c);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00fa5852),
        Color::from_u32(0x008dc252),
        Color::from_u32(0x00ffea51),
        Color::from_u32(0x006ab5f8),
        Color::from_u32(0x00be68ca),
        Color::from_u32(0x0089ffdb),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00d3322d),
        Color::from_u32(0x00588b35),
        Color::from_u32(0x00fca93a),
        Color::from_u32(0x002465c2),
        Color::from_u32(0x007332b4),
        Color::from_u32(0x0064e1b8),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00242424), Color::from_u32(0x002A2A2A)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00D7D7D7);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00CC790A);
}
