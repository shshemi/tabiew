use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct RaycastDark;

impl SixColorsTwoRowsStyler for RaycastDark {
    const BACKGROUND: Color = Color::from_u32(0x001a1a1a);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x003A3A3A);
    const FOREGROUND: Color = Color::from_u32(0x00ffffff);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00000000);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00ff6363),
        Color::from_u32(0x0059d499),
        Color::from_u32(0x00ffc531),
        Color::from_u32(0x0056c2ff),
        Color::from_u32(0x00cf2f98),
        Color::from_u32(0x0052eee5),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00ff5360),
        Color::from_u32(0x0059d499),
        Color::from_u32(0x00ffc531),
        Color::from_u32(0x0056c2ff),
        Color::from_u32(0x00cf2f98),
        Color::from_u32(0x0052eee5),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00202020), Color::from_u32(0x00262626)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00ACACAC);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00CF9501);
}
