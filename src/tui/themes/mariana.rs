
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Mariana;

impl SixColorsTwoRowsStyler for Mariana {
    const BACKGROUND: Color = Color::from_u32(0x00343d46);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00545D66);
    const FOREGROUND: Color = Color::from_u32(0x00d8dee9);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00000000);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00f97b58),
        Color::from_u32(0x00acd1a8),
        Color::from_u32(0x00fac761),
        Color::from_u32(0x0085add6),
        Color::from_u32(0x00d8b6d8),
        Color::from_u32(0x0082c4c4),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00ec5f66),
        Color::from_u32(0x0099c794),
        Color::from_u32(0x00f9ae58),
        Color::from_u32(0x006699cc),
        Color::from_u32(0x00c695c6),
        Color::from_u32(0x005fb4b4),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x003A434C), Color::from_u32(0x00404952)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00DC9B4A);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00C97E28);
}
