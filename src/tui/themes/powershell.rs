
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Powershell;

impl SixColorsTwoRowsStyler for Powershell {
    const BACKGROUND: Color = Color::from_u32(0x00052454);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00254474);
    const FOREGROUND: Color = Color::from_u32(0x00f6f6f7);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00000000);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00ef2929),
        Color::from_u32(0x001cfe3c),
        Color::from_u32(0x00fefe45),
        Color::from_u32(0x00268ad2),
        Color::from_u32(0x00fe13fa),
        Color::from_u32(0x0029fffe),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x007e0008),
        Color::from_u32(0x00098003),
        Color::from_u32(0x00c4a000),
        Color::from_u32(0x00010083),
        Color::from_u32(0x00d33682),
        Color::from_u32(0x000e807f),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x000B2A5A), Color::from_u32(0x00113060)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00D6D6D7);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00A30652);
}
