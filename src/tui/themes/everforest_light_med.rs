
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct EverforestLightMed;

impl SixColorsTwoRowsStyler for EverforestLightMed {
    const BACKGROUND: Color = Color::from_u32(0x00efebd4);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00FFFFF4);
    const FOREGROUND: Color = Color::from_u32(0x005c6a72);
    const DARK_FOREGROUND: Color = Color::from_u32(0x007a8478);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00f85552),
        Color::from_u32(0x008da101),
        Color::from_u32(0x00dfa000),
        Color::from_u32(0x003a94c5),
        Color::from_u32(0x00df69ba),
        Color::from_u32(0x0035a77c),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00e67e80),
        Color::from_u32(0x00a7c080),
        Color::from_u32(0x00dbbc7f),
        Color::from_u32(0x007fbbb3),
        Color::from_u32(0x00d699b6),
        Color::from_u32(0x0083c092),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00F5F1DA), Color::from_u32(0x00FBF7E0)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00D55D06);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00B64E50);
}
