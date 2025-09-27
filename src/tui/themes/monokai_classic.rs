
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct MonokaiClassic;

impl SixColorsTwoRowsStyler for MonokaiClassic {
    const BACKGROUND: Color = Color::from_u32(0x00272822);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00474842);
    const FOREGROUND: Color = Color::from_u32(0x00fdfff1);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00272822);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00f92672),
        Color::from_u32(0x00a6e22e),
        Color::from_u32(0x00e6db74),
        Color::from_u32(0x00fd971f),
        Color::from_u32(0x00ae81ff),
        Color::from_u32(0x0066d9ef),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00f92672),
        Color::from_u32(0x00a6e22e),
        Color::from_u32(0x00e6db74),
        Color::from_u32(0x00fd971f),
        Color::from_u32(0x00ae81ff),
        Color::from_u32(0x0066d9ef),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x002D2E28), Color::from_u32(0x0033342E)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00A0A195);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00CD6700);

    fn id(&self) -> &str {
        "monokai_classic"
    }

    fn title(&self) -> &str {
        "MonokaiClassic"
    }
}
