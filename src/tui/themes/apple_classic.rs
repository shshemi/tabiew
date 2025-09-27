
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct AppleClassic;

impl SixColorsTwoRowsStyler for AppleClassic {
    const BACKGROUND: Color = Color::from_u32(0x002c2b2b);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x004C4B4B);
    const FOREGROUND: Color = Color::from_u32(0x00d5a200);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00000000);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00ff6e67),
        Color::from_u32(0x005ffa68),
        Color::from_u32(0x00fffc67),
        Color::from_u32(0x006871ff),
        Color::from_u32(0x00ff77ff),
        Color::from_u32(0x0060fdff),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00c91b00),
        Color::from_u32(0x0000c200),
        Color::from_u32(0x00c7c400),
        Color::from_u32(0x000225c7),
        Color::from_u32(0x00ca30c7),
        Color::from_u32(0x0000c5c7),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00323131), Color::from_u32(0x00383737)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00A7A7A7);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x009A0097);

    fn id(&self) -> &str {
        "apple_classic"
    }

    fn title(&self) -> &str {
        "AppleClassic"
    }
}
