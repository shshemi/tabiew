use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Elementary;

impl SixColorsTwoRowsStyler for Elementary {
    const BACKGROUND: Color = Color::from_u32(0x00181818);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00383838);
    const FOREGROUND: Color = Color::from_u32(0x00efefef);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00242424);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00fc1c18),
        Color::from_u32(0x006bc219),
        Color::from_u32(0x00fec80e),
        Color::from_u32(0x000955ff),
        Color::from_u32(0x00fb0050),
        Color::from_u32(0x003ea8fc),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00d71c15),
        Color::from_u32(0x005aa513),
        Color::from_u32(0x00fdb40c),
        Color::from_u32(0x00063b8c),
        Color::from_u32(0x00e40038),
        Color::from_u32(0x002595e1),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x001E1E1E), Color::from_u32(0x00242424)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x009B9B9B);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00CD8400);
}
