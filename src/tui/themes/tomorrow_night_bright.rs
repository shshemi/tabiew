
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct TomorrowNightBright;

impl SixColorsTwoRowsStyler for TomorrowNightBright {
    const BACKGROUND: Color = Color::from_u32(0x00000000);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00202020);
    const FOREGROUND: Color = Color::from_u32(0x00eaeaea);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00000000);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00d54e53),
        Color::from_u32(0x00b9ca4a),
        Color::from_u32(0x00e7c547),
        Color::from_u32(0x007aa6da),
        Color::from_u32(0x00c397d8),
        Color::from_u32(0x0070c0b1),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00d54e53),
        Color::from_u32(0x00b9ca4a),
        Color::from_u32(0x00e7c547),
        Color::from_u32(0x007aa6da),
        Color::from_u32(0x00c397d8),
        Color::from_u32(0x0070c0b1),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00060606), Color::from_u32(0x000C0C0C)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00CACACA);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00B79517);

    fn id(&self) -> &str {
        "tomorrow_night_bright"
    }

    fn title(&self) -> &str {
        "TomorrowNightBright"
    }
}
