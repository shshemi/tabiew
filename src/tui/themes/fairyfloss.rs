use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Fairyfloss;

impl SixColorsTwoRowsStyler for Fairyfloss {
    const BACKGROUND: Color = Color::from_u32(0x005a5475);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x007A7495);
    const FOREGROUND: Color = Color::from_u32(0x00f8f8f2);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00040303);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00ff857f),
        Color::from_u32(0x00c2ffdf),
        Color::from_u32(0x00ffea00),
        Color::from_u32(0x00c2ffdf),
        Color::from_u32(0x00ffb8d1),
        Color::from_u32(0x00c5a3ff),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00f92672),
        Color::from_u32(0x00c2ffdf),
        Color::from_u32(0x00e6c000),
        Color::from_u32(0x00c2ffdf),
        Color::from_u32(0x00ffb8d1),
        Color::from_u32(0x00c5a3ff),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00605A7B), Color::from_u32(0x00666081)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00D8D8D0);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00CF88A1);
}
