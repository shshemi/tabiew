use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct BlueBerryPie;

impl SixColorsTwoRowsStyler for BlueBerryPie {
    const BACKGROUND: Color = Color::from_u32(0x001c0c28);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x003C2C48);
    const FOREGROUND: Color = Color::from_u32(0x00babab9);
    const DARK_FOREGROUND: Color = Color::from_u32(0x000a4c62);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00c87272),
        Color::from_u32(0x000a6c7e),
        Color::from_u32(0x007a3188),
        Color::from_u32(0x0039173d),
        Color::from_u32(0x00bc94b7),
        Color::from_u32(0x005e6071),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x0099246e),
        Color::from_u32(0x005cb1b3),
        Color::from_u32(0x00eab9a8),
        Color::from_u32(0x0090a5bd),
        Color::from_u32(0x009d54a7),
        Color::from_u32(0x007e83cc),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x0022122E), Color::from_u32(0x00281834)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00DCDAB6);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00BA8978);
}
