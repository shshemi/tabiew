use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Grass;

impl SixColorsTwoRowsStyler for Grass {
    const BACKGROUND: Color = Color::from_u32(0x0013773d);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x0033975D);
    const FOREGROUND: Color = Color::from_u32(0x00fff0a5);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00000000);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00bb0000),
        Color::from_u32(0x0000bb00),
        Color::from_u32(0x00e7b000),
        Color::from_u32(0x000000bb),
        Color::from_u32(0x00ff55ff),
        Color::from_u32(0x0055ffff),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00bb0000),
        Color::from_u32(0x0000bb00),
        Color::from_u32(0x00e7b000),
        Color::from_u32(0x000000a3),
        Color::from_u32(0x00950062),
        Color::from_u32(0x0000bbbb),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00197D43), Color::from_u32(0x001F8349)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x006C0800);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00B78000);
}
