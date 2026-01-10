use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct PaulMillr;

impl SixColorsTwoRowsStyler for PaulMillr {
    const BACKGROUND: Color = Color::from_u32(0x00000000);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00202020);
    const FOREGROUND: Color = Color::from_u32(0x00f2f2f2);
    const DARK_FOREGROUND: Color = Color::from_u32(0x002a2a2a);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00ff0080),
        Color::from_u32(0x0066ff66),
        Color::from_u32(0x00f3d64e),
        Color::from_u32(0x00709aed),
        Color::from_u32(0x00db67e6),
        Color::from_u32(0x007adff2),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00ff0000),
        Color::from_u32(0x0079ff0f),
        Color::from_u32(0x00e7bf00),
        Color::from_u32(0x00396bd7),
        Color::from_u32(0x00b449be),
        Color::from_u32(0x0066ccff),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00060606), Color::from_u32(0x000C0C0C)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x002D2D2D);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00CF0000);
}
