
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct MonokaiProMachine;

impl SixColorsTwoRowsStyler for MonokaiProMachine {
    const BACKGROUND: Color = Color::from_u32(0x00273136);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00475156);
    const FOREGROUND: Color = Color::from_u32(0x00f2fffc);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00273136);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00ff6d7e),
        Color::from_u32(0x00a2e57b),
        Color::from_u32(0x00ffed72),
        Color::from_u32(0x00ffb270),
        Color::from_u32(0x00baa0f8),
        Color::from_u32(0x007cd5f1),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00ff6d7e),
        Color::from_u32(0x00a2e57b),
        Color::from_u32(0x00ffed72),
        Color::from_u32(0x00ffb270),
        Color::from_u32(0x00baa0f8),
        Color::from_u32(0x007cd5f1),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x002D373C), Color::from_u32(0x00333D42)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x0098A4A3);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00CF8240);
}
