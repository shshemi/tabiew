
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct JackieBrown;

impl SixColorsTwoRowsStyler for JackieBrown {
    const BACKGROUND: Color = Color::from_u32(0x002c1d16);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x004C3D36);
    const FOREGROUND: Color = Color::from_u32(0x00ffcc2f);
    const DARK_FOREGROUND: Color = Color::from_u32(0x002c1d16);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00e50000),
        Color::from_u32(0x0086a93e),
        Color::from_u32(0x00e5e500),
        Color::from_u32(0x000000ff),
        Color::from_u32(0x00e500e5),
        Color::from_u32(0x0000e5e5),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00ef5734),
        Color::from_u32(0x002baf2b),
        Color::from_u32(0x00bebf00),
        Color::from_u32(0x00246eb2),
        Color::from_u32(0x00d05ec1),
        Color::from_u32(0x0000acee),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x0032231C), Color::from_u32(0x00382922)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x0003DF00);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00BF2704);
}
