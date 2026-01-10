use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct DoomOne;

impl SixColorsTwoRowsStyler for DoomOne {
    const BACKGROUND: Color = Color::from_u32(0x00282c34);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00484C54);
    const FOREGROUND: Color = Color::from_u32(0x00bbc2cf);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00000000);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00ff6655),
        Color::from_u32(0x0099bb66),
        Color::from_u32(0x00ecbe7b),
        Color::from_u32(0x00a9a1e1),
        Color::from_u32(0x00c678dd),
        Color::from_u32(0x0051afef),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00ff6c6b),
        Color::from_u32(0x0098be65),
        Color::from_u32(0x00ecbe7b),
        Color::from_u32(0x00a9a1e1),
        Color::from_u32(0x00c678dd),
        Color::from_u32(0x0051afef),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x002E323A), Color::from_u32(0x00343840)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00318FCF);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00CF3C3B);
}
