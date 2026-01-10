use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct ElectronHighlighter;

impl SixColorsTwoRowsStyler for ElectronHighlighter {
    const BACKGROUND: Color = Color::from_u32(0x0023283d);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x0043485D);
    const FOREGROUND: Color = Color::from_u32(0x00a5b6d4);
    const DARK_FOREGROUND: Color = Color::from_u32(0x0015161f);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00ff6c8d),
        Color::from_u32(0x0000ffc3),
        Color::from_u32(0x00ffd7a9),
        Color::from_u32(0x0077abff),
        Color::from_u32(0x00daa4f4),
        Color::from_u32(0x0000fdff),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00ff6c8d),
        Color::from_u32(0x0000ffc3),
        Color::from_u32(0x00ffd7a9),
        Color::from_u32(0x0077abff),
        Color::from_u32(0x00daa4f4),
        Color::from_u32(0x0000fdff),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00292E43), Color::from_u32(0x002F3449)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x008596B4);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00CF3C5D);
}
