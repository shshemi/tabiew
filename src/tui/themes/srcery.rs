
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Srcery;

impl SixColorsTwoRowsStyler for Srcery {
    const BACKGROUND: Color = Color::from_u32(0x001c1b19);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x003C3B39);
    const FOREGROUND: Color = Color::from_u32(0x00fce8c3);
    const DARK_FOREGROUND: Color = Color::from_u32(0x001c1b19);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00f75341),
        Color::from_u32(0x0098bc37),
        Color::from_u32(0x00fed06e),
        Color::from_u32(0x0068a8e4),
        Color::from_u32(0x00ff5c8f),
        Color::from_u32(0x002be4d0),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00ef2f27),
        Color::from_u32(0x00519f50),
        Color::from_u32(0x00fbb829),
        Color::from_u32(0x002c78bf),
        Color::from_u32(0x00e02c6d),
        Color::from_u32(0x000aaeb3),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x0022211F), Color::from_u32(0x00282725)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00DB9809);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00CB8800);

    fn id(&self) -> &str {
        "srcery"
    }

    fn title(&self) -> &str {
        "Srcery"
    }
}
