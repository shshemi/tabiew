use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Encom;

impl SixColorsTwoRowsStyler for Encom {
    const BACKGROUND: Color = Color::from_u32(0x00000000);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00202020);
    const FOREGROUND: Color = Color::from_u32(0x0000a595);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00000000);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00ff0000),
        Color::from_u32(0x0000ee00),
        Color::from_u32(0x00ffff00),
        Color::from_u32(0x000000ff),
        Color::from_u32(0x00ff00ff),
        Color::from_u32(0x0000cdcd),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x009f0000),
        Color::from_u32(0x00008b00),
        Color::from_u32(0x00ffd000),
        Color::from_u32(0x000081ff),
        Color::from_u32(0x00bc00ca),
        Color::from_u32(0x00008b8b),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00060606), Color::from_u32(0x000C0C0C)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x009B9B9B);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00CFA000);
}
