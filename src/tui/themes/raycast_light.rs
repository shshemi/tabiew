
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct RaycastLight;

impl SixColorsTwoRowsStyler for RaycastLight {
    const BACKGROUND: Color = Color::from_u32(0x00ffffff);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00FFFFFF);
    const FOREGROUND: Color = Color::from_u32(0x00000000);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00000000);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00b12424),
        Color::from_u32(0x00006b4f),
        Color::from_u32(0x00f8a300),
        Color::from_u32(0x00138af2),
        Color::from_u32(0x009a1b6e),
        Color::from_u32(0x003eb8bf),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00b12424),
        Color::from_u32(0x00006b4f),
        Color::from_u32(0x00f8a300),
        Color::from_u32(0x00138af2),
        Color::from_u32(0x009a1b6e),
        Color::from_u32(0x003eb8bf),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00FFFFFF), Color::from_u32(0x00FFFFFF)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00000000);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00C87300);
}
