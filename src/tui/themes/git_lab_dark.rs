
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct GitlabDark;

impl SixColorsTwoRowsStyler for GitlabDark {
    const BACKGROUND: Color = Color::from_u32(0x0028262b);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x0048464B);
    const FOREGROUND: Color = Color::from_u32(0x00ffffff);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00000000);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00fcb5aa),
        Color::from_u32(0x0091d4a8),
        Color::from_u32(0x00e9be74),
        Color::from_u32(0x00498dd1),
        Color::from_u32(0x00fcacc5),
        Color::from_u32(0x005edee3),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00f57f6c),
        Color::from_u32(0x0052b87a),
        Color::from_u32(0x00d99530),
        Color::from_u32(0x007fb6ed),
        Color::from_u32(0x00f88aaf),
        Color::from_u32(0x0032c5d2),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x002E2C31), Color::from_u32(0x00343237)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00DFDFDF);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00C85A7F);
}
