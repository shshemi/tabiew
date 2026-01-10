use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Chester;

impl SixColorsTwoRowsStyler for Chester {
    const BACKGROUND: Color = Color::from_u32(0x002c3643);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x004C5663);
    const FOREGROUND: Color = Color::from_u32(0x00ffffff);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00080200);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00fa5e5b),
        Color::from_u32(0x0016c98d),
        Color::from_u32(0x00feef6d),
        Color::from_u32(0x00278ad6),
        Color::from_u32(0x00d34590),
        Color::from_u32(0x0027dede),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00fa5e5b),
        Color::from_u32(0x0016c98d),
        Color::from_u32(0x00ffc83f),
        Color::from_u32(0x00288ad6),
        Color::from_u32(0x00d34590),
        Color::from_u32(0x0028ddde),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00323C49), Color::from_u32(0x0038424F)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00949191);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00CF980F);
}
