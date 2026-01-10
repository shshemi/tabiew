use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Snazzy;

impl SixColorsTwoRowsStyler for Snazzy {
    const BACKGROUND: Color = Color::from_u32(0x001e1f29);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x003E3F49);
    const FOREGROUND: Color = Color::from_u32(0x00ebece6);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00000000);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00fc4346),
        Color::from_u32(0x0050fb7c),
        Color::from_u32(0x00f0fb8c),
        Color::from_u32(0x0049baff),
        Color::from_u32(0x00fc4cb4),
        Color::from_u32(0x008be9fe),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00fc4346),
        Color::from_u32(0x0050fb7c),
        Color::from_u32(0x00f0fb8c),
        Color::from_u32(0x0049baff),
        Color::from_u32(0x00fc4cb4),
        Color::from_u32(0x008be9fe),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x0024252F), Color::from_u32(0x002A2B35)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00C4C4C4);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00CC1316);
}
