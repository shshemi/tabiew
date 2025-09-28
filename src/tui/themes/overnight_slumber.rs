
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct OvernightSlumber;

impl SixColorsTwoRowsStyler for OvernightSlumber {
    const BACKGROUND: Color = Color::from_u32(0x000e1729);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x002E3749);
    const FOREGROUND: Color = Color::from_u32(0x00ced2d6);
    const DARK_FOREGROUND: Color = Color::from_u32(0x000a1222);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00ffa7c4),
        Color::from_u32(0x0085cc95),
        Color::from_u32(0x00ffcb8b),
        Color::from_u32(0x008dabe1),
        Color::from_u32(0x00c792eb),
        Color::from_u32(0x00ffa7c4),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00ffa7c4),
        Color::from_u32(0x0085cc95),
        Color::from_u32(0x00ffcb8b),
        Color::from_u32(0x008dabe1),
        Color::from_u32(0x00c792eb),
        Color::from_u32(0x0078ccf0),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00141D2F), Color::from_u32(0x001A2335)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00DF87A4);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00CF9B5B);
}
