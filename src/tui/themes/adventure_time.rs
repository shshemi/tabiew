
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct AdventureTime;

impl SixColorsTwoRowsStyler for AdventureTime {
    const BACKGROUND: Color = Color::from_u32(0x001f1d45);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x003F3D65);
    const FOREGROUND: Color = Color::from_u32(0x00f8dcc0);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00050404);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00fc5f5a),
        Color::from_u32(0x009eff6e),
        Color::from_u32(0x00efc11a),
        Color::from_u32(0x001997c6),
        Color::from_u32(0x009b5953),
        Color::from_u32(0x00c8faf4),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00bd0013),
        Color::from_u32(0x004ab118),
        Color::from_u32(0x00e7741e),
        Color::from_u32(0x000f4ac6),
        Color::from_u32(0x00665993),
        Color::from_u32(0x0070a598),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x0025234B), Color::from_u32(0x002B2951)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00CF9F18);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00B74400);
}
