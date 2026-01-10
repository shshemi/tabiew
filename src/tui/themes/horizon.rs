use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Horizon;

impl SixColorsTwoRowsStyler for Horizon {
    const BACKGROUND: Color = Color::from_u32(0x001c1e26);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x003C3E46);
    const FOREGROUND: Color = Color::from_u32(0x00d5d8da);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00000000);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00ec6a88),
        Color::from_u32(0x003fdaa4),
        Color::from_u32(0x00fbc3a7),
        Color::from_u32(0x003fc4de),
        Color::from_u32(0x00f075b5),
        Color::from_u32(0x006be4e6),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00e95678),
        Color::from_u32(0x0029d398),
        Color::from_u32(0x00fab795),
        Color::from_u32(0x0026bbd9),
        Color::from_u32(0x00ee64ac),
        Color::from_u32(0x0059e1e3),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x0022242C), Color::from_u32(0x00282A32)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x004C4F73);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00CA8765);
}
