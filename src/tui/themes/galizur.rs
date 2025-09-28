
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Galizur;

impl SixColorsTwoRowsStyler for Galizur {
    const BACKGROUND: Color = Color::from_u32(0x00071317);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00273337);
    const FOREGROUND: Color = Color::from_u32(0x00ddeeff);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00223344);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00ff1133),
        Color::from_u32(0x0033ff11),
        Color::from_u32(0x00ffdd33),
        Color::from_u32(0x003377ff),
        Color::from_u32(0x00aa77ff),
        Color::from_u32(0x0033ddff),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00aa1122),
        Color::from_u32(0x0033aa11),
        Color::from_u32(0x00ccaa22),
        Color::from_u32(0x002255cc),
        Color::from_u32(0x007755aa),
        Color::from_u32(0x0022bbdd),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x000D191D), Color::from_u32(0x00131F23)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00BDCEDF);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x009C7A00);
}
