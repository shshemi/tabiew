use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct OneHalfLight;

impl SixColorsTwoRowsStyler for OneHalfLight {
    const BACKGROUND: Color = Color::from_u32(0x00fafafa);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00FFFFFF);
    const FOREGROUND: Color = Color::from_u32(0x00383a42);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00383a42);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00e06c75),
        Color::from_u32(0x0098c379),
        Color::from_u32(0x00e5c07b),
        Color::from_u32(0x0061afef),
        Color::from_u32(0x00c678dd),
        Color::from_u32(0x0056b6c2),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00e45649),
        Color::from_u32(0x0050a14f),
        Color::from_u32(0x00c18401),
        Color::from_u32(0x000184bc),
        Color::from_u32(0x00a626a4),
        Color::from_u32(0x000997b3),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00FFFFFF), Color::from_u32(0x00FFFFFF)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x009FAEDF);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00B42619);
}
