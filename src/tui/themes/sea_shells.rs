
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct SeaShells;

impl SixColorsTwoRowsStyler for SeaShells {
    const BACKGROUND: Color = Color::from_u32(0x0009141b);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x0029343B);
    const FOREGROUND: Color = Color::from_u32(0x00deb88d);
    const DARK_FOREGROUND: Color = Color::from_u32(0x0017384c);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00d48678),
        Color::from_u32(0x00628d98),
        Color::from_u32(0x00fdd39f),
        Color::from_u32(0x001bbcdd),
        Color::from_u32(0x00bbe3ee),
        Color::from_u32(0x0087acb4),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00d15123),
        Color::from_u32(0x00027c9b),
        Color::from_u32(0x00fca02f),
        Color::from_u32(0x001e4950),
        Color::from_u32(0x0068d4f1),
        Color::from_u32(0x0050a3b5),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x000F1A21), Color::from_u32(0x00152027)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00DC800F);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00CC7000);
}
