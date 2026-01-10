use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct FireflyTraditional;

impl SixColorsTwoRowsStyler for FireflyTraditional {
    const BACKGROUND: Color = Color::from_u32(0x00000000);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00202020);
    const FOREGROUND: Color = Color::from_u32(0x00f5f5f5);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00000000);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00ff3b1e),
        Color::from_u32(0x002ee720),
        Color::from_u32(0x00ecec16),
        Color::from_u32(0x00838dff),
        Color::from_u32(0x00ff5cfe),
        Color::from_u32(0x0029f0f0),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00c23720),
        Color::from_u32(0x0033bc26),
        Color::from_u32(0x00afad24),
        Color::from_u32(0x005a63ff),
        Color::from_u32(0x00d53ad2),
        Color::from_u32(0x0033bbc7),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00060606), Color::from_u32(0x000C0C0C)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x0000D900);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00A50AA2);
}
