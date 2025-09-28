
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct SeoulbonesDark;

impl SixColorsTwoRowsStyler for SeoulbonesDark {
    const BACKGROUND: Color = Color::from_u32(0x004b4b4b);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x006B6B6B);
    const FOREGROUND: Color = Color::from_u32(0x00dddddd);
    const DARK_FOREGROUND: Color = Color::from_u32(0x004b4b4b);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00eb99b1),
        Color::from_u32(0x008fcd92),
        Color::from_u32(0x00ffe5b3),
        Color::from_u32(0x00a2c8e9),
        Color::from_u32(0x00b2b3da),
        Color::from_u32(0x006bcacb),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00e388a3),
        Color::from_u32(0x0098bd99),
        Color::from_u32(0x00ffdf9b),
        Color::from_u32(0x0097bdde),
        Color::from_u32(0x00a5a6c5),
        Color::from_u32(0x006fbdbe),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00515151), Color::from_u32(0x00575757)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00C2C2C2);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00CFAF6B);
}
