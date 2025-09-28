
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct ZenbonesDark;

impl SixColorsTwoRowsStyler for ZenbonesDark {
    const BACKGROUND: Color = Color::from_u32(0x001c1917);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x003C3937);
    const FOREGROUND: Color = Color::from_u32(0x00b4bdc3);
    const DARK_FOREGROUND: Color = Color::from_u32(0x001c1917);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00e8838f),
        Color::from_u32(0x008bae68),
        Color::from_u32(0x00d68c67),
        Color::from_u32(0x0061abda),
        Color::from_u32(0x00cf86c1),
        Color::from_u32(0x0065b8c1),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00de6e7c),
        Color::from_u32(0x00819b69),
        Color::from_u32(0x00b77e64),
        Color::from_u32(0x006099c0),
        Color::from_u32(0x00b279a7),
        Color::from_u32(0x0066a5ad),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00221F1D), Color::from_u32(0x00282523)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00A4AAAF);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00AE3E4C);
}
