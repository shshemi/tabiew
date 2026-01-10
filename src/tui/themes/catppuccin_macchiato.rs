use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct CatppuccinMacchiato;

impl SixColorsTwoRowsStyler for CatppuccinMacchiato {
    const BACKGROUND: Color = Color::from_u32(0x0024273a);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x0044475A);
    const FOREGROUND: Color = Color::from_u32(0x00cad3f5);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00494d64);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00ec7486),
        Color::from_u32(0x008ccf7f),
        Color::from_u32(0x00e1c682),
        Color::from_u32(0x0078a1f6),
        Color::from_u32(0x00f2a9dd),
        Color::from_u32(0x0063cbc0),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00ed8796),
        Color::from_u32(0x00a6da95),
        Color::from_u32(0x00eed49f),
        Color::from_u32(0x008aadf4),
        Color::from_u32(0x00f5bde6),
        Color::from_u32(0x008bd5ca),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x002A2D40), Color::from_u32(0x00303346)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00D4BBB6);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00C58DB6);
}
