
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Treehouse;

impl SixColorsTwoRowsStyler for Treehouse {
    const BACKGROUND: Color = Color::from_u32(0x00191919);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00393939);
    const FOREGROUND: Color = Color::from_u32(0x00786b53);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00321300);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00ed5d20),
        Color::from_u32(0x0055f238),
        Color::from_u32(0x00f2b732),
        Color::from_u32(0x0085cfed),
        Color::from_u32(0x00e14c5a),
        Color::from_u32(0x00f07d14),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00b2270e),
        Color::from_u32(0x0044a900),
        Color::from_u32(0x00aa820c),
        Color::from_u32(0x0058859a),
        Color::from_u32(0x0097363d),
        Color::from_u32(0x00b25a1e),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x001F1F1F), Color::from_u32(0x00252525)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00DAA800);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00820000);
}
