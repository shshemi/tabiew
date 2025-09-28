
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct NordLight;

impl SixColorsTwoRowsStyler for NordLight {
    const BACKGROUND: Color = Color::from_u32(0x00e5e9f0);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00FFFFFF);
    const FOREGROUND: Color = Color::from_u32(0x00414858);
    const DARK_FOREGROUND: Color = Color::from_u32(0x003b4252);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00bf616a),
        Color::from_u32(0x00a3be8c),
        Color::from_u32(0x00ebcb8b),
        Color::from_u32(0x0081a1c1),
        Color::from_u32(0x00b48ead),
        Color::from_u32(0x008fbcbb),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00bf616a),
        Color::from_u32(0x00a3be8c),
        Color::from_u32(0x00ebcb8b),
        Color::from_u32(0x0081a1c1),
        Color::from_u32(0x00b48ead),
        Color::from_u32(0x0088c0d0),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00EBEFF6), Color::from_u32(0x00F1F5FC)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x0068A0B0);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00BB9B5B);
}
