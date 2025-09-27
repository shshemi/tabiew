
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Misterioso;

impl SixColorsTwoRowsStyler for Misterioso {
    const BACKGROUND: Color = Color::from_u32(0x002d3743);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x004D5763);
    const FOREGROUND: Color = Color::from_u32(0x00e1e1e0);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00000000);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00ff3242),
        Color::from_u32(0x0074cd68),
        Color::from_u32(0x00ffb929),
        Color::from_u32(0x0023d7d7),
        Color::from_u32(0x00ff37ff),
        Color::from_u32(0x0000ede1),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00ff4242),
        Color::from_u32(0x0074af68),
        Color::from_u32(0x00ffad29),
        Color::from_u32(0x00338f86),
        Color::from_u32(0x009414e6),
        Color::from_u32(0x0023d7d7),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00333D49), Color::from_u32(0x0039434F)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00000000);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00CF7D00);

    fn id(&self) -> &str {
        "misterioso"
    }

    fn title(&self) -> &str {
        "Misterioso"
    }
}
