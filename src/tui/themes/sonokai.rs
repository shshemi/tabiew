use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Sonokai;

impl SixColorsTwoRowsStyler for Sonokai {
    const BACKGROUND: Color = Color::from_u32(0x002c2e34);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x004C4E54);
    const FOREGROUND: Color = Color::from_u32(0x00e2e2e3);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00181819);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00fc5d7c),
        Color::from_u32(0x009ed072),
        Color::from_u32(0x00e7c664),
        Color::from_u32(0x0076cce0),
        Color::from_u32(0x00b39df3),
        Color::from_u32(0x00f39660),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00fc5d7c),
        Color::from_u32(0x009ed072),
        Color::from_u32(0x00e7c664),
        Color::from_u32(0x0076cce0),
        Color::from_u32(0x00b39df3),
        Color::from_u32(0x00f39660),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x0032343A), Color::from_u32(0x00383A40)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00C2C2C3);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00CC2D4C);
}
