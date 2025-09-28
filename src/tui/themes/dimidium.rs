
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Dimidium;

impl SixColorsTwoRowsStyler for Dimidium {
    const BACKGROUND: Color = Color::from_u32(0x00141414);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00343434);
    const FOREGROUND: Color = Color::from_u32(0x00bab7b6);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00000000);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00ff643b),
        Color::from_u32(0x0037e57b),
        Color::from_u32(0x00fccd1a),
        Color::from_u32(0x00688dfd),
        Color::from_u32(0x00ed6fe9),
        Color::from_u32(0x0032e0fb),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00cf494c),
        Color::from_u32(0x0060b442),
        Color::from_u32(0x00db9c11),
        Color::from_u32(0x000575d8),
        Color::from_u32(0x00af5ed2),
        Color::from_u32(0x001db6bb),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x001A1A1A), Color::from_u32(0x00202020)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x0017C55B);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00AB6C00);
}
