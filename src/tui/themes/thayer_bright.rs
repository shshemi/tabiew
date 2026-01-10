use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct ThayerBright;

impl SixColorsTwoRowsStyler for ThayerBright {
    const BACKGROUND: Color = Color::from_u32(0x001b1d1e);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x003B3D3E);
    const FOREGROUND: Color = Color::from_u32(0x00f8f8f8);
    const DARK_FOREGROUND: Color = Color::from_u32(0x001b1d1e);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00ff5995),
        Color::from_u32(0x00b6e354),
        Color::from_u32(0x00feed6c),
        Color::from_u32(0x003f78ff),
        Color::from_u32(0x009e6ffe),
        Color::from_u32(0x0023cfd5),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00f92672),
        Color::from_u32(0x004df840),
        Color::from_u32(0x00f4fd22),
        Color::from_u32(0x002757d6),
        Color::from_u32(0x008c54fe),
        Color::from_u32(0x0038c8b5),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00212324), Color::from_u32(0x0027292A)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00DC7700);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00C90042);
}
