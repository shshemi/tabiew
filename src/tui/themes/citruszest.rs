
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Citruszest;

impl SixColorsTwoRowsStyler for Citruszest {
    const BACKGROUND: Color = Color::from_u32(0x00121212);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00323232);
    const FOREGROUND: Color = Color::from_u32(0x00bfbfbf);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00404040);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00ff1a75),
        Color::from_u32(0x001affa3),
        Color::from_u32(0x00ffff00),
        Color::from_u32(0x0033cfff),
        Color::from_u32(0x00ffb2fe),
        Color::from_u32(0x0000fff2),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00ff5454),
        Color::from_u32(0x0000cc7a),
        Color::from_u32(0x00ffd400),
        Color::from_u32(0x0000bfff),
        Color::from_u32(0x00ff90fe),
        Color::from_u32(0x0048d1cc),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00181818), Color::from_u32(0x001E1E1E)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00464646);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00CFA400);

    fn id(&self) -> &str {
        "citruszest"
    }

    fn title(&self) -> &str {
        "Citruszest"
    }
}
