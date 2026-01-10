use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct NightOwl;

impl SixColorsTwoRowsStyler for NightOwl {
    const BACKGROUND: Color = Color::from_u32(0x00011627);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00213647);
    const FOREGROUND: Color = Color::from_u32(0x00d6deeb);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00011627);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00ef5350),
        Color::from_u32(0x0022da6e),
        Color::from_u32(0x00ffeb95),
        Color::from_u32(0x0082aaff),
        Color::from_u32(0x00c792ea),
        Color::from_u32(0x007fdbca),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00ef5350),
        Color::from_u32(0x0022da6e),
        Color::from_u32(0x00addb67),
        Color::from_u32(0x0082aaff),
        Color::from_u32(0x00c792ea),
        Color::from_u32(0x0021c7a8),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00071C2D), Color::from_u32(0x000D2233)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x005E37A2);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00BF2320);
}
