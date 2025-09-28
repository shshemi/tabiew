
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Royal;

impl SixColorsTwoRowsStyler for Royal {
    const BACKGROUND: Color = Color::from_u32(0x00100815);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00302835);
    const FOREGROUND: Color = Color::from_u32(0x00514968);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00241f2b);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00d5356c),
        Color::from_u32(0x002cd946),
        Color::from_u32(0x00fde83b),
        Color::from_u32(0x0090baf9),
        Color::from_u32(0x00a479e3),
        Color::from_u32(0x00acd4eb),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x0091284c),
        Color::from_u32(0x0023801c),
        Color::from_u32(0x00b49d27),
        Color::from_u32(0x006580b0),
        Color::from_u32(0x00674d96),
        Color::from_u32(0x008aaabe),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00160E1B), Color::from_u32(0x001C1421)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00322946);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00846D00);
}
