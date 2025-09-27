
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct BlueDolphin;

impl SixColorsTwoRowsStyler for BlueDolphin {
    const BACKGROUND: Color = Color::from_u32(0x00006984);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x002089A4);
    const FOREGROUND: Color = Color::from_u32(0x00c5f2ff);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00292d3e);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00ff8b92),
        Color::from_u32(0x00ddffa7),
        Color::from_u32(0x00ffe585),
        Color::from_u32(0x009cc4ff),
        Color::from_u32(0x00ddb0f6),
        Color::from_u32(0x00a3f7ff),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00ff8288),
        Color::from_u32(0x00b4e88d),
        Color::from_u32(0x00f4d69f),
        Color::from_u32(0x0082aaff),
        Color::from_u32(0x00e9c1ff),
        Color::from_u32(0x0089ebff),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00066F8A), Color::from_u32(0x000C7590)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00DFAC00);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00CF5258);

    fn id(&self) -> &str {
        "blue_dolphin"
    }

    fn title(&self) -> &str {
        "BlueDolphin"
    }
}
