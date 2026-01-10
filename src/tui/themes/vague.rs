use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Vague;

impl SixColorsTwoRowsStyler for Vague {
    const BACKGROUND: Color = Color::from_u32(0x00141415);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00343435);
    const FOREGROUND: Color = Color::from_u32(0x00cdcdcd);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00141415);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00df6882),
        Color::from_u32(0x008cb66d),
        Color::from_u32(0x00f3be7c),
        Color::from_u32(0x007e98e8),
        Color::from_u32(0x00c3c3d5),
        Color::from_u32(0x009bb4bc),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00df6882),
        Color::from_u32(0x008cb66d),
        Color::from_u32(0x00f3be7c),
        Color::from_u32(0x007e98e8),
        Color::from_u32(0x00c3c3d5),
        Color::from_u32(0x009bb4bc),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x001A1A1B), Color::from_u32(0x00202021)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00ADADAD);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00C38E4C);
}
