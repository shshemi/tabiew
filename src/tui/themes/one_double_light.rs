
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct OneDoubleLight;

impl SixColorsTwoRowsStyler for OneDoubleLight {
    const BACKGROUND: Color = Color::from_u32(0x00fafafa);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00FFFFFF);
    const FOREGROUND: Color = Color::from_u32(0x00383a43);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00454b58);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00ff3711),
        Color::from_u32(0x0000b90e),
        Color::from_u32(0x00ec9900),
        Color::from_u32(0x001065de),
        Color::from_u32(0x00e500d8),
        Color::from_u32(0x0000b4dd),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00f74840),
        Color::from_u32(0x0025a343),
        Color::from_u32(0x00cc8100),
        Color::from_u32(0x000087c1),
        Color::from_u32(0x00b50da9),
        Color::from_u32(0x00009ab7),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00FFFFFF), Color::from_u32(0x00FFFFFF)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00000000);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00C71810);
}
