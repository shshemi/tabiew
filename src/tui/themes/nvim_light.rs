use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct NvimLight;

impl SixColorsTwoRowsStyler for NvimLight {
    const BACKGROUND: Color = Color::from_u32(0x00e0e2ea);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00FFFFFF);
    const FOREGROUND: Color = Color::from_u32(0x0014161b);
    const DARK_FOREGROUND: Color = Color::from_u32(0x0007080d);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00590008),
        Color::from_u32(0x00005523),
        Color::from_u32(0x006b5300),
        Color::from_u32(0x00004c73),
        Color::from_u32(0x00470045),
        Color::from_u32(0x00007373),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00590008),
        Color::from_u32(0x00005523),
        Color::from_u32(0x006b5300),
        Color::from_u32(0x00004c73),
        Color::from_u32(0x00470045),
        Color::from_u32(0x00007373),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00E6E8F0), Color::from_u32(0x00ECEEF6)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x007B7E84);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x003B2300);
}
