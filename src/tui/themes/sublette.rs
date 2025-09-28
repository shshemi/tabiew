
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Sublette;

impl SixColorsTwoRowsStyler for Sublette {
    const BACKGROUND: Color = Color::from_u32(0x00202535);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00404555);
    const FOREGROUND: Color = Color::from_u32(0x00ccced0);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00253045);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00ee6655),
        Color::from_u32(0x0099ee77),
        Color::from_u32(0x00ffff77),
        Color::from_u32(0x0077bbff),
        Color::from_u32(0x00aa88ff),
        Color::from_u32(0x0055ffbb),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00ee5577),
        Color::from_u32(0x0055ee77),
        Color::from_u32(0x00ffdd88),
        Color::from_u32(0x005588ff),
        Color::from_u32(0x00ff77cc),
        Color::from_u32(0x0044eeee),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00262B3B), Color::from_u32(0x002C3141)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00ACAEB0);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00CFAD58);
}
