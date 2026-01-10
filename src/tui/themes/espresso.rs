use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Espresso;

impl SixColorsTwoRowsStyler for Espresso {
    const BACKGROUND: Color = Color::from_u32(0x00323232);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00525252);
    const FOREGROUND: Color = Color::from_u32(0x00ffffff);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00353535);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00f00c0c),
        Color::from_u32(0x00c2e075),
        Color::from_u32(0x00e1e48b),
        Color::from_u32(0x008ab7d9),
        Color::from_u32(0x00efb5f7),
        Color::from_u32(0x00dcf4ff),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00d25252),
        Color::from_u32(0x00a5c261),
        Color::from_u32(0x00ffc66d),
        Color::from_u32(0x006c99bb),
        Color::from_u32(0x00d197d9),
        Color::from_u32(0x00bed6ff),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00383838), Color::from_u32(0x003E3E3E)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00B6B6B6);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00CF963D);
}
