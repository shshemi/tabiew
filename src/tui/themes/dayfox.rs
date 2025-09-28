
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Dayfox;

impl SixColorsTwoRowsStyler for Dayfox {
    const BACKGROUND: Color = Color::from_u32(0x00f6f2ee);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00FFFFFF);
    const FOREGROUND: Color = Color::from_u32(0x003d2b5a);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00352c24);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00b3434e),
        Color::from_u32(0x00577f63),
        Color::from_u32(0x00b86e28),
        Color::from_u32(0x004863b6),
        Color::from_u32(0x008452d5),
        Color::from_u32(0x00488d93),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00a5222f),
        Color::from_u32(0x00396847),
        Color::from_u32(0x00ac5402),
        Color::from_u32(0x002848a9),
        Color::from_u32(0x006e33ce),
        Color::from_u32(0x00287980),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00FCF8F4), Color::from_u32(0x00FFFEFA)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x001D0B3A);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x007C2400);
}
