use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct SquirrelsongDark;

impl SixColorsTwoRowsStyler for SquirrelsongDark {
    const BACKGROUND: Color = Color::from_u32(0x00372920);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00574940);
    const FOREGROUND: Color = Color::from_u32(0x00b19b89);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00372920);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00df4d43),
        Color::from_u32(0x00659a4c),
        Color::from_u32(0x00e8c23f),
        Color::from_u32(0x004ca4db),
        Color::from_u32(0x009d70da),
        Color::from_u32(0x0060aca9),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00ba4138),
        Color::from_u32(0x00468336),
        Color::from_u32(0x00d4b139),
        Color::from_u32(0x004395c6),
        Color::from_u32(0x00855fb8),
        Color::from_u32(0x002f9794),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x003D2F26), Color::from_u32(0x0043352C)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00917B69);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00A48109);
}
