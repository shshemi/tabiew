use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct LiquidCarbonTransparent;

impl SixColorsTwoRowsStyler for LiquidCarbonTransparent {
    const BACKGROUND: Color = Color::from_u32(0x00000000);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00202020);
    const FOREGROUND: Color = Color::from_u32(0x00afc2c2);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00000000);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00ff3030),
        Color::from_u32(0x00559a70),
        Color::from_u32(0x00ccac00),
        Color::from_u32(0x000099cc),
        Color::from_u32(0x00cc69c8),
        Color::from_u32(0x007ac4cc),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00ff3030),
        Color::from_u32(0x00559a70),
        Color::from_u32(0x00ccac00),
        Color::from_u32(0x000099cc),
        Color::from_u32(0x00cc69c8),
        Color::from_u32(0x007ac4cc),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00060606), Color::from_u32(0x000C0C0C)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00DFDFDF);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00CF0000);
}
