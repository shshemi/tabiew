
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct BelafonteDay;

impl SixColorsTwoRowsStyler for BelafonteDay {
    const BACKGROUND: Color = Color::from_u32(0x00d5ccba);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00F5ECDA);
    const FOREGROUND: Color = Color::from_u32(0x0045373c);
    const DARK_FOREGROUND: Color = Color::from_u32(0x0020111b);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00be100e),
        Color::from_u32(0x00858162),
        Color::from_u32(0x00eaa549),
        Color::from_u32(0x00426a79),
        Color::from_u32(0x0097522c),
        Color::from_u32(0x00989a9c),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00be100e),
        Color::from_u32(0x00858162),
        Color::from_u32(0x00eaa549),
        Color::from_u32(0x00426a79),
        Color::from_u32(0x0097522c),
        Color::from_u32(0x00989a9c),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00DBD2C0), Color::from_u32(0x00E1D8C6)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x0025171C);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00BA7519);
}
