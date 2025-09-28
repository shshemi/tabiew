
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct HorizonBright;

impl SixColorsTwoRowsStyler for HorizonBright {
    const BACKGROUND: Color = Color::from_u32(0x00fdf0ed);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00FFFFFF);
    const FOREGROUND: Color = Color::from_u32(0x0016161d);
    const DARK_FOREGROUND: Color = Color::from_u32(0x0016161d);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00fe5f87),
        Color::from_u32(0x0000dea0),
        Color::from_u32(0x00ffc0a2),
        Color::from_u32(0x0000c9e2),
        Color::from_u32(0x00ff6cba),
        Color::from_u32(0x0007e9e8),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00fc4777),
        Color::from_u32(0x0000d793),
        Color::from_u32(0x00ffb38f),
        Color::from_u32(0x0000bedd),
        Color::from_u32(0x00ff58b1),
        Color::from_u32(0x0000e7e5),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00FFF6F3), Color::from_u32(0x00FFFCF9)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00D9AEA3);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00CF835F);
}
