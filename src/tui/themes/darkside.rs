
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Darkside;

impl SixColorsTwoRowsStyler for Darkside {
    const BACKGROUND: Color = Color::from_u32(0x00222324);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00424344);
    const FOREGROUND: Color = Color::from_u32(0x00bababa);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00000000);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00e05a4f),
        Color::from_u32(0x0077b869),
        Color::from_u32(0x00efd64b),
        Color::from_u32(0x00387cd3),
        Color::from_u32(0x00957bbe),
        Color::from_u32(0x003d97e2),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00e8341c),
        Color::from_u32(0x0068c256),
        Color::from_u32(0x00f2d42c),
        Color::from_u32(0x001c98e8),
        Color::from_u32(0x008e69c9),
        Color::from_u32(0x001c98e8),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x0028292A), Color::from_u32(0x002E2F30)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x009B9B9B);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00C2A400);

    fn id(&self) -> &str {
        "darkside"
    }

    fn title(&self) -> &str {
        "Darkside"
    }
}
