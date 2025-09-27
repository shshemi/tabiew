
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct AyuLight;

impl SixColorsTwoRowsStyler for AyuLight {
    const BACKGROUND: Color = Color::from_u32(0x00fafafa);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00FFFFFF);
    const FOREGROUND: Color = Color::from_u32(0x005c6773);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00000000);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00ff6565),
        Color::from_u32(0x00b8e532),
        Color::from_u32(0x00ffc94a),
        Color::from_u32(0x0073d8ff),
        Color::from_u32(0x00ffa3aa),
        Color::from_u32(0x007ff1cb),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00ff3333),
        Color::from_u32(0x0086b300),
        Color::from_u32(0x00f29718),
        Color::from_u32(0x0041a6d9),
        Color::from_u32(0x00f07178),
        Color::from_u32(0x004dbf99),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00FFFFFF), Color::from_u32(0x00FFFFFF)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00DF4A00);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00CF0303);

    fn id(&self) -> &str {
        "ayu_light"
    }

    fn title(&self) -> &str {
        "AyuLight"
    }
}
