
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct ParaisoDark;

impl SixColorsTwoRowsStyler for ParaisoDark {
    const BACKGROUND: Color = Color::from_u32(0x002f1e2e);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x004F3E4E);
    const FOREGROUND: Color = Color::from_u32(0x00a39e9b);
    const DARK_FOREGROUND: Color = Color::from_u32(0x002f1e2e);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00ef6155),
        Color::from_u32(0x0048b685),
        Color::from_u32(0x00fec418),
        Color::from_u32(0x0006b6ef),
        Color::from_u32(0x00815ba4),
        Color::from_u32(0x005bc4bf),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00ef6155),
        Color::from_u32(0x0048b685),
        Color::from_u32(0x00fec418),
        Color::from_u32(0x0006b6ef),
        Color::from_u32(0x00815ba4),
        Color::from_u32(0x005bc4bf),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00352434), Color::from_u32(0x003B2A3A)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00837E7B);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00CE9400);
}
