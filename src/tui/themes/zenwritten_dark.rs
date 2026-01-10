use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct ZenwrittenDark;

impl SixColorsTwoRowsStyler for ZenwrittenDark {
    const BACKGROUND: Color = Color::from_u32(0x00191919);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00393939);
    const FOREGROUND: Color = Color::from_u32(0x00bbbbbb);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00191919);

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

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x001F1F1F), Color::from_u32(0x00252525)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00A9A9A9);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00AE3E4C);
}
