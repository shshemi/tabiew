
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Calamity;

impl SixColorsTwoRowsStyler for Calamity {
    const BACKGROUND: Color = Color::from_u32(0x002f2833);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x004F4853);
    const FOREGROUND: Color = Color::from_u32(0x00d5ced9);
    const DARK_FOREGROUND: Color = Color::from_u32(0x002f2833);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00fc644d),
        Color::from_u32(0x00a5f69c),
        Color::from_u32(0x00e9d7a5),
        Color::from_u32(0x003b79c7),
        Color::from_u32(0x00f92672),
        Color::from_u32(0x0074d3de),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00fc644d),
        Color::from_u32(0x00a5f69c),
        Color::from_u32(0x00e9d7a5),
        Color::from_u32(0x003b79c7),
        Color::from_u32(0x00f92672),
        Color::from_u32(0x0074d3de),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00352E39), Color::from_u32(0x003B343F)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00B5AEB9);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00CC341D);

    fn id(&self) -> &str {
        "calamity"
    }

    fn title(&self) -> &str {
        "Calamity"
    }
}
