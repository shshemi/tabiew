use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Jellybeans;

impl SixColorsTwoRowsStyler for Jellybeans {
    const BACKGROUND: Color = Color::from_u32(0x00121212);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00323232);
    const FOREGROUND: Color = Color::from_u32(0x00dedede);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00929292);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00ffa1a1),
        Color::from_u32(0x00bddeab),
        Color::from_u32(0x00ffdca0),
        Color::from_u32(0x00b1d8f6),
        Color::from_u32(0x00fbdaff),
        Color::from_u32(0x001ab2a8),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00e27373),
        Color::from_u32(0x0094b979),
        Color::from_u32(0x00ffba7b),
        Color::from_u32(0x0097bedc),
        Color::from_u32(0x00e1c0fa),
        Color::from_u32(0x0000988e),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00181818), Color::from_u32(0x001E1E1E)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00DF8540);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00CF8A4B);
}
