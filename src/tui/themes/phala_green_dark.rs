
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct PhalaGreenDark;

impl SixColorsTwoRowsStyler for PhalaGreenDark {
    const BACKGROUND: Color = Color::from_u32(0x00000000);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00202020);
    const FOREGROUND: Color = Color::from_u32(0x00c1fc03);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00000000);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00ed2200),
        Color::from_u32(0x0000db00),
        Color::from_u32(0x00eae700),
        Color::from_u32(0x000433ff),
        Color::from_u32(0x00ed3aea),
        Color::from_u32(0x0000e8ea),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00ab1500),
        Color::from_u32(0x0000b100),
        Color::from_u32(0x00a9a700),
        Color::from_u32(0x000223c0),
        Color::from_u32(0x00c22ec0),
        Color::from_u32(0x0000b4c0),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00060606), Color::from_u32(0x000C0C0C)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00A1DC00);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00920090);

    fn id(&self) -> &str {
        "phala_green_dark"
    }

    fn title(&self) -> &str {
        "PhalaGreenDark"
    }
}
