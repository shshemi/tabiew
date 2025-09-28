
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct SynthwaveAlpha;

impl SixColorsTwoRowsStyler for SynthwaveAlpha {
    const BACKGROUND: Color = Color::from_u32(0x00241b30);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00443B50);
    const FOREGROUND: Color = Color::from_u32(0x00f2f2e3);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00241b30);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00e60a70),
        Color::from_u32(0x000ae4a4),
        Color::from_u32(0x00f9f972),
        Color::from_u32(0x00aa54f9),
        Color::from_u32(0x00ff00f6),
        Color::from_u32(0x0000fbfd),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00e60a70),
        Color::from_u32(0x0000986c),
        Color::from_u32(0x00adad3e),
        Color::from_u32(0x006e29ad),
        Color::from_u32(0x00b300ad),
        Color::from_u32(0x0000b0b1),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x002A2136), Color::from_u32(0x0030273C)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00D2D2C3);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00B60040);
}
