use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Primary;

impl SixColorsTwoRowsStyler for Primary {
    const BACKGROUND: Color = Color::from_u32(0x00ffffff);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00FFFFFF);
    const FOREGROUND: Color = Color::from_u32(0x00000000);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00000000);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00db4437),
        Color::from_u32(0x000f9d58),
        Color::from_u32(0x00f4b400),
        Color::from_u32(0x004285f4),
        Color::from_u32(0x004285f4),
        Color::from_u32(0x000f9d58),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00db4437),
        Color::from_u32(0x000f9d58),
        Color::from_u32(0x00f4b400),
        Color::from_u32(0x004285f4),
        Color::from_u32(0x00db4437),
        Color::from_u32(0x004285f4),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00FFFFFF), Color::from_u32(0x00FFFFFF)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00000000);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00C48400);
}
