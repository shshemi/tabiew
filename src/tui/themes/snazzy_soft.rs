use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct SnazzySoft;

impl SixColorsTwoRowsStyler for SnazzySoft {
    const BACKGROUND: Color = Color::from_u32(0x00282a36);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00484A56);
    const FOREGROUND: Color = Color::from_u32(0x00eff0eb);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00000000);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00ff5c57),
        Color::from_u32(0x005af78e),
        Color::from_u32(0x00f3f99d),
        Color::from_u32(0x0057c7ff),
        Color::from_u32(0x00ff6ac1),
        Color::from_u32(0x009aedfe),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00ff5c57),
        Color::from_u32(0x005af78e),
        Color::from_u32(0x00f3f99d),
        Color::from_u32(0x0057c7ff),
        Color::from_u32(0x00ff6ac1),
        Color::from_u32(0x009aedfe),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x002E303C), Color::from_u32(0x00343642)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00CACACA);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00CF2C27);
}
