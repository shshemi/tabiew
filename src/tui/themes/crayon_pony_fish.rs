use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct CrayonPonyFish;

impl SixColorsTwoRowsStyler for CrayonPonyFish {
    const BACKGROUND: Color = Color::from_u32(0x00150707);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00352727);
    const FOREGROUND: Color = Color::from_u32(0x0068525a);
    const DARK_FOREGROUND: Color = Color::from_u32(0x002b1b1d);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00c5255d),
        Color::from_u32(0x008dff57),
        Color::from_u32(0x00c8381d),
        Color::from_u32(0x00cfc9ff),
        Color::from_u32(0x00fc6cba),
        Color::from_u32(0x00ffceaf),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x0091002b),
        Color::from_u32(0x00579524),
        Color::from_u32(0x00ab311b),
        Color::from_u32(0x008c87b0),
        Color::from_u32(0x00692f50),
        Color::from_u32(0x00e8a866),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x001B0D0D), Color::from_u32(0x00211313)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x0048323A);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00B87836);
}
