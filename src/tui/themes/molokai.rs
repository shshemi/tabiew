
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Molokai;

impl SixColorsTwoRowsStyler for Molokai {
    const BACKGROUND: Color = Color::from_u32(0x00121212);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00323232);
    const FOREGROUND: Color = Color::from_u32(0x00bbbbbb);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00121212);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00f6669d),
        Color::from_u32(0x00b1e05f),
        Color::from_u32(0x00fff26d),
        Color::from_u32(0x0000afff),
        Color::from_u32(0x00af87ff),
        Color::from_u32(0x0051ceff),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00fa2573),
        Color::from_u32(0x0098e123),
        Color::from_u32(0x00dfd460),
        Color::from_u32(0x001080d0),
        Color::from_u32(0x008700ff),
        Color::from_u32(0x0043a8d0),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00181818), Color::from_u32(0x001E1E1E)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x009B9B9B);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00CA0043);

    fn id(&self) -> &str {
        "molokai"
    }

    fn title(&self) -> &str {
        "Molokai"
    }
}
