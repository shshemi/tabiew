use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Pro;

impl SixColorsTwoRowsStyler for Pro {
    const BACKGROUND: Color = Color::from_u32(0x00000000);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00202020);
    const FOREGROUND: Color = Color::from_u32(0x00f2f2f2);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00000000);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00e50000),
        Color::from_u32(0x0000d900),
        Color::from_u32(0x00e5e500),
        Color::from_u32(0x000000ff),
        Color::from_u32(0x00e500e5),
        Color::from_u32(0x0000e5e5),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00990000),
        Color::from_u32(0x0000a600),
        Color::from_u32(0x00999900),
        Color::from_u32(0x002009db),
        Color::from_u32(0x00b200b2),
        Color::from_u32(0x0000a6b2),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00060606), Color::from_u32(0x000C0C0C)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x002D2D2D);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00820082);
}
