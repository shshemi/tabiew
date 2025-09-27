
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct BlackMetal;

impl SixColorsTwoRowsStyler for BlackMetal {
    const BACKGROUND: Color = Color::from_u32(0x00000000);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00202020);
    const FOREGROUND: Color = Color::from_u32(0x00c1c1c1);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00000000);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00486e6f),
        Color::from_u32(0x00dd9999),
        Color::from_u32(0x00a06666),
        Color::from_u32(0x00888888),
        Color::from_u32(0x00999999),
        Color::from_u32(0x00aaaaaa),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00486e6f),
        Color::from_u32(0x00dd9999),
        Color::from_u32(0x00a06666),
        Color::from_u32(0x00888888),
        Color::from_u32(0x00999999),
        Color::from_u32(0x00aaaaaa),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00060606), Color::from_u32(0x000C0C0C)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00A1A1A1);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00AD6969);

    fn id(&self) -> &str {
        "black_metal"
    }

    fn title(&self) -> &str {
        "BlackMetal"
    }
}
