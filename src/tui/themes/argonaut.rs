
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Argonaut;

impl SixColorsTwoRowsStyler for Argonaut {
    const BACKGROUND: Color = Color::from_u32(0x000e1019);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x002E3039);
    const FOREGROUND: Color = Color::from_u32(0x00fffaf4);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00232323);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00ff2740),
        Color::from_u32(0x00abe15b),
        Color::from_u32(0x00ffd242),
        Color::from_u32(0x000092ff),
        Color::from_u32(0x009a5feb),
        Color::from_u32(0x0067fff0),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00ff000f),
        Color::from_u32(0x008ce10b),
        Color::from_u32(0x00ffb900),
        Color::from_u32(0x00008df8),
        Color::from_u32(0x006d43a6),
        Color::from_u32(0x0000d8eb),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x0014161F), Color::from_u32(0x001A1C25)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00DF0000);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00CF8900);
}
