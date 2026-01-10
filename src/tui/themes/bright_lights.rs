use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct BrightLights;

impl SixColorsTwoRowsStyler for BrightLights {
    const BACKGROUND: Color = Color::from_u32(0x00191919);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00393939);
    const FOREGROUND: Color = Color::from_u32(0x00b3c9d7);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00191919);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00ff355b),
        Color::from_u32(0x00b7e876),
        Color::from_u32(0x00ffc251),
        Color::from_u32(0x0076d5ff),
        Color::from_u32(0x00ba76e7),
        Color::from_u32(0x006cbfb5),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00ff355b),
        Color::from_u32(0x00b7e876),
        Color::from_u32(0x00ffc251),
        Color::from_u32(0x0076d4ff),
        Color::from_u32(0x00ba76e7),
        Color::from_u32(0x006cbfb5),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x001F1F1F), Color::from_u32(0x00252525)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00D32B00);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00CF9221);
}
