use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Ollie;

impl SixColorsTwoRowsStyler for Ollie {
    const BACKGROUND: Color = Color::from_u32(0x00222125);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00424145);
    const FOREGROUND: Color = Color::from_u32(0x008a8dae);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00000000);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00ff3d48),
        Color::from_u32(0x003bff99),
        Color::from_u32(0x00ff5e1e),
        Color::from_u32(0x004488ff),
        Color::from_u32(0x00ffc21d),
        Color::from_u32(0x001ffaff),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00ac2e31),
        Color::from_u32(0x0031ac61),
        Color::from_u32(0x00ac4300),
        Color::from_u32(0x002d57ac),
        Color::from_u32(0x00b08528),
        Color::from_u32(0x001fa6ac),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x0028272B), Color::from_u32(0x002E2D31)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x003B4E87);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00805500);
}
