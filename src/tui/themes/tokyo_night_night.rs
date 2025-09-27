use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct TokyoNightNight;

impl SixColorsTwoRowsStyler for TokyoNightNight {
    const BACKGROUND: Color = Color::from_u32(0x001a1b26);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x003A3B46);
    const FOREGROUND: Color = Color::from_u32(0x00c0caf5);
    const DARK_FOREGROUND: Color = Color::from_u32(0x0015161e);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00f7768e),
        Color::from_u32(0x009ece6a),
        Color::from_u32(0x00e0af68),
        Color::from_u32(0x007aa2f7),
        Color::from_u32(0x00bb9af7),
        Color::from_u32(0x007dcfff),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00f7768e),
        Color::from_u32(0x009ece6a),
        Color::from_u32(0x00e0af68),
        Color::from_u32(0x007aa2f7),
        Color::from_u32(0x00bb9af7),
        Color::from_u32(0x007dcfff),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x0020212C), Color::from_u32(0x00262732)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00A0AAD5);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00C7465E);

    fn id(&self) -> &str {
        "tokyo_night_night"
    }

    fn title(&self) -> &str {
        "TokyoNightNight"
    }
}
