
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct BlackMetalDarkFuneral;

impl SixColorsTwoRowsStyler for BlackMetalDarkFuneral {
    const BACKGROUND: Color = Color::from_u32(0x00000000);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00202020);
    const FOREGROUND: Color = Color::from_u32(0x00c1c1c1);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00000000);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x005f8787),
        Color::from_u32(0x00d0dfee),
        Color::from_u32(0x005f81a5),
        Color::from_u32(0x00888888),
        Color::from_u32(0x00999999),
        Color::from_u32(0x00aaaaaa),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x005f8787),
        Color::from_u32(0x00d0dfee),
        Color::from_u32(0x005f81a5),
        Color::from_u32(0x00888888),
        Color::from_u32(0x00999999),
        Color::from_u32(0x00aaaaaa),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00060606), Color::from_u32(0x000C0C0C)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00A1A1A1);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00A0AFBE);
}
