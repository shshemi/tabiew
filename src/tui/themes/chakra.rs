use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Chakra;

impl SixColorsTwoRowsStyler for Chakra {
    const BACKGROUND: Color = Color::from_u32(0x00111111);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00191917);
    const FOREGROUND: Color = Color::from_u32(0x00fafafa);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00a1a1aa);
    const COLORS: [Color; 6] = [
        Color::from_u32(0x00f472b6),
        Color::from_u32(0x00c084fc),
        Color::from_u32(0x0022d3ee),
        Color::from_u32(0x0060a5fa),
        Color::from_u32(0x002dd4bf),
        Color::from_u32(0x004ade80),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00ec4899),
        Color::from_u32(0x00a855f7),
        Color::from_u32(0x0006b6d4),
        Color::from_u32(0x003b82f6),
        Color::from_u32(0x0014b8a6),
        Color::from_u32(0x0022c55e),
    ];
    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x0018181b), Color::from_u32(0x00111111)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00ca8a04);
    const HIGHLIGHT_FOREGROUND: Color = Color::from_u32(0x00fafafa);
    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00991919);
}
