
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Jubi;

impl SixColorsTwoRowsStyler for Jubi {
    const BACKGROUND: Color = Color::from_u32(0x00262b33);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00464B53);
    const FOREGROUND: Color = Color::from_u32(0x00c3d3de);
    const DARK_FOREGROUND: Color = Color::from_u32(0x003b3750);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00de90ab),
        Color::from_u32(0x00bcdd61),
        Color::from_u32(0x0087e9ea),
        Color::from_u32(0x008c9fcd),
        Color::from_u32(0x00e16c87),
        Color::from_u32(0x00b7c9ef),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00cf7b98),
        Color::from_u32(0x0090a94b),
        Color::from_u32(0x006ebfc0),
        Color::from_u32(0x00576ea6),
        Color::from_u32(0x00bc4f68),
        Color::from_u32(0x0075a7d2),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x002C3139), Color::from_u32(0x0032373F)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00A3B3BE);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x009F4B68);
}
