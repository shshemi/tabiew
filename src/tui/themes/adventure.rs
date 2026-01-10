use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Adventure;

impl SixColorsTwoRowsStyler for Adventure {
    const BACKGROUND: Color = Color::from_u32(0x00040404);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00242424);
    const FOREGROUND: Color = Color::from_u32(0x00feffff);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00040404);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00d76b42),
        Color::from_u32(0x0099b52c),
        Color::from_u32(0x00ffb670),
        Color::from_u32(0x0097d7ef),
        Color::from_u32(0x00aa7900),
        Color::from_u32(0x00bdcfe5),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00d84a33),
        Color::from_u32(0x005da602),
        Color::from_u32(0x00eebb6e),
        Color::from_u32(0x00417ab3),
        Color::from_u32(0x00e5c499),
        Color::from_u32(0x00bdcfe5),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x000A0A0A), Color::from_u32(0x00101010)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00DEDFDF);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00BE8B3E);
}
