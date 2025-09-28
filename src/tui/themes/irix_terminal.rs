
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct IrixTerminal;

impl SixColorsTwoRowsStyler for IrixTerminal {
    const BACKGROUND: Color = Color::from_u32(0x00000043);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00202063);
    const FOREGROUND: Color = Color::from_u32(0x00f2f2f2);
    const DARK_FOREGROUND: Color = Color::from_u32(0x001a1919);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00ffff44),
        Color::from_u32(0x00ffff44),
        Color::from_u32(0x00fffc72),
        Color::from_u32(0x00ffff44),
        Color::from_u32(0x00ffff44),
        Color::from_u32(0x00ffff44),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00ff2b1e),
        Color::from_u32(0x0057ff3d),
        Color::from_u32(0x00ffff44),
        Color::from_u32(0x000004ff),
        Color::from_u32(0x00ff2cff),
        Color::from_u32(0x0056ffff),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00060649), Color::from_u32(0x000C0C4F)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00A7A7A7);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00CF0000);
}
