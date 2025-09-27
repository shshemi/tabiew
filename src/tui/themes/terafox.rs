
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Terafox;

impl SixColorsTwoRowsStyler for Terafox {
    const BACKGROUND: Color = Color::from_u32(0x00152528);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00354548);
    const FOREGROUND: Color = Color::from_u32(0x00e6eaea);
    const DARK_FOREGROUND: Color = Color::from_u32(0x002f3239);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00eb746b),
        Color::from_u32(0x008eb2af),
        Color::from_u32(0x00fdb292),
        Color::from_u32(0x0073a3b7),
        Color::from_u32(0x00b97490),
        Color::from_u32(0x00afd4de),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00e85c51),
        Color::from_u32(0x007aa4a1),
        Color::from_u32(0x00fda47f),
        Color::from_u32(0x005a93aa),
        Color::from_u32(0x00ad5c7c),
        Color::from_u32(0x00a1cdd8),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x001B2B2E), Color::from_u32(0x00213134)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00C6CACA);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00CD744F);

    fn id(&self) -> &str {
        "terafox"
    }

    fn title(&self) -> &str {
        "Terafox"
    }
}
