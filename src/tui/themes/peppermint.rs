
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Peppermint;

impl SixColorsTwoRowsStyler for Peppermint {
    const BACKGROUND: Color = Color::from_u32(0x00000000);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00202020);
    const FOREGROUND: Color = Color::from_u32(0x00c8c8c8);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00353535);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00e4859b),
        Color::from_u32(0x00a3cca2),
        Color::from_u32(0x00e1e487),
        Color::from_u32(0x006fbce2),
        Color::from_u32(0x00e586e7),
        Color::from_u32(0x0096dcdb),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00e74669),
        Color::from_u32(0x0089d287),
        Color::from_u32(0x00dab853),
        Color::from_u32(0x00449fd0),
        Color::from_u32(0x00da62dc),
        Color::from_u32(0x0065aaaf),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00060606), Color::from_u32(0x000C0C0C)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x009B9B9B);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00B71639);
}
