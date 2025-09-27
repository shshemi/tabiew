
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Unikitty;

impl SixColorsTwoRowsStyler for Unikitty {
    const BACKGROUND: Color = Color::from_u32(0x00ff8cd9);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00FFACF9);
    const FOREGROUND: Color = Color::from_u32(0x000b0b0b);
    const DARK_FOREGROUND: Color = Color::from_u32(0x000c0c0c);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00d91329),
        Color::from_u32(0x00d3ffaf),
        Color::from_u32(0x00ffef50),
        Color::from_u32(0x000075ea),
        Color::from_u32(0x00fdd5e5),
        Color::from_u32(0x0079ecd5),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00a80f20),
        Color::from_u32(0x00bafc8b),
        Color::from_u32(0x00eedf4b),
        Color::from_u32(0x00145fcd),
        Color::from_u32(0x00ff36a2),
        Color::from_u32(0x006bd1bc),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00FF92DF), Color::from_u32(0x00FF98E5)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x009ADC6B);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00CF0672);

    fn id(&self) -> &str {
        "unikitty"
    }

    fn title(&self) -> &str {
        "Unikitty"
    }
}
