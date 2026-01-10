use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Guezwhoz;

impl SixColorsTwoRowsStyler for Guezwhoz {
    const BACKGROUND: Color = Color::from_u32(0x001d1d1d);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x003D3D3D);
    const FOREGROUND: Color = Color::from_u32(0x00d9d9d9);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00333333);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00e85181),
        Color::from_u32(0x00afd7af),
        Color::from_u32(0x00d1ed85),
        Color::from_u32(0x0064b2ed),
        Color::from_u32(0x00a398ed),
        Color::from_u32(0x0061ede4),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00e85181),
        Color::from_u32(0x007ad694),
        Color::from_u32(0x00b7d074),
        Color::from_u32(0x005aa0d6),
        Color::from_u32(0x009a90e0),
        Color::from_u32(0x0058d6ce),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00232323), Color::from_u32(0x00292929)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x0079B491);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00B82151);
}
