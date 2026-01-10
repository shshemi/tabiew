use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Neopolitan;

impl SixColorsTwoRowsStyler for Neopolitan {
    const BACKGROUND: Color = Color::from_u32(0x00271f19);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00473F39);
    const FOREGROUND: Color = Color::from_u32(0x00ffffff);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00000000);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00800000),
        Color::from_u32(0x0061ce3c),
        Color::from_u32(0x00fbde2d),
        Color::from_u32(0x00253b76),
        Color::from_u32(0x00ff0080),
        Color::from_u32(0x008da6ce),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00800000),
        Color::from_u32(0x0061ce3c),
        Color::from_u32(0x00fbde2d),
        Color::from_u32(0x00253b76),
        Color::from_u32(0x00ff0080),
        Color::from_u32(0x008da6ce),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x002D251F), Color::from_u32(0x00332B25)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00DFDFDF);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00CF0050);
}
