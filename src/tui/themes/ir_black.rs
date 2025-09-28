
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct IrBlack;

impl SixColorsTwoRowsStyler for IrBlack {
    const BACKGROUND: Color = Color::from_u32(0x00000000);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00202020);
    const FOREGROUND: Color = Color::from_u32(0x00f1f1f1);
    const DARK_FOREGROUND: Color = Color::from_u32(0x004f4f4f);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00fcb6b0),
        Color::from_u32(0x00cfffab),
        Color::from_u32(0x00ffffcc),
        Color::from_u32(0x00b5dcff),
        Color::from_u32(0x00fb9cfe),
        Color::from_u32(0x00e0e0fe),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00fa6c60),
        Color::from_u32(0x00a8ff60),
        Color::from_u32(0x00fffeb7),
        Color::from_u32(0x0096cafe),
        Color::from_u32(0x00fa73fd),
        Color::from_u32(0x00c6c5fe),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00060606), Color::from_u32(0x000C0C0C)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00606060);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00CFCE87);
}
