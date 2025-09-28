
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct OneDoubleDark;

impl SixColorsTwoRowsStyler for OneDoubleDark {
    const BACKGROUND: Color = Color::from_u32(0x00282c34);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00484C54);
    const FOREGROUND: Color = Color::from_u32(0x00dbdfe5);
    const DARK_FOREGROUND: Color = Color::from_u32(0x003d4452);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00ff777b),
        Color::from_u32(0x0082d882),
        Color::from_u32(0x00f5c065),
        Color::from_u32(0x006dcaff),
        Color::from_u32(0x00ff7bf4),
        Color::from_u32(0x0000e5fb),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00f16372),
        Color::from_u32(0x008cc570),
        Color::from_u32(0x00ecbe70),
        Color::from_u32(0x003fb1f5),
        Color::from_u32(0x00d373e3),
        Color::from_u32(0x0017b9c4),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x002E323A), Color::from_u32(0x00343840)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00D5C0BC);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00C13342);
}
