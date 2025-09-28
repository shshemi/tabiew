
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct CatppuccinFrappe;

impl SixColorsTwoRowsStyler for CatppuccinFrappe {
    const BACKGROUND: Color = Color::from_u32(0x00303446);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00505466);
    const FOREGROUND: Color = Color::from_u32(0x00c6d0f5);
    const DARK_FOREGROUND: Color = Color::from_u32(0x0051576d);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00e67172),
        Color::from_u32(0x008ec772),
        Color::from_u32(0x00d9ba73),
        Color::from_u32(0x007b9ef0),
        Color::from_u32(0x00f2a4db),
        Color::from_u32(0x005abfb5),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00e78284),
        Color::from_u32(0x00a6d189),
        Color::from_u32(0x00e5c890),
        Color::from_u32(0x008caaee),
        Color::from_u32(0x00f4b8e4),
        Color::from_u32(0x0081c8be),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00363A4C), Color::from_u32(0x003C4052)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00D2B5AF);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00C488B4);
}
