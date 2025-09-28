
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Shaman;

impl SixColorsTwoRowsStyler for Shaman {
    const BACKGROUND: Color = Color::from_u32(0x00001015);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00203035);
    const FOREGROUND: Color = Color::from_u32(0x00405555);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00012026);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00ff4242),
        Color::from_u32(0x002aea5e),
        Color::from_u32(0x008ed4fd),
        Color::from_u32(0x0061d5ba),
        Color::from_u32(0x001298ff),
        Color::from_u32(0x0098d028),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00b2302d),
        Color::from_u32(0x0000a941),
        Color::from_u32(0x005e8baa),
        Color::from_u32(0x00449a86),
        Color::from_u32(0x0000599d),
        Color::from_u32(0x005d7e19),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x0006161B), Color::from_u32(0x000C1C21)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x002ADCB6);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00820000);
}
