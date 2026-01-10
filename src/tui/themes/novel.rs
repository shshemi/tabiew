use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Novel;

impl SixColorsTwoRowsStyler for Novel {
    const BACKGROUND: Color = Color::from_u32(0x00dfdbc3);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00FFFBE3);
    const FOREGROUND: Color = Color::from_u32(0x003b2322);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00000000);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00cc0000),
        Color::from_u32(0x00009600),
        Color::from_u32(0x00d06b00),
        Color::from_u32(0x000000cc),
        Color::from_u32(0x00cc00cc),
        Color::from_u32(0x000087cc),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00cc0000),
        Color::from_u32(0x00009600),
        Color::from_u32(0x00d06b00),
        Color::from_u32(0x000000cc),
        Color::from_u32(0x00cc00cc),
        Color::from_u32(0x000087cc),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00E5E1C9), Color::from_u32(0x00EBE7CF)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x0053433A);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00A03B00);
}
