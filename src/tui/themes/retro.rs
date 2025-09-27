
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Retro;

impl SixColorsTwoRowsStyler for Retro {
    const BACKGROUND: Color = Color::from_u32(0x00000000);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00202020);
    const FOREGROUND: Color = Color::from_u32(0x0013a10e);
    const DARK_FOREGROUND: Color = Color::from_u32(0x0013a10e);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x0016ba10),
        Color::from_u32(0x0016ba10),
        Color::from_u32(0x0016ba10),
        Color::from_u32(0x0016ba10),
        Color::from_u32(0x0016ba10),
        Color::from_u32(0x0016ba10),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x0013a10e),
        Color::from_u32(0x0013a10e),
        Color::from_u32(0x0013a10e),
        Color::from_u32(0x0013a10e),
        Color::from_u32(0x0013a10e),
        Color::from_u32(0x0013a10e),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00060606), Color::from_u32(0x000C0C0C)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00008100);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00007100);

    fn id(&self) -> &str {
        "retro"
    }

    fn title(&self) -> &str {
        "Retro"
    }
}
