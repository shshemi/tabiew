use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct PurpleRain;

impl SixColorsTwoRowsStyler for PurpleRain {
    const BACKGROUND: Color = Color::from_u32(0x0021084a);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x0041286A);
    const FOREGROUND: Color = Color::from_u32(0x00fffbf6);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00000000);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00ff4250),
        Color::from_u32(0x00b8e36e),
        Color::from_u32(0x00ffd852),
        Color::from_u32(0x0000a6ff),
        Color::from_u32(0x00ac7bf0),
        Color::from_u32(0x0074fdf3),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00ff260e),
        Color::from_u32(0x009be205),
        Color::from_u32(0x00ffc400),
        Color::from_u32(0x0000a2fa),
        Color::from_u32(0x00815bb5),
        Color::from_u32(0x0000deef),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00270E50), Color::from_u32(0x002D1456)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00DF0700);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00CF9400);
}
