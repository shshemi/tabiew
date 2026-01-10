use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Spring;

impl SixColorsTwoRowsStyler for Spring {
    const BACKGROUND: Color = Color::from_u32(0x00ffffff);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00FFFFFF);
    const FOREGROUND: Color = Color::from_u32(0x004d4d4c);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00000000);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00ff0021),
        Color::from_u32(0x001fc231),
        Color::from_u32(0x00d5b807),
        Color::from_u32(0x0015a9fd),
        Color::from_u32(0x008959a8),
        Color::from_u32(0x003e999f),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00ff4d83),
        Color::from_u32(0x001f8c3b),
        Color::from_u32(0x001fc95b),
        Color::from_u32(0x001dd3ee),
        Color::from_u32(0x008959a8),
        Color::from_u32(0x003e999f),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00FFFFFF), Color::from_u32(0x00FFFFFF)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x002D2D2C);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00CF1D53);
}
