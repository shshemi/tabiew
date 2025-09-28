
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Fahrenheit;

impl SixColorsTwoRowsStyler for Fahrenheit {
    const BACKGROUND: Color = Color::from_u32(0x00000000);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00202020);
    const FOREGROUND: Color = Color::from_u32(0x00ffffce);
    const DARK_FOREGROUND: Color = Color::from_u32(0x001d1d1d);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00fecea0),
        Color::from_u32(0x00cc734d),
        Color::from_u32(0x00fd9f4d),
        Color::from_u32(0x00cb4a05),
        Color::from_u32(0x004e739f),
        Color::from_u32(0x00fed04d),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00cda074),
        Color::from_u32(0x009e744d),
        Color::from_u32(0x00fecf75),
        Color::from_u32(0x00720102),
        Color::from_u32(0x00734c4d),
        Color::from_u32(0x00979797),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00060606), Color::from_u32(0x000C0C0C)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x009B9B9B);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00CE9F45);
}
