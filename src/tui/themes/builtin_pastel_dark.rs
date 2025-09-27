
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct BuiltinPastelDark;

impl SixColorsTwoRowsStyler for BuiltinPastelDark {
    const BACKGROUND: Color = Color::from_u32(0x00000000);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00202020);
    const FOREGROUND: Color = Color::from_u32(0x00bbbbbb);
    const DARK_FOREGROUND: Color = Color::from_u32(0x004f4f4f);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00ffb6b0),
        Color::from_u32(0x00ceffac),
        Color::from_u32(0x00ffffcc),
        Color::from_u32(0x00b5dcff),
        Color::from_u32(0x00ff9cfe),
        Color::from_u32(0x00dfdffe),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00ff6c60),
        Color::from_u32(0x00a8ff60),
        Color::from_u32(0x00ffffb6),
        Color::from_u32(0x0096cbfe),
        Color::from_u32(0x00ff73fd),
        Color::from_u32(0x00c6c5fe),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00060606), Color::from_u32(0x000C0C0C)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00DF8540);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00CF3C30);

    fn id(&self) -> &str {
        "builtin_pastel_dark"
    }

    fn title(&self) -> &str {
        "BuiltinPastelDark"
    }
}
