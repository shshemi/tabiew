use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct ProLight;

impl SixColorsTwoRowsStyler for ProLight {
    const BACKGROUND: Color = Color::from_u32(0x00ffffff);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00FFFFFF);
    const FOREGROUND: Color = Color::from_u32(0x00191919);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00000000);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00ff6640),
        Color::from_u32(0x0061ef57),
        Color::from_u32(0x00f2f156),
        Color::from_u32(0x000082ff),
        Color::from_u32(0x00ff7eff),
        Color::from_u32(0x0061f7f8),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00e5492b),
        Color::from_u32(0x0050d148),
        Color::from_u32(0x00c6c440),
        Color::from_u32(0x003b75ff),
        Color::from_u32(0x00ed66e8),
        Color::from_u32(0x004ed2de),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00FFFFFF), Color::from_u32(0x00FFFFFF)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x002D2D2D);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00BD36B8);
}
