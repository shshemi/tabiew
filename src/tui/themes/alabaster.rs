
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Alabaster;

impl SixColorsTwoRowsStyler for Alabaster {
    const BACKGROUND: Color = Color::from_u32(0x00f7f7f7);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00FFFFFF);
    const FOREGROUND: Color = Color::from_u32(0x00000000);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00000000);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00f05050),
        Color::from_u32(0x0060cb00),
        Color::from_u32(0x00ffbc5d),
        Color::from_u32(0x00007acc),
        Color::from_u32(0x00e64ce6),
        Color::from_u32(0x0000aacb),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00aa3731),
        Color::from_u32(0x00448c27),
        Color::from_u32(0x00cb9000),
        Color::from_u32(0x00325cc0),
        Color::from_u32(0x007a3e9d),
        Color::from_u32(0x000083b2),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00FDFDFD), Color::from_u32(0x00FFFFFF)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00005AAC);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x009B6000);

    fn id(&self) -> &str {
        "alabaster"
    }

    fn title(&self) -> &str {
        "Alabaster"
    }
}
