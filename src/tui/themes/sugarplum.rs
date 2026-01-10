use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Sugarplum;

impl SixColorsTwoRowsStyler for Sugarplum {
    const BACKGROUND: Color = Color::from_u32(0x00111147);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00313167);
    const FOREGROUND: Color = Color::from_u32(0x00db7ddd);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00111147);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x005cb5dc),
        Color::from_u32(0x0052deb5),
        Color::from_u32(0x0001f5c7),
        Color::from_u32(0x00fa5dfd),
        Color::from_u32(0x00c6a5fd),
        Color::from_u32(0x00ffffff),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x005ca8dc),
        Color::from_u32(0x0053b397),
        Color::from_u32(0x00249a84),
        Color::from_u32(0x00db7ddd),
        Color::from_u32(0x00d0beee),
        Color::from_u32(0x00f9f3f9),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x0017174D), Color::from_u32(0x001D1D53)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00339377);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00C9C3C9);
}
