
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Aurora;

impl SixColorsTwoRowsStyler for Aurora {
    const BACKGROUND: Color = Color::from_u32(0x0023262e);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x0043464E);
    const FOREGROUND: Color = Color::from_u32(0x00ffca28);
    const DARK_FOREGROUND: Color = Color::from_u32(0x0023262e);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00f92672),
        Color::from_u32(0x008fd46d),
        Color::from_u32(0x00ffe66d),
        Color::from_u32(0x0003d6b8),
        Color::from_u32(0x00ee5d43),
        Color::from_u32(0x0003d6b8),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00f0266f),
        Color::from_u32(0x008fd46d),
        Color::from_u32(0x00ffe66d),
        Color::from_u32(0x000321d7),
        Color::from_u32(0x00ee5d43),
        Color::from_u32(0x0003d6b8),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00292C34), Color::from_u32(0x002F323A)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00CE3D23);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00CFB63D);
}
