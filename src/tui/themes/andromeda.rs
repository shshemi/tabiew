
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Andromeda;

impl SixColorsTwoRowsStyler for Andromeda {
    const BACKGROUND: Color = Color::from_u32(0x00262a33);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00464A53);
    const FOREGROUND: Color = Color::from_u32(0x00e5e5e5);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00000000);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00cd3131),
        Color::from_u32(0x0005bc79),
        Color::from_u32(0x00e5e512),
        Color::from_u32(0x002472c8),
        Color::from_u32(0x00bc3fbc),
        Color::from_u32(0x000fa8cd),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00cd3131),
        Color::from_u32(0x0005bc79),
        Color::from_u32(0x00e5e512),
        Color::from_u32(0x002472c8),
        Color::from_u32(0x00bc3fbc),
        Color::from_u32(0x000fa8cd),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x002C3039), Color::from_u32(0x0032363F)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00D8D8D0);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00B5B500);
}
