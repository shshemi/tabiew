
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Arcoiris;

impl SixColorsTwoRowsStyler for Arcoiris {
    const BACKGROUND: Color = Color::from_u32(0x00201f1e);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00403F3E);
    const FOREGROUND: Color = Color::from_u32(0x00eee4d9);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00333333);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00ffb9b9),
        Color::from_u32(0x00e3f6aa),
        Color::from_u32(0x00ffddaa),
        Color::from_u32(0x00b3e8f3),
        Color::from_u32(0x00cbbaf9),
        Color::from_u32(0x00bcffc7),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00da2700),
        Color::from_u32(0x0012c258),
        Color::from_u32(0x00ffc656),
        Color::from_u32(0x00518bfc),
        Color::from_u32(0x00e37bd9),
        Color::from_u32(0x0063fad5),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00262524), Color::from_u32(0x002C2B2A)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x005A0000);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00CF9626);

    fn id(&self) -> &str {
        "arcoiris"
    }

    fn title(&self) -> &str {
        "Arcoiris"
    }
}
