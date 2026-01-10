use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Ryuuko;

impl SixColorsTwoRowsStyler for Ryuuko {
    const BACKGROUND: Color = Color::from_u32(0x002c3941);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x004C5961);
    const FOREGROUND: Color = Color::from_u32(0x00ececec);
    const DARK_FOREGROUND: Color = Color::from_u32(0x002c3941);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00865f5b),
        Color::from_u32(0x0066907d),
        Color::from_u32(0x00b1a990),
        Color::from_u32(0x006a8e95),
        Color::from_u32(0x00b18a73),
        Color::from_u32(0x0088b2ac),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00865f5b),
        Color::from_u32(0x0066907d),
        Color::from_u32(0x00b1a990),
        Color::from_u32(0x006a8e95),
        Color::from_u32(0x00b18a73),
        Color::from_u32(0x0088b2ac),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00323F47), Color::from_u32(0x0038454D)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00CCCCCC);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00815A43);
}
