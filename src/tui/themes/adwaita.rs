
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Adwaita;

impl SixColorsTwoRowsStyler for Adwaita {
    const BACKGROUND: Color = Color::from_u32(0x00ffffff);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00FFFFFF);
    const FOREGROUND: Color = Color::from_u32(0x00000000);
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

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00FFFFFF), Color::from_u32(0x00FFFFFF)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00000000);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00C59200);
}
