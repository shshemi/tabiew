use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct AdwaitaDark;

impl SixColorsTwoRowsStyler for AdwaitaDark {
    const BACKGROUND: Color = Color::from_u32(0x001d1d20);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x003D3D40);
    const FOREGROUND: Color = Color::from_u32(0x00ffffff);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00241f31);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00ed333b),
        Color::from_u32(0x0057e389),
        Color::from_u32(0x00f8e45c),
        Color::from_u32(0x0051a1ff),
        Color::from_u32(0x00c061cb),
        Color::from_u32(0x004fd2fd),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00c01c28),
        Color::from_u32(0x002ec27e),
        Color::from_u32(0x00f5c211),
        Color::from_u32(0x001e78e4),
        Color::from_u32(0x009841bb),
        Color::from_u32(0x000ab9dc),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00232326), Color::from_u32(0x0029292C)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00DFDFDF);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00C59200);
}
