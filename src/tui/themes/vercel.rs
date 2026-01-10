use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Vercel;

impl SixColorsTwoRowsStyler for Vercel {
    const BACKGROUND: Color = Color::from_u32(0x00101010);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00303030);
    const FOREGROUND: Color = Color::from_u32(0x00fafafa);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00000000);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00ff8080),
        Color::from_u32(0x004be15d),
        Color::from_u32(0x00ffae00),
        Color::from_u32(0x0049aeff),
        Color::from_u32(0x00f97ea8),
        Color::from_u32(0x0000e4c4),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00fc0036),
        Color::from_u32(0x0029a948),
        Color::from_u32(0x00ffae00),
        Color::from_u32(0x00006aff),
        Color::from_u32(0x00f32882),
        Color::from_u32(0x0000ac96),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00161616), Color::from_u32(0x001C1C1C)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00D30862);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00CF7E00);
}
