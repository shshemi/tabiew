
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Urple;

impl SixColorsTwoRowsStyler for Urple {
    const BACKGROUND: Color = Color::from_u32(0x001b1b23);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x003B3B43);
    const FOREGROUND: Color = Color::from_u32(0x00877a9b);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00000000);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00ff6388),
        Color::from_u32(0x0029e620),
        Color::from_u32(0x00f08161),
        Color::from_u32(0x00867aed),
        Color::from_u32(0x00a05eee),
        Color::from_u32(0x00eaeaea),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00b0425b),
        Color::from_u32(0x0037a415),
        Color::from_u32(0x00ad5c42),
        Color::from_u32(0x00564d9b),
        Color::from_u32(0x006c3ca1),
        Color::from_u32(0x00808080),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00212129), Color::from_u32(0x0027272F)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x008043CB);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x0080122B);
}
