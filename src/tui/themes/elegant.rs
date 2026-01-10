use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Elegant;

impl SixColorsTwoRowsStyler for Elegant {
    const BACKGROUND: Color = Color::from_u32(0x00292b31);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00494B51);
    const FOREGROUND: Color = Color::from_u32(0x00ced2d6);
    const DARK_FOREGROUND: Color = Color::from_u32(0x000a1222);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00ff0057),
        Color::from_u32(0x0085cc95),
        Color::from_u32(0x00ffcb8b),
        Color::from_u32(0x008dabe1),
        Color::from_u32(0x00c792eb),
        Color::from_u32(0x003facef),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00ff0257),
        Color::from_u32(0x0085cc95),
        Color::from_u32(0x00ffcb8b),
        Color::from_u32(0x008dabe1),
        Color::from_u32(0x00c792eb),
        Color::from_u32(0x0078ccf0),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x002F3137), Color::from_u32(0x0035373D)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00009EDF);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00CF0027);
}
