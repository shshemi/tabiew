
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Rapture;

impl SixColorsTwoRowsStyler for Rapture {
    const BACKGROUND: Color = Color::from_u32(0x00111e2a);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00313E4A);
    const FOREGROUND: Color = Color::from_u32(0x00c0c9e5);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00000000);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00fc644d),
        Color::from_u32(0x007afde1),
        Color::from_u32(0x00fff09b),
        Color::from_u32(0x006c9bf5),
        Color::from_u32(0x00ff4fa1),
        Color::from_u32(0x0064e0ff),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00fc644d),
        Color::from_u32(0x007afde1),
        Color::from_u32(0x00fff09b),
        Color::from_u32(0x006c9bf5),
        Color::from_u32(0x00ff4fa1),
        Color::from_u32(0x0064e0ff),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00172430), Color::from_u32(0x001D2A36)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00DFDFDF);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00CFC06B);
}
