use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct IdleToes;

impl SixColorsTwoRowsStyler for IdleToes {
    const BACKGROUND: Color = Color::from_u32(0x00323232);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00525252);
    const FOREGROUND: Color = Color::from_u32(0x00ffffff);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00323232);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00f07070),
        Color::from_u32(0x009dff91),
        Color::from_u32(0x00ffe48b),
        Color::from_u32(0x005eb7f7),
        Color::from_u32(0x00ff9dff),
        Color::from_u32(0x00dcf4ff),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00d25252),
        Color::from_u32(0x007fe173),
        Color::from_u32(0x00ffc66d),
        Color::from_u32(0x004099ff),
        Color::from_u32(0x00f680ff),
        Color::from_u32(0x00bed6ff),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00383838), Color::from_u32(0x003E3E3E)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00B6B6B6);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00CF963D);
}
