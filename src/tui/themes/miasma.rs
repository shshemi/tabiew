
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Miasma;

impl SixColorsTwoRowsStyler for Miasma {
    const BACKGROUND: Color = Color::from_u32(0x00222222);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00424242);
    const FOREGROUND: Color = Color::from_u32(0x00c2c2b0);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00000000);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00685742),
        Color::from_u32(0x005f875f),
        Color::from_u32(0x00b36d43),
        Color::from_u32(0x0078824b),
        Color::from_u32(0x00bb7744),
        Color::from_u32(0x00c9a554),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00685742),
        Color::from_u32(0x005f875f),
        Color::from_u32(0x00b36d43),
        Color::from_u32(0x0078824b),
        Color::from_u32(0x00bb7744),
        Color::from_u32(0x00c9a554),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00282828), Color::from_u32(0x002E2E2E)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00A7A7A7);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00997524);
}
