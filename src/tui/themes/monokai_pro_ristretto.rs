use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct MonokaiProRistretto;

impl SixColorsTwoRowsStyler for MonokaiProRistretto {
    const BACKGROUND: Color = Color::from_u32(0x002c2525);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x004C4545);
    const FOREGROUND: Color = Color::from_u32(0x00fff1f3);
    const DARK_FOREGROUND: Color = Color::from_u32(0x002c2525);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00fd6883),
        Color::from_u32(0x00adda78),
        Color::from_u32(0x00f9cc6c),
        Color::from_u32(0x00f38d70),
        Color::from_u32(0x00a8a9eb),
        Color::from_u32(0x0085dacc),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00fd6883),
        Color::from_u32(0x00adda78),
        Color::from_u32(0x00f9cc6c),
        Color::from_u32(0x00f38d70),
        Color::from_u32(0x00a8a9eb),
        Color::from_u32(0x0085dacc),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00322B2B), Color::from_u32(0x00383131)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00A39798);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00CD3853);
}
