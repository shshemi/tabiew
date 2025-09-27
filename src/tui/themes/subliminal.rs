
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Subliminal;

impl SixColorsTwoRowsStyler for Subliminal {
    const BACKGROUND: Color = Color::from_u32(0x00282c35);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00484C55);
    const FOREGROUND: Color = Color::from_u32(0x00d4d4d4);
    const DARK_FOREGROUND: Color = Color::from_u32(0x007f7f7f);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00e15a60),
        Color::from_u32(0x00a9cfa4),
        Color::from_u32(0x00ffe2a9),
        Color::from_u32(0x006699cc),
        Color::from_u32(0x00f1a5ab),
        Color::from_u32(0x005fb3b3),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00e15a60),
        Color::from_u32(0x00a9cfa4),
        Color::from_u32(0x00ffe2a9),
        Color::from_u32(0x006699cc),
        Color::from_u32(0x00f1a5ab),
        Color::from_u32(0x005fb3b3),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x002E323B), Color::from_u32(0x00343841)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00A7A7A7);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00CFB279);

    fn id(&self) -> &str {
        "subliminal"
    }

    fn title(&self) -> &str {
        "Subliminal"
    }
}
