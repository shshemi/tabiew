
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct EmbersDark;

impl SixColorsTwoRowsStyler for EmbersDark {
    const BACKGROUND: Color = Color::from_u32(0x0016130f);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x0036332F);
    const FOREGROUND: Color = Color::from_u32(0x00a39a90);
    const DARK_FOREGROUND: Color = Color::from_u32(0x0016130f);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00828257),
        Color::from_u32(0x002c2620),
        Color::from_u32(0x00433b32),
        Color::from_u32(0x008a8075),
        Color::from_u32(0x00beb6ae),
        Color::from_u32(0x00825757),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00826d57),
        Color::from_u32(0x0057826d),
        Color::from_u32(0x006d8257),
        Color::from_u32(0x006d5782),
        Color::from_u32(0x0082576d),
        Color::from_u32(0x00576d82),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x001C1915), Color::from_u32(0x00221F1B)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00837A70);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00523D27);

    fn id(&self) -> &str {
        "embers_dark"
    }

    fn title(&self) -> &str {
        "EmbersDark"
    }
}
