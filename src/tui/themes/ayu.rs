
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Ayu;

impl SixColorsTwoRowsStyler for Ayu {
    const BACKGROUND: Color = Color::from_u32(0x000f1419);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x002F3439);
    const FOREGROUND: Color = Color::from_u32(0x00e6e1cf);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00000000);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00ff6565),
        Color::from_u32(0x00eafe84),
        Color::from_u32(0x00fff779),
        Color::from_u32(0x0068d5ff),
        Color::from_u32(0x00ffa3aa),
        Color::from_u32(0x00c7fffd),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00ff3333),
        Color::from_u32(0x00b8cc52),
        Color::from_u32(0x00e7c547),
        Color::from_u32(0x0036a3d9),
        Color::from_u32(0x00f07178),
        Color::from_u32(0x0095e6cb),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00151A1F), Color::from_u32(0x001B2025)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00D27700);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00CF0303);
}
