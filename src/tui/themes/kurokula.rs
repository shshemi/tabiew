
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Kurokula;

impl SixColorsTwoRowsStyler for Kurokula {
    const BACKGROUND: Color = Color::from_u32(0x00141515);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00343535);
    const FOREGROUND: Color = Color::from_u32(0x00e0cfc2);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00333333);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00ffc34c),
        Color::from_u32(0x00afffa5),
        Color::from_u32(0x00fff700),
        Color::from_u32(0x0090dbff),
        Color::from_u32(0x00ad93ff),
        Color::from_u32(0x00ffcdb6),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00c35a52),
        Color::from_u32(0x0078b3a9),
        Color::from_u32(0x00e1b917),
        Color::from_u32(0x005c91dd),
        Color::from_u32(0x008b79a6),
        Color::from_u32(0x00867268),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x001A1B1B), Color::from_u32(0x00202121)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x005A0000);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00B18900);
}
