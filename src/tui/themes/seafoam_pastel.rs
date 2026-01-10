use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct SeafoamPastel;

impl SixColorsTwoRowsStyler for SeafoamPastel {
    const BACKGROUND: Color = Color::from_u32(0x00243435);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00445455);
    const FOREGROUND: Color = Color::from_u32(0x00d4e7d4);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00757575);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00cf937a),
        Color::from_u32(0x0098d9aa),
        Color::from_u32(0x00fae79d),
        Color::from_u32(0x007ac3cf),
        Color::from_u32(0x00d6b2a1),
        Color::from_u32(0x00ade0e0),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00825d4d),
        Color::from_u32(0x00728c62),
        Color::from_u32(0x00ada16d),
        Color::from_u32(0x004d7b82),
        Color::from_u32(0x008a7267),
        Color::from_u32(0x00729494),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x002A3A3B), Color::from_u32(0x00304041)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x0037445A);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x007D713D);
}
