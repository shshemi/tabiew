
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Atom;

impl SixColorsTwoRowsStyler for Atom {
    const BACKGROUND: Color = Color::from_u32(0x00161719);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00363739);
    const FOREGROUND: Color = Color::from_u32(0x00c5c8c6);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00000000);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00fd5ff1),
        Color::from_u32(0x0094fa36),
        Color::from_u32(0x00f5ffa8),
        Color::from_u32(0x0096cbfe),
        Color::from_u32(0x00b9b6fc),
        Color::from_u32(0x0085befd),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00fd5ff1),
        Color::from_u32(0x0087c38a),
        Color::from_u32(0x00ffd7b1),
        Color::from_u32(0x0085befd),
        Color::from_u32(0x00b9b6fc),
        Color::from_u32(0x0085befd),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x001C1D1F), Color::from_u32(0x00222325)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00B0B0B0);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00CFA781);

    fn id(&self) -> &str {
        "atom"
    }

    fn title(&self) -> &str {
        "Atom"
    }
}
