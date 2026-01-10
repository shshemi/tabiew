use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct BananaBlueberry;

impl SixColorsTwoRowsStyler for BananaBlueberry {
    const BACKGROUND: Color = Color::from_u32(0x00191323);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00393343);
    const FOREGROUND: Color = Color::from_u32(0x00cccccc);
    const DARK_FOREGROUND: Color = Color::from_u32(0x0017141f);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00fe9ea1),
        Color::from_u32(0x0098c379),
        Color::from_u32(0x00f9e46b),
        Color::from_u32(0x0091fff4),
        Color::from_u32(0x00da70d6),
        Color::from_u32(0x00bcf3ff),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00ff6b7f),
        Color::from_u32(0x0000bd9c),
        Color::from_u32(0x00e6c62f),
        Color::from_u32(0x0022e8df),
        Color::from_u32(0x00dc396a),
        Color::from_u32(0x0056b6c2),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x001F1929), Color::from_u32(0x00251F2F)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00C05D00);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00CF3B4F);
}
