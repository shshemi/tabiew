
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Brogrammer;

impl SixColorsTwoRowsStyler for Brogrammer {
    const BACKGROUND: Color = Color::from_u32(0x00131313);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00333333);
    const FOREGROUND: Color = Color::from_u32(0x00d6dbe5);
    const DARK_FOREGROUND: Color = Color::from_u32(0x001f1f1f);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00de352e),
        Color::from_u32(0x001dd361),
        Color::from_u32(0x00f3bd09),
        Color::from_u32(0x001081d6),
        Color::from_u32(0x005350b9),
        Color::from_u32(0x000f7ddb),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00f81118),
        Color::from_u32(0x002dc55e),
        Color::from_u32(0x00ecba0f),
        Color::from_u32(0x002a84d2),
        Color::from_u32(0x004e5ab7),
        Color::from_u32(0x001081d6),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00191919), Color::from_u32(0x001F1F1F)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00999999);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00C80000);

    fn id(&self) -> &str {
        "brogrammer"
    }

    fn title(&self) -> &str {
        "Brogrammer"
    }
}
