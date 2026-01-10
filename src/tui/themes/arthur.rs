use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Arthur;

impl SixColorsTwoRowsStyler for Arthur {
    const BACKGROUND: Color = Color::from_u32(0x001c1c1c);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x003C3C3C);
    const FOREGROUND: Color = Color::from_u32(0x00ddeedd);
    const DARK_FOREGROUND: Color = Color::from_u32(0x003d352a);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00cc5533),
        Color::from_u32(0x0088aa22),
        Color::from_u32(0x00ffa75d),
        Color::from_u32(0x0087ceeb),
        Color::from_u32(0x00996600),
        Color::from_u32(0x00b0c4de),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00cd5c5c),
        Color::from_u32(0x0086af80),
        Color::from_u32(0x00e8ae5b),
        Color::from_u32(0x006495ed),
        Color::from_u32(0x00deb887),
        Color::from_u32(0x00b0c4de),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00222222), Color::from_u32(0x00282828)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00C29BCF);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00B87E2B);
}
