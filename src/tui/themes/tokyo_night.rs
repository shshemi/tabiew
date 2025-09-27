use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct TokyoNight;

impl SixColorsTwoRowsStyler for TokyoNight {
    const BACKGROUND: Color = Color::from_u32(0x001f2335);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00292e42);
    const FOREGROUND: Color = Color::from_u32(0x00dfe3f5);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00494e62);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00c53b53),
        Color::from_u32(0x00ff757f),
        Color::from_u32(0x00ff9e64),
        Color::from_u32(0x007aa2f7),
        Color::from_u32(0x009d7cd8),
        Color::from_u32(0x0041a6b5),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00c53b53),
        Color::from_u32(0x00ff757f),
        Color::from_u32(0x00ff9e64),
        Color::from_u32(0x007aa2f7),
        Color::from_u32(0x009d7cd8),
        Color::from_u32(0x0041a6b5),
    ];
    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00292e42), Color::from_u32(0x0024283b)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00ffc777);
    const HIGHLIGHT_FOREGROUND: Color = Color::from_u32(0x001f2335);
    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00c53b53);

    fn id(&self) -> &str {
        "tokyo-night"
    }

    fn title(&self) -> &str {
        "TokyoNightSimple"
    }
}
